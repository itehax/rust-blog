use crate::components::post::RenderPost;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn RenderBooksPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax Book Post"/>
        <Meta name="description" content="My Book Post."/>
        <RenderPost post_type=PostType::Book/>
    }
}
