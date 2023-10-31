use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_router::prelude::*;

use crate::{components::AppSideNav, pages::Route};

#[inline_props]
pub(crate) fn AppLayout(cx: Scope) -> Element {
    let create_eval = use_eval(cx);
    let themer = create_eval(
        r#"
        const theme = await dioxus.recv();
        const htmlElement = document.querySelector('html')
        htmlElement.setAttribute('data-theme', theme)
        "#,
    )
    .unwrap();
    themer.send("dark".into()).unwrap();

    render! {
        div {
            class: class!(flex flex_row gap_2 min_w_full),

            AppSideNav {},

            Outlet::<Route> {},
        }
    }
}
