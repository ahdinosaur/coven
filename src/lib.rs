#![allow(non_snake_case)]

pub mod service;
pub mod state;

use cable_core::MemoryStore;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_init_atom_root, use_read};

use crate::service::{create_service, Command};
use crate::state::CABAL_IDS;

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

pub fn App(cx: Scope) -> Element {
    let atoms = use_init_atom_root(cx);
    let service = use_coroutine(cx, |rx| create_service::<MemoryStore>(rx, atoms.clone()));

    service.send(Command::Listen {
        tcp_addr: "1337".into(),
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

    render! {
        nav {
            Link {
                to: Route::AppHomePage { },
                "Home"
            },
            CabalNavList { cabal_ids: cabal_ids.clone() }
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
fn CabalHomePage(cx: Scope, cabal_id: String) -> Element {
    render! {
        div {
            h1 {
                "{cabal_id}"
            }
        }
    }
}

#[inline_props]
fn CabalChannelPage(cx: Scope, cabal_id: String, channel_id: String) -> Element {
    render! {
        div {
            "Cabal Channel"
        }
    }
}
