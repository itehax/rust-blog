use crate::server_functions::posts::PostType;
use leptos::*;

use crate::components::post::RenderPost;
#[component]
pub fn RenderProjectsPost() -> impl IntoView {
    view! { <RenderPost post_type=PostType::Project /> }
}
