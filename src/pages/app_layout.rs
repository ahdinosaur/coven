use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

#[inline_props]
pub(crate) fn AppLayout(cx: Scope) -> Element {
    render! {
        Outlet::<Route> {}
    }
}
