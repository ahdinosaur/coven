use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

#[inline_props]
pub(crate) fn AppNav(cx: Scope, cabal_ids: Vec<String>) -> Element {
    render! {
        nav {
            Link {
                to: Route::AppHomePage {},
                "Home"
            },
            ul {
                for cabal_id in cabal_ids {
                    li {
                        key: "{cabal_id}",
                        Link {
                            to: Route::CabalHomePage { cabal_id: cabal_id.into() },
                            "{cabal_id}",
                        }
                    }
                }
            }
        }
    }
}
