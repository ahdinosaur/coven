// Define your messages
#[derive(Debug, Serialize, Deserialize)]
struct Ping;

#[derive(Debug, Serialize, Deserialize)]
struct Pong;

// Define your RPC service and its request/response types
#[derive(Debug, Clone)]
struct PingService;

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
enum PingRequest {
    Ping(Ping),
}

#[derive(Debug, Serialize, Deserialize, From, TryInto)]
enum PingResponse {
    Pong(Pong),
}

impl Service for PingService {
  type Req = PingRequest;
  type Res = PingResponse;
}

// Define interaction patterns for each request type
impl RpcMsg<PingService> for Ping {
  type Response = Pong;
}
