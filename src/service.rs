use derive_more::{From, TryInto};
use quic_rpc::{message::RpcMsg, transport::flume, RpcClient, RpcServer, Service};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pong;

#[derive(Debug, Clone)]
pub struct CovenService;

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum PingRequest {
    Ping(Ping),
}

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum PingResponse {
    Pong(Pong),
}

impl Service for CovenService {
    type Req = PingRequest;
    type Res = PingResponse;
}

impl RpcMsg<CovenService> for Ping {
    type Response = Pong;
}

pub type Client = RpcClient<CovenService, flume::FlumeConnection<PingResponse, PingRequest>>;

pub fn start_service() -> Result<Client, anyhow::Error> {
    let (server, client) = flume::connection::<PingRequest, PingResponse>(1);

    let client = RpcClient::<CovenService, _>::new(client);
    let server = RpcServer::<CovenService, _>::new(server);

    tokio::task::spawn(async move {
        let handler = Handler;
        loop {
            let (msg, chan) = server.accept().await.unwrap();
            match msg {
                PingRequest::Ping(ping) => chan.rpc(ping, handler, Handler::ping).await.unwrap(),
            }
        }
    });

    Ok(client)
}

#[derive(Debug, Clone, Copy)]
struct Handler;

impl Handler {
    async fn ping(self, _req: Ping) -> Pong {
        Pong
    }
}
