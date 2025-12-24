use crate::components::post::Post;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn BlogPost() -> impl IntoView {
    view! {
        <Title text="Itehax Blog" />
        <Meta name="description" content="My Blog Posts." />
        <Post
            post_type=PostType::Blog
            post_description="Posts about tutorial and explanation of projects i built.".to_string()
        />
    }
}
