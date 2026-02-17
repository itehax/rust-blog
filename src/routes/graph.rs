use crate::components::footer::{GoBack, HomeFooter};
use crate::components::graph::GraphView;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn GraphPage() -> impl IntoView {
    view! {
        <Body class="bg-[#0D1117]" />
        <Title text="Post Connections | itehax" />
        <div class="max-w-5xl mx-auto px-4 py-16 sm:px-6 lg:px-8">
            <div class="text-center mb-10">
                <h1 class="text-3xl font-bold text-[#E6EDF3] mb-2">"Post Connections"</h1>
                <p class="text-[#8B949E]">
                    "An interactive graph showing how posts are connected through shared tags and links."
                </p>
            </div>
            <div class="border border-[#30363D] rounded-xl p-4 bg-[#161B22]">
                <GraphView />
            </div>
        </div>
        <GoBack content="Back to Home".to_string() url="".to_string() />
        <HomeFooter />
    }
}
