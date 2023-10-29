use cable_core::{Keypair, PublicKey};
use fermi::Atom;

pub type CabalId = String;
pub type ChannelId = String;

pub type PeerNick = String;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peer {
    pub key: PublicKey,
    pub name: PeerNick,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyPeer {
    pub keypair: Keypair,
    pub name: PeerNick,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub text: String,
}

pub static MY_PEER: Atom<Option<Peer>> = Atom(|_| None);
pub static CABAL_IDS: Atom<Vec<CabalId>> =
    Atom(|_| vec!["APPLE".into(), "PEAR".into(), "BANANA".into()]);
pub static ACTIVE_CABAL_ID: Atom<Option<CabalId>> = Atom(|_| None);
pub static UNREAD_CABAL_IDS: Atom<Vec<CabalId>> = Atom(|_| vec![]);
pub static CABAL_PEERS: Atom<Option<Vec<Peer>>> = Atom(|_| None);
pub static CABAL_CHANNEL_IDS: Atom<Option<Vec<ChannelId>>> = Atom(|_| Some(vec!["default".into()]));
pub static ACTIVE_CABAL_CHANNEL_ID: Atom<Option<ChannelId>> = Atom(|_| None);
pub static CABAL_CHANNEL_POSTS: Atom<Option<Vec<Post>>> = Atom(|_| None);
pub static CABAL_CHANNEL_PEERS: Atom<Option<Vec<Peer>>> = Atom(|_| None);
