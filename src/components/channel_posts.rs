use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use crate::state::Post;

#[inline_props]
pub(crate) fn ChannelPosts(cx: Scope, posts: Vec<Post>) -> Element {
    render! {
        ul {
            class: class!(),

            for post in posts {
                li {
                    class: class!(chat chat_start),

                    div {
                        class: class!(chat_bubble),
                        "{post.text}"
                    }
                }
            }
        }
    }
}
