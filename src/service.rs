use async_std::net::{TcpListener, TcpStream};
use cable::{post::PostBody, ChannelOptions};
use cable_core::{CableManager, Store};
use dioxus::prelude::UnboundedReceiver;
use fermi::{AtomRoot, Readable};
use futures_util::stream::StreamExt;
use std::{collections::HashMap, rc::Rc};
use tokio::task::AbortHandle;

use crate::{
    state::{Post, CABAL_CHANNEL_POSTS},
    time,
};

// type CableAddr = Vec<u8>;
type TcpAddr = String;
type ChannelId = String;

pub enum Command {
    Connect {
        // cable_addr: CableAddr,
        tcp_addr: TcpAddr,
    },
    Listen {
        // cable_addr: CableAddr,
        tcp_addr: TcpAddr,
    },
    OpenChannel {
        // cable_addr: CableAddr,
        channel_id: ChannelId,
    },
}

pub async fn create_service<S: Store + Default>(
    mut rx: UnboundedReceiver<Command>,
    atoms: Rc<AtomRoot>,
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
            Command::OpenChannel { channel_id } => {
                service
                    .handle_open_channel(atoms.clone(), /*cable_addr, */ channel_id)
                    .await
            }
        }
    }
}

struct Service<S: Store + Default> {
    // cables: HashMap<CableAddr, CableManager<S>>,
    cable: CableManager<S>,
    channel_aborts: HashMap<ChannelId, AbortHandle>,
}

impl<S: Store + Default> Default for Service<S> {
    fn default() -> Self {
        Self {
            // cables: HashMap::new(),
            cable: CableManager::new(S::default()),
            channel_aborts: HashMap::new(),
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
        if !tcp_addr.contains(':') {
            tcp_addr = format!("0.0.0.0:{}", tcp_addr);
        }

        let cable = self.get_cable(/*&cable_addr*/).unwrap().clone();

        tokio::spawn(async move {
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
        });

        // TODO: store a reference so we can un-listen later
    }

    async fn handle_open_channel(
        &mut self,
        atoms: Rc<AtomRoot>,
        /*cable_addr: CableAddr, */ channel_id: ChannelId,
    ) {
        let opts = ChannelOptions {
            channel: channel_id.clone(),
            time_start: time::two_weeks_ago().unwrap(),
            time_end: 0,
            limit: 4096,
        };

        let mut cable = self.get_cable().unwrap().clone();
        let subscription = tokio::task::spawn_local(async move {
            println!("two");
            let mut posts: Vec<Post> = Vec::new();
            atoms.set((&CABAL_CHANNEL_POSTS).unique_id(), Some(posts.clone()));

            let mut post_stream = cable.open_channel(&opts).await.unwrap();
            while let Some(Ok(post)) = post_stream.next().await {
                println!("post: {}", post);
                if let PostBody::Text { channel: _, text } = post.body {
                    posts.push(Post { text });
                    atoms.set((&CABAL_CHANNEL_POSTS).unique_id(), Some(posts.clone()));
                }
            }
        });

        self.channel_aborts
            .insert(channel_id, subscription.abort_handle());
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
