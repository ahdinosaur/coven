use dioxus::prelude::*;

use crate::hooks::use_active_cabal_id;

#[inline_props]
pub(crate) fn CabalHomePage(cx: Scope, cabal_id: String) -> Element {
    use_active_cabal_id(cx, cabal_id.into());

    render! {
        div {
            h1 {
                "Cabal: {cabal_id}"
            }
        }
    }
}
