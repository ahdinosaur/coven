use dioxus::prelude::*;
use fermi::use_read;

use crate::components::CabalNavList;
use crate::state::CABAL_IDS;

#[inline_props]
pub(crate) fn AppHomePage(cx: Scope) -> Element {
    let cabal_ids = use_read(cx, &CABAL_IDS);

    render! {
        div {
            h1 {
                "Welcome to Cabal"
            },
            section {
                h2 {
                    "Cabals:"
                }
                CabalNavList { cabal_ids: cabal_ids.clone() }
            }
        }
    }
}
