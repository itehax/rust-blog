use crate::components::post::Post;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn WritingPost() -> impl IntoView {
    view! {
        <Title text="Itehax Blog"/>
        <Meta name="description" content="Stuff i wrote (poetry and so on)"/>
        <Post
            post_type=PostType::Writing
            post_description="Posts about stuff i wrote.".to_string()
        />
    }
}
