#![allow(non_snake_case)]

pub mod service;
pub mod state;
pub mod time;

use cable_core::MemoryStore;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_atom_root, use_init_atom_root, use_read, use_set};

use crate::service::Service;
use crate::state::Post;
use crate::state::{
    ACTIVE_CABAL_CHANNEL_ID, ACTIVE_CABAL_ID, CABAL_CHANNEL_IDS, CABAL_CHANNEL_POSTS, CABAL_IDS,
};

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        AppHomePage {},

        #[layout(CabalLayout)]
            #[nest("/cabal/:cabal_id")]
                #[route("/")]
                CabalHomePage {
                    cabal_id: String,
                },
                #[route("/channel/:channel_id")]
                CabalChannelPage {
                    cabal_id: String,
                    channel_id: String
                },
}

type Store = MemoryStore;

fn use_service(cx: &ScopeState) -> Service<Store> {
    use_context::<Service<Store>>(cx).unwrap().clone()
}

fn use_service_provider(cx: &ScopeState) {
    use_context_provider(cx, || Service::<Store>::default());
}

pub fn App(cx: Scope) -> Element {
    use_service_provider(cx);
    use_init_atom_root(cx);

    let mut service = use_service(cx);
    use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
        service.listen("1337".into()).await
    });

    render! {
        Router::<Route> {}
    }
}

#[inline_props]
fn AppHomePage(cx: Scope) -> Element {
    let cabal_ids = use_read(cx, &CABAL_IDS);

    render! {
        div {
            h1 {
                "Welcome to Cabal"
            },
            section {
                h2 {
                    "Cabals:"
                }
                CabalCardList { cabal_ids: cabal_ids.clone() }
            }
        }
    }
}

#[inline_props]
fn AppLayout(cx: Scope) -> Element {
    render! {
        Outlet::<Route> {}
    }
}

#[inline_props]
fn CabalCardList(cx: Scope, cabal_ids: Vec<String>) -> Element {
    render! {
        ul {
            for cabal_id in cabal_ids {
                li {
                    Link {
                        to: Route::CabalHomePage { cabal_id: cabal_id.into() },
                        "{cabal_id}",
                    }
                }
            }
        }
    }
}

#[inline_props]
fn CabalLayout(cx: Scope) -> Element {
    let cabal_ids = use_read(cx, &CABAL_IDS);
    let active_cabal_id = use_read(cx, &ACTIVE_CABAL_ID);
    let channel_ids = use_read(cx, &CABAL_CHANNEL_IDS);
    let _active_channel_id = use_read(cx, &ACTIVE_CABAL_CHANNEL_ID);

    render! {
        nav {
            Link {
                to: Route::AppHomePage {},
                "Home"
            },
            div {
                p { "Cabals:" }
                CabalNavList { cabal_ids: cabal_ids.clone() }
            }
        }
        if active_cabal_id.is_some() && channel_ids.is_some() {
            rsx!(div {
                p { "Channels:" }
                CabalChannelList {
                    cabal_id: active_cabal_id.clone().unwrap(),
                    channel_ids: channel_ids.clone().unwrap(),
                }
            })
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn CabalNavList(cx: Scope, cabal_ids: Vec<String>) -> Element {
    render! {
        ul {
            for cabal_id in cabal_ids {
                li {
                    key: "{cabal_id}",
                    Link {
                        to: Route::CabalHomePage { cabal_id: cabal_id.into() },
                        "{cabal_id}",
                    }
                }
            }
        }
    }
}

#[inline_props]
fn CabalChannelList(cx: Scope, cabal_id: String, channel_ids: Vec<String>) -> Element {
    render! {
        ul {
            for channel_id in channel_ids {
                li {
                    key: "{channel_id}",
                    Link {
                        to: Route::CabalChannelPage {
                            cabal_id: cabal_id.into(),
                            channel_id: channel_id.into()
                        },
                        "{channel_id}",
                    }
                }
            }
        }
    }
}

#[inline_props]
fn CabalHomePage(cx: Scope, cabal_id: String) -> Element {
    use_active_cabal_id(cx, cabal_id.into());

    render! {
        div {
            h1 {
                "Cabal: {cabal_id}"
            }
        }
    }
}

#[inline_props]
fn CabalChannelPage(cx: Scope, cabal_id: String, channel_id: String) -> Element {
    use_active_cabal_id(cx, cabal_id.into());
    use_active_channel_id(cx, channel_id.into());

    let service = use_service(cx);
    let atoms = use_atom_root(cx);
    use_coroutine(cx, |_rx: UnboundedReceiver<()>| {
        to_owned![service, atoms, channel_id];
        async move { service.open_channel(atoms, channel_id).await }
    });

    let posts = use_read(cx, &CABAL_CHANNEL_POSTS);

    render! {
        div {
            h1 {
                "Channel: {channel_id}"
            },

            if let Some(posts) = posts {
                rsx!(ChannelPosts {
                    posts: posts.clone()
                })
            }
        }
    }
}

#[inline_props]
fn ChannelPosts(cx: Scope, posts: Vec<Post>) -> Element {
    render! {
        ul {
            for post in posts {
                li {
                    "{post.text}"
                }
            }
        }
    }
}

fn use_active_cabal_id(cx: &ScopeState, cabal_id: String) {
    let set_active_cabal_id = use_set(cx, &ACTIVE_CABAL_ID);
    use_effect(cx, (), |_| {
        to_owned![set_active_cabal_id];
        async move {
            set_active_cabal_id(Some(cabal_id));
        }
    });
    use_on_unmount(cx, {
        to_owned![set_active_cabal_id];
        move || {
            set_active_cabal_id(None);
        }
    });
}

fn use_active_channel_id(cx: &ScopeState, channel_id: String) {
    let set_active_channel_id = use_set(cx, &ACTIVE_CABAL_CHANNEL_ID);
    use_effect(cx, (), |_| {
        to_owned![set_active_channel_id];
        async move {
            set_active_channel_id(Some(channel_id));
        }
    });
    use_on_unmount(cx, {
        to_owned![set_active_channel_id];
        move || {
            set_active_channel_id(None);
        }
    });
}
