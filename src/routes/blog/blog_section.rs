use crate::components::post::Post;
use crate::server_functions::posts::PostType;
use leptos::Scope;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn BlogPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax Blog"/>
        <Meta name="description" content="My Blog Posts."/>
        <Post
            post_type=PostType::Blog
            post_description="Posts about tutorial and project i built.".to_string()
        />
    }
}
