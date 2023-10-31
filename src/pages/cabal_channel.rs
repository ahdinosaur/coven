use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use fermi::{use_atom_root, use_read};

use crate::{
    components::ChannelPosts,
    hooks::{use_active_cabal_id, use_active_channel_id, use_service},
    state::CABAL_CHANNEL_POSTS,
};

#[inline_props]
pub(crate) fn CabalChannelPage(cx: Scope, cabal_id: String, channel_id: String) -> Element {
    use_active_cabal_id(cx, cabal_id.into());
    use_active_channel_id(cx, channel_id.into());

    let service = use_service(cx);
    let atoms = use_atom_root(cx);
    use_coroutine(cx, |_rx: UnboundedReceiver<()>| {
        to_owned![service, atoms, channel_id];
        async move { service.open_channel(atoms, channel_id).await }
    });

    let posts = use_read(cx, &CABAL_CHANNEL_POSTS);

    render! {
        div {
            class: class!(grow min_h_screen bg_base_200),
            div {
                class: class!(text_center),
                div {
                    class: class!(max_w_md),
                    h1 {
                        class: class!(text_5xl font_bold),
                        "Channel: {channel_id}"
                    }
                }
            }

            if let Some(posts) = posts {
                rsx!(ChannelPosts {
                    posts: posts.clone()
                })
            }
        }
    }
}
