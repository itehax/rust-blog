use crate::components::post::RenderPost;
use crate::server_functions::posts::PostType;
use leptos::*;
#[component]
pub fn RenderBlogPost(cx: Scope) -> impl IntoView {
    view! { cx, <RenderPost post_type=PostType::Blog/> }
}
