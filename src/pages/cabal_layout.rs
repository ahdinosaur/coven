use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_router::prelude::*;

use crate::{components::ChannelsNav, pages::Route};

#[inline_props]
pub(crate) fn CabalLayout(cx: Scope) -> Element {
    render! {
        div {
            class: class!(flex flex_row gap_2 min_w_full),

            ChannelsNav {}
            Outlet::<Route> {}
        }
    }
}
