use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_read;

use crate::{
    components::{CabalChannelList, CabalNavList},
    pages::Route,
    state::{ACTIVE_CABAL_CHANNEL_ID, ACTIVE_CABAL_ID, CABAL_CHANNEL_IDS, CABAL_IDS},
};

#[inline_props]
pub(crate) fn CabalLayout(cx: Scope) -> Element {
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
