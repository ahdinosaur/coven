use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_read;

use crate::{
    pages::Route,
    state::{ACTIVE_CABAL_ID, CABAL_IDS},
};

#[inline_props]
pub(crate) fn AppSideNav(cx: Scope) -> Element {
    let cabal_ids = use_read(cx, &CABAL_IDS);
    let _active_cabal_id = use_read(cx, &ACTIVE_CABAL_ID);

    render! {
        nav {
            class: class!(navbar flex_col),

            Link {
                to: Route::AppHomePage {},
                "Coven"
            },
            ul {
                aria_label: "Cabals",
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
