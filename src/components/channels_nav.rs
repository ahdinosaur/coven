use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_read;

use crate::{
    pages::Route,
    state::{ACTIVE_CABAL_CHANNEL_ID, ACTIVE_CABAL_ID, CABAL_CHANNEL_IDS},
};

#[inline_props]
pub(crate) fn ChannelsNav(cx: Scope) -> Element {
    let cabal_id = use_read(cx, &ACTIVE_CABAL_ID);
    let channel_ids = use_read(cx, &CABAL_CHANNEL_IDS);
    let active_channel_id = use_read(cx, &ACTIVE_CABAL_CHANNEL_ID);

    render! {
        nav {
            if cabal_id.is_some() && channel_ids.is_some() {
                rsx!(ChannelList {
                    cabal_id: cabal_id.clone().unwrap(),
                    channel_ids: channel_ids.clone().unwrap(),
                    active_channel_id: active_channel_id.clone(),
                })
            }
        }
    }
}

#[inline_props]
fn ChannelList(
    cx: Scope,
    cabal_id: String,
    channel_ids: Vec<String>,
    #[props(!optional)] active_channel_id: Option<String>,
) -> Element {
    render! {
        ul {
            class: class!(w_full join join_vertical),
            aria_label: "Channels",
            for channel_id in channel_ids.iter() {
                ChannelListItem {
                    cabal_id: cabal_id.clone(),
                    channel_id: channel_id.clone(),
                    is_active: Some(channel_id) == active_channel_id.as_ref(),
                }
            }
        }
    }
}

#[inline_props]
fn ChannelListItem(cx: Scope, cabal_id: String, channel_id: String, is_active: bool) -> Element {
    let item_class = class!(w_full btn btn_outline join_item);
    let active_class = if *is_active {
        class!(btn_active btn_primary)
    } else {
        class!()
    };

    render! {
        li {
            key: "{channel_id}",
            class: "{item_class} {active_class}",
            Link {
                class: "w-full",
                to: Route::CabalChannelPage {
                    cabal_id: cabal_id.into(),
                    channel_id: channel_id.into()
                },
                span {
                    "{channel_id}",
                }
            }
        }
    }
}
