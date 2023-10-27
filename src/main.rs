#![allow(non_snake_case)]

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use coven::{
    service::{start_service, Client, Ping},
    state::use_service_provider,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

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
    use_service_provider(cx);

    render! {
        Router::<Route> {}
    }
}

#[component]
fn AppHome(cx: Scope) -> Element {
    render! {
        div {
            "Home"
        }
    }
}

#[component]
fn CabalHome(cx: Scope, cabal_id: String) -> Element {
    render! {
        div {
            "Cabal Home"
        }
    }
}

#[component]
fn CabalChannel(cx: Scope, cabal_id: String, channel_id: String) -> Element {
    render! {
        div {
            "Cabal Channel"
        }
    }
}
