use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProjectsPost() -> impl IntoView {
    view! {
        <Title text="Itehax Projects" />
        <Meta name="description" content="My Projects Posts." />
        <Post
            post_type=PostType::Project
            post_description="Posts about projects i created.".to_string()
        />
    }
}
