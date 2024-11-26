use crate::components::post::RenderPost;
use crate::server_functions::posts::PostType;
use leptos::*;
#[component]
pub fn RenderNotesPost() -> impl IntoView {
    view! { <RenderPost post_type=PostType::Notes/> }
}
