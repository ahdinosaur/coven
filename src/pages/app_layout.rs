use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_read;

use crate::{
    components::{AppContainer, AppTopNav, CabalsNav},
    pages::Route,
    state::{ACTIVE_CABAL_ID, CABAL_IDS},
};

#[inline_props]
pub(crate) fn AppLayout(cx: Scope) -> Element {
    let cabal_ids = use_read(cx, &CABAL_IDS);
    let _active_cabal_id = use_read(cx, &ACTIVE_CABAL_ID);

    render! {
        AppContainer {
            top: render!(AppTopNav {}),
            side: render!(CabalsNav {
                cabal_ids: cabal_ids.clone()
            }),
            content: render!(Outlet::<Route> {}),
        }
    }
}
