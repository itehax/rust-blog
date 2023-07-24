use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::Scope;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProjectsPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax Projects"/>
        <Meta name="description" content="My Projects Posts."/>
        <Post
            post_type=PostType::Project
            post_description="Posts about projects i created.".to_string()
        />
    }
}
