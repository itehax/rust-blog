use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn Manifesto() -> impl IntoView {
    view! {
        <Title text="Itehax Manifesto" />
        <Meta name="description" content="Edoardo D'Errico Manifesto" />
        <Body class="bg-[#080A21]" />
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                  
                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string() url="".to_string() />
        <HomeFooter />
    }
}
