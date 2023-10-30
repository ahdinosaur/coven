use dioxus::prelude::*;

turf::style_sheet!("src/components/cabal_container.scss");

#[inline_props]
pub(crate) fn CabalContainer<'a>(
    cx: Scope,
    top: Element<'a>,
    side: Element<'a>,
    content: Element<'a>,
) -> Element {
    render! {
        style { STYLE_SHEET }
        div {
            class: ClassName::COMPONENT,

            div {
                class: "top",
                &cx.props.top
            }

            div {
                class: "bottom",

                div {
                    class: "side",
                    &cx.props.side
                }
                div {
                    class: "content",
                    &cx.props.content
                }
            }
        }
    }
}
