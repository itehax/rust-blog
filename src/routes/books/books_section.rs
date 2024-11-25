use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn BooksPost() -> impl IntoView {
    view! {
        <Title text="Itehax Books"/>
        <Meta name="description" content="My Books Posts."/>
        <Post post_type=PostType::Book post_description="Posts about books i read.".to_string()/>
    }
}
