use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_read;

use crate::{
    components::ChannelsNav,
    pages::Route,
    state::{ACTIVE_CABAL_CHANNEL_ID, ACTIVE_CABAL_ID, CABAL_CHANNEL_IDS},
};

#[inline_props]
pub(crate) fn CabalLayout(cx: Scope) -> Element {
    let active_cabal_id = use_read(cx, &ACTIVE_CABAL_ID);
    let channel_ids = use_read(cx, &CABAL_CHANNEL_IDS);
    let active_channel_id = use_read(cx, &ACTIVE_CABAL_CHANNEL_ID);

    let active_channel_label = active_channel_id.clone().unwrap_or("".into());

    render! {
        div {
            h2 {
                "{active_channel_label}"
            }
            Fragment {
                if active_cabal_id.is_some() && channel_ids.is_some() {
                    rsx!(nav {
                        ChannelsNav {
                            cabal_id: active_cabal_id.clone().unwrap(),
                            channel_ids: channel_ids.clone().unwrap(),
                        }
                    })
                }
            }
            Outlet::<Route> {}
        }
    }
}
