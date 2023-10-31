use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use crate::hooks::use_active_cabal_id;

#[inline_props]
pub(crate) fn CabalHomePage(cx: Scope, cabal_id: String) -> Element {
    use_active_cabal_id(cx, cabal_id.into());

    render! {
        div {
            class: class!(hero grow min_h_screen bg_base_200),
            div {
                class: class!(hero_content text_center),
                div {
                    class: class!(max_w_md),
                    h1 {
                        class: class!(text_5xl font_bold),
                        "Cabal: {cabal_id}"
                    }
                }
            }
        }
    }
}
