use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

use crate::components::post::RenderPost;
#[component]
pub fn RenderProjectsPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax Project Post"/>
        <Meta name="description" content="My Project Post."/>
        <RenderPost post_type=PostType::Project/>
    }
}
