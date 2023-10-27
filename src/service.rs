// TODO refactor like https://dioxuslabs.com/docs/nightly/guide/en/async/use_coroutine.html#sending-values

use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};
use std::collections::HashMap;

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

    pub async fn connect(&self, cable_addr: &CableAddr, tcp_addr: &TcpAddr) {
        let cable = self.get_cable(&cable_addr).unwrap().clone();
        let tcp_addr = tcp_addr.clone();
        tokio::spawn(async move {
            let stream = match TcpStream::connect(&tcp_addr).await {
                Ok(stream) => stream,
                Err(err) => {
                    println!("Unable to connect to: {}, Error: {}", &tcp_addr, err);
                    return;
                }
            };
            if let Err(err) = cable.listen(stream).await {
                println!("Error listening to stream: {}", err)
            }
        });
    }

    pub async fn listen(&mut self, cable_addr: &CableAddr, tcp_addr: &TcpAddr) {
        let cable = self.get_cable(cable_addr).unwrap();

        let tcp_addr = if !tcp_addr.contains(':') {
            format!("0.0.0.0:{}", tcp_addr)
        } else {
            tcp_addr.clone()
        };

        let listener = TcpListener::bind(tcp_addr.clone()).await.unwrap();

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            if let Ok(stream) = stream {
                let cable = cable.clone();
                tokio::spawn(async move {
                    if let Err(err) = cable.listen(stream).await {
                        println!("Cable stream listener error: {}", err);
                    }
                });
            }
        }
    }
}
