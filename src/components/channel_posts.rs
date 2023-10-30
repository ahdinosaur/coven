use dioxus::prelude::*;

use crate::state::Post;

#[inline_props]
pub(crate) fn ChannelPosts(cx: Scope, posts: Vec<Post>) -> Element {
    render! {
        ul {
            for post in posts {
                li {
                    "{post.text}"
                }
            }
        }
    }
}
