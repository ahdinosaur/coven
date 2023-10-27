#![allow(non_snake_case)]

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use coven::service::{start_service, Client, Ping};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
// use dioxus_signals;

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

fn use_client(cx: Scope) -> Rc<RefCell<Client>> {
    use_context::<Rc<RefCell<Client>>>(cx).unwrap().clone()
}

fn App(cx: Scope) -> Element {
    use_context_provider(cx, || start_service().unwrap());

    render! {
        Router::<Route> {}
    }
}

#[component]
fn AppHome(cx: Scope) -> Element {
    let client = use_client(cx);

    use_future!(cx, || async move {
        let res = client.borrow_mut().rpc(Ping).await.unwrap();
        println!("pong: {:?}", res);
    });

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
