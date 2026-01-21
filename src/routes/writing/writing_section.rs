use crate::components::post::Post;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn WritingPost() -> impl IntoView {
     let title = "Writing | Thoughts on Technology & Security by Edoardo D'Errico";
    let description = "Personal thoughts, reflections, and insights on cybersecurity, technology, coding, and creative problem-solving. Unfiltered perspectives from Edoardo D'Errico.";
    let url = "https://edoardoderrico.com/writing";
    view! {
        <Title text=title />
        <Meta name="description" content=description />

        // Open Graph / Facebook
        <Meta property="og:url" content=url />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=title />
        <Meta property="og:description" content=description />
        // <Meta property="og:image" content={image}/>

        // Twitter
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta property="twitter:domain" content="edoardoderrico.com" />
        <Meta property="twitter:url" content=url />
        <Meta name="twitter:title" content=title />
        <Meta name="twitter:description" content=description />
        // <Meta name="twitter:image" content={image}/>

        // Canonical URL
        <Link rel="canonical" href=url />
        <Post
            post_type=PostType::Writing
            post_description="Posts about stuff i wrote.".to_string()
        />
    }
}
