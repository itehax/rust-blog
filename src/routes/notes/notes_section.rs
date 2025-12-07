use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotesPost() -> impl IntoView {
    view! {
        <Title text="Itehax Notes" />
        <Meta name="description" content="Notes on what i learn" />
        <Post post_type=PostType::Notes post_description="Notes on what i learn.".to_string() />
    }
}
