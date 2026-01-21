use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProjectsPost() -> impl IntoView {
    let title = "Projects | Edoardo D'Errico - Cybersecurity Engineer & Developer";
    let description = "Portfolio of cybersecurity projects, CTF challenges, and security tools by Edoardo D'Errico. Expertise in cryptography, exploit development, and secure system design.";
    let url = "https://edoardoderrico.com/projects";
    view! {
        <Title text=title />
        <Meta name="description" content=description />
        <Meta
            name="keywords"
            content="cybersecurity engineer, security researcher, CTF, cryptography, exploit development, penetration testing, security tools, portfolio"
        />

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
            post_type=PostType::Project
            post_description="Posts about projects i created.".to_string()
        />
    }
}
