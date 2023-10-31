use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[inline_props]
pub(crate) fn AppHomePage(cx: Scope) -> Element {
    render! {
        div {
            class: class!(hero min_h_screen bg_base_200),
            div {
                class: class!(hero_content text_center),
                div {
                    class: class!(max_w_md),
                    h1 {
                        class: class!(text_5xl font_bold),
                        "Welcome to Coven"
                    }
                    p {
                        class: class!(py_6),
                        "An occult chat app using Cabal 🧙✨🔮"
                    }
                }
            }
        }
    }
}
