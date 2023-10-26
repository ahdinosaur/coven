#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

// TODO provide service as root context
// - https://github.com/DioxusLabs/dioxus/pull/1193/files

fn main() {
    dioxus_desktop::launch(App)
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    AppHome {},
    #[nest("/cabal/:cabal_id")]
        #[route("/")]
        CabalHome {
            cabal_id: String,
        },
        #[route("/channel/:channel_id")]
        CabalChannel {
            cabal_id: String,
            channel_id: String
        },
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[inline_props]
fn AppHome(cx: Scope) -> Element {
    render! {
        div {
            "Home"
        }
    }
}

#[inline_props]
fn CabalHome(cx: Scope, cabal_id: String) -> Element {
    render! {
        div {
            "Cabal Home"
        }
    }
}

#[inline_props]
fn CabalChannel(cx: Scope, cabal_id: String, channel_id: String) -> Element {
    render! {
        div {
            "Cabal Channel"
        }
    }
}
