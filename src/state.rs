use dioxus::prelude::{use_context, use_context_provider, Scope};
use fermi::{use_set, Atom};

use crate::service::{start_service, Client};

pub fn use_client(cx: Scope) -> Client {
    use_context::<Client>(cx).unwrap().clone()
}

pub fn use_service_provider(cx: Scope) {
    use_context_provider(cx, || start_service().unwrap());
}

pub type CabalId = String;
pub type ChannelId = String;

pub type PeerId = String;
pub type PeerNick = String;
pub struct Peer {
    id: PeerId,
    nick: PeerNick,
}

pub struct Message {
    text: String,
}

static MY_PEER: Atom<Option<Peer>> = Atom(|_| None);
static CABAL_IDS: Atom<Option<Vec<CabalId>>> = Atom(|_| None);
static ACTIVE_CABAL_ID: Atom<Option<CabalId>> = Atom(|_| None);
static UNREAD_CABAL_IDS: Atom<Vec<CabalId>> = Atom(|_| vec![]);
static CABAL_PEERS: Atom<Option<Vec<Peer>>> = Atom(|_| None);
static CABAL_CHANNEL_IDS: Atom<Option<Vec<ChannelId>>> = Atom(|_| None);
static ACTIVE_CABAL_CHANNEL: Atom<Option<ChannelId>> = Atom(|_| None);
static CABAL_CHANNEL_MESSAGES: Atom<Option<Vec<Message>>> = Atom(|_| None);
static CABAL_CHANNEL_PEERS: Atom<Option<Vec<Peer>>> = Atom(|_| None);

pub fn load_my_peer(cx: Scope) {
    let set_my_peer = use_set(cx, &MY_PEER);
    let client = use_client(cx);

    /*
    use_future!(cx, || async move {
        let res = client.rpc(Ping).await.unwrap();
        println!("pong: {:?}", res);
    });
    */
}
