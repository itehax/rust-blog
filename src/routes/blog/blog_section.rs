use crate::components::post::Post;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn BlogPost() -> impl IntoView {
let title = "Blog | Cybersecurity & CTF Writeups by Edoardo D'Errico";
    let description = "Read technical writeups on cryptography challenges, CTF solutions, exploit development, and security research. Deep dives into real-world cybersecurity problems.";
    let url = "https://edoardoderrico.com/blog";
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

        // Additional SEO for blog listing
        <Link rel="canonical" href=url />
        <Post
            post_type=PostType::Blog
            post_description="Posts about tutorial and explanation of projects i built.".to_string()
        />
    }
}
