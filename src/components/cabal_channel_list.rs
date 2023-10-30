use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

#[inline_props]
pub(crate) fn CabalChannelList(cx: Scope, cabal_id: String, channel_ids: Vec<String>) -> Element {
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
