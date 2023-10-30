use dioxus::prelude::*;

#[inline_props]
pub(crate) fn AppContainer<'a>(cx: Scope, nav: Element<'a>, main: Element<'a>) -> Element {
    render! {
        div {
            div {
                &cx.props.nav
            }
            div {
                &cx.props.main
            }
        }
    }
}
