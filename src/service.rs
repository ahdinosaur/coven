use async_std::net::{TcpListener, TcpStream};
use cable_core::{CableManager, Store};
use dioxus::prelude::UnboundedReceiver;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use std::{collections::HashMap, rc::Rc};

// type CableAddr = Vec<u8>;
type TcpAddr = String;

pub enum Command {
    Connect {
        // cable_addr: CableAddr,
        tcp_addr: TcpAddr,
    },
    Listen {
        // cable_addr: CableAddr,
        tcp_addr: TcpAddr,
    },
}

pub async fn create_service<S: Store + Default>(
    mut rx: UnboundedReceiver<Command>,
    _atoms: Rc<AtomRoot>,
) {
    let mut service = Service::<S>::default();

    while let Some(cmd) = rx.next().await {
        match cmd {
            Command::Connect {
                // cable_addr,
                tcp_addr,
            } => service.handle_connect(/*cable_addr, */ tcp_addr).await,
            Command::Listen {
                // cable_addr,
                tcp_addr,
            } => service.handle_listen(/*cable_addr, */ tcp_addr).await,
        }
    }
}

#[derive(Clone)]
struct Service<S: Store + Default> {
    // cables: HashMap<CableAddr, CableManager<S>>,
    cable: CableManager<S>,
}

impl<S: Store + Default> Default for Service<S> {
    fn default() -> Self {
        Self {
            // cables: HashMap::new(),
            cable: CableManager::new(S::default()),
        }
    }
}

impl<S: Store + Default> Service<S> {
    async fn handle_connect(&self, /*cable_addr: CableAddr, */ tcp_addr: TcpAddr) {
        let cable = self.get_cable(/*&cable_addr*/).unwrap().clone();
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

        // TODO: store a reference so we can un-connect later
    }

    async fn handle_listen(&mut self, /*cable_addr: CableAddr, */ mut tcp_addr: TcpAddr) {
        let cable = self.get_cable(/*&cable_addr*/).unwrap();

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
                        println!("Cable stream listener error: {}", err);
                    }
                });
            }
        }

        // TODO: store a reference so we can un-listen later
    }

    fn get_cable(&self /*, cable_addr: &CableAddr*/) -> Option<&CableManager<S>> {
        // self.cables.get(cable_addr)
        Some(&self.cable)
    }

    /*
    fn add_cable(&mut self, cable_addr: CableAddr) {
        self.cables
            .insert(cable_addr.to_vec(), CableManager::new(S::default()));
    }
    */
}
