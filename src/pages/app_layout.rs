use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_router::prelude::*;

use crate::{components::AppSideNav, pages::Route};

#[inline_props]
pub(crate) fn AppLayout(cx: Scope) -> Element {
    render! {
        div {
            class: class!(flex flex_row),

            AppSideNav {},

            Outlet::<Route> {},
        }
    }
}
