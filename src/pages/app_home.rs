use dioxus::prelude::*;

#[inline_props]
pub(crate) fn AppHomePage(cx: Scope) -> Element {
    render! {
        div {
            h1 {
                "Welcome to Cabal"
            }
        }
    }
}
