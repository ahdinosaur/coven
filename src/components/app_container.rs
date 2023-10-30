use dioxus::prelude::*;

turf::style_sheet!("src/components/app_container.scss");

#[inline_props]
pub(crate) fn AppContainer<'a>(
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
                class: ClassName::TOP,
                &cx.props.top
            }

            div {
                class: ClassName::BOTTOM,

                div {
                    class: ClassName::SIDE,
                    &cx.props.side
                }
                div {
                    class: ClassName::CONTENT,
                    &cx.props.content
                }
            }
        }
    }
}
