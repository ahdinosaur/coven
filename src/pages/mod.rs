use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_init_atom_root;

use crate::hooks::{use_service, use_service_provider};

mod app_home;
mod app_layout;
mod cabal_channel;
mod cabal_home;
mod cabal_layout;

use app_home::AppHomePage;
use app_layout::AppLayout;
use cabal_channel::CabalChannelPage;
use cabal_home::CabalHomePage;
use cabal_layout::CabalLayout;

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

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub(crate) enum Route {
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
