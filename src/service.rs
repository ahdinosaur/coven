use std::collections::{HashMap, HashSet};

use cable_core::{CableManager, MemoryStore, Store};
use derive_more::{From, TryInto};
use quic_rpc::{message::RpcMsg, transport::flume, RpcClient, RpcServer, Service};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pong;

#[derive(Debug, Clone)]
pub struct CovenService;

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum CovenRequest {
    Ping(Ping),
}

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum CovenResponse {
    Pong(Pong),
}

impl Service for CovenService {
    type Req = CovenRequest;
    type Res = CovenResponse;
}

impl RpcMsg<CovenService> for Ping {
    type Response = Pong;
}

pub type Client = RpcClient<CovenService, flume::FlumeConnection<CovenResponse, CovenRequest>>;

pub fn start_service() -> Result<Client, anyhow::Error> {
    let (server, client) = flume::connection::<CovenRequest, CovenResponse>(1);

    let client = RpcClient::<CovenService, _>::new(client);
    let server = RpcServer::<CovenService, _>::new(server);

    tokio::task::spawn(async move {
        let mut handler = Handler::new();
        loop {
            let (msg, chan) = server.accept().await.unwrap();
            match msg {
                CovenRequest::Ping(ping) => chan.rpc(ping, handler, Handler::ping).await.unwrap(),
            }
        }
    });

    Ok(client)
}

type PeerAddress = Vec<u8>;

/// A TCP connection and associated address (host:post).
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Connection {
    Connected(String),
    Listening(String),
}

#[derive(Clone)]
struct Handler {
    cables: HashMap<PeerAddress, CableManager<MemoryStore>>,
    connections: HashSet<Connection>,
}

impl Handler {
    fn new() -> Self {
        Self {
            cables: HashMap::new(),
            connections: HashSet::new(),
        }
    }

    async fn ping(self, _req: Ping) -> Pong {
        Pong
    }
}
