use crate::server_functions::posts::PostType;
use leptos::*;

use crate::components::post::RenderPost;
#[component]
pub fn RenderProjectsPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <RenderPost post_type=PostType::Project/>
    }
}
