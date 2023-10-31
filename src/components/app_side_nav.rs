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
    let active_cabal_id = use_read(cx, &ACTIVE_CABAL_ID);

    render! {
        nav {
            div {
                class: class!(w_12 flex flex_col gap_2),
                Link {
                    class: "w-full",
                    to: Route::AppHomePage {},
                    button {
                        class: class!(w_full btn btn_outline),
                        "🏠"
                    }
                }

                CabalList {
                    cabal_ids: cabal_ids.clone(),
                    active_cabal_id: active_cabal_id.clone(),
                }
            }
        }
    }
}

#[inline_props]
fn CabalList(
    cx: Scope,
    cabal_ids: Vec<String>,
    #[props(!optional)] active_cabal_id: Option<String>,
) -> Element {
    render! {
        ul {
            class: class!(w_full join join_vertical),
            aria_label: "Cabals",
            for cabal_id in cabal_ids.iter() {
                CabalListItem {
                    cabal_id: cabal_id.clone(),
                    is_active: Some(cabal_id) == active_cabal_id.as_ref(),
                }
            }
        }
    }
}

#[inline_props]
fn CabalListItem(cx: Scope, cabal_id: String, is_active: bool) -> Element {
    let item_class = class!(w_full btn btn_outline join_item);
    let active_class = if *is_active {
        class!(btn_active btn_primary)
    } else {
        class!()
    };

    render! {
        li {
            key: "{cabal_id}",
            class: "{item_class} {active_class}",
            Link {
                class: "w-full",
                to: Route::CabalHomePage { cabal_id: cabal_id.into() },
                button {
                    class: class!(w_full),
                    "{&cabal_id[0..2]}",
                }
            }
        }
    }
}
