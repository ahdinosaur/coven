use fermi::Atom;

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
