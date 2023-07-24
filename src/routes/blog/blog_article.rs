use crate::components::post::RenderPost;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn RenderBlogPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax Blog Post"/>
        <Meta name="description" content="My Blog Post."/>
        <RenderPost post_type=PostType::Blog/>
    }
}
