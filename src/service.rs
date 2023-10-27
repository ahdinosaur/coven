// TODO refactor like https://dioxuslabs.com/docs/nightly/guide/en/async/use_coroutine.html#sending-values

use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};
use std::collections::HashMap;

use anyhow::anyhow;
use cable_core::{CableManager, Store};

type CableAddr = Vec<u8>;
type TcpAddr = String;

#[derive(Clone)]
struct Service<S: Store + Default> {
    cables: HashMap<CableAddr, CableManager<S>>,
}

impl<S: Store + Default> Default for Service<S> {
    fn default() -> Self {
        Self {
            cables: HashMap::new(),
        }
    }
}

impl<S: Store + Default> Service<S> {
    pub fn get_cable(&self, cable_addr: &CableAddr) -> Option<&CableManager<S>> {
        self.cables.get(cable_addr)
    }

    pub fn get_cable_mut(&self, cable_addr: &CableAddr) -> Option<&mut CableManager<S>> {
        self.cables.get_mut(cable_addr)
    }

    pub fn add_cable(&mut self, cable_addr: &CableAddr) {
        self.cables
            .insert(cable_addr.to_vec(), CableManager::new(S::default()));
    }

    pub async fn connect(
        &mut self,
        cable_addr: &CableAddr,
        tcp_addr: String,
    ) -> anyhow::Result<()> {
        let stream = TcpStream::connect(tcp_addr).await?;
        let cable = self.get_cable(cable_addr).unwrap();
        cable
            .listen(stream)
            .await
            .map_err(|err| anyhow!("{}", err))?;
        Ok(())
    }

    pub async fn listen(&mut self, cable_addr: &CableAddr, tcp_addr: String) {
        // Format the TCP address if a host was not supplied.
        if !tcp_addr.contains(':') {
            tcp_addr = format!("0.0.0.0:{}", tcp_addr);
        }

        let listener = TcpListener::bind(tcp_addr.clone()).await.unwrap();

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            if let Ok(stream) = stream {
                let cable = cable.clone();
                tokio::spawn(async move {
                    if let Err(err) = cable.listen(stream).await {
                        error!("Cable stream listener error: {}", err);
                    }
                });
            }
        }
    }
}
