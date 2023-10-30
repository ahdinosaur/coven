use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

#[inline_props]
pub(crate) fn AppTopNav(cx: Scope) -> Element {
    render! {
        nav {
            Link {
                to: Route::AppHomePage {},
                "Coven"
            },
        }
    }
}
