use cable_core::MemoryStore;
use dioxus::prelude::*;
use fermi::use_set;

use crate::service::Service;
use crate::state::{ACTIVE_CABAL_CHANNEL_ID, ACTIVE_CABAL_ID};

type Store = MemoryStore;

pub(crate) fn use_service(cx: &ScopeState) -> Service<Store> {
    use_context::<Service<Store>>(cx).unwrap().clone()
}

pub(crate) fn use_service_provider(cx: &ScopeState) {
    use_context_provider(cx, || Service::<Store>::default());
}

pub(crate) fn use_active_cabal_id(cx: &ScopeState, cabal_id: String) {
    let set_active_cabal_id = use_set(cx, &ACTIVE_CABAL_ID);
    use_effect(cx, (), |_| {
        to_owned![set_active_cabal_id];
        async move {
            set_active_cabal_id(Some(cabal_id));
        }
    });
    use_on_unmount(cx, {
        to_owned![set_active_cabal_id];
        move || {
            set_active_cabal_id(None);
        }
    });
}

pub(crate) fn use_active_channel_id(cx: &ScopeState, channel_id: String) {
    let set_active_channel_id = use_set(cx, &ACTIVE_CABAL_CHANNEL_ID);
    use_effect(cx, (), |_| {
        to_owned![set_active_channel_id];
        async move {
            set_active_channel_id(Some(channel_id));
        }
    });
    use_on_unmount(cx, {
        to_owned![set_active_channel_id];
        move || {
            set_active_channel_id(None);
        }
    });
}
