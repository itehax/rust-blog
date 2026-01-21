use crate::server_functions::posts::{PostMetadata, PostType};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn PostSeo(post_metadata: PostMetadata, post_type: PostType) -> impl IntoView {
    // use SEO fields if provided, otherwise fall back to regular fields
    let seo_title = post_metadata
        .seo_title
        .as_ref()
        .cloned()
        .unwrap_or_else(|| format!("{} | Edoardo D'Errico", post_metadata.title));

    let seo_description = post_metadata
        .seo_description
        .as_ref()
        .cloned()
        .unwrap_or_else(|| post_metadata.description.clone());

    let seo_image = post_metadata
        .seo_image
        .as_ref()
        .cloned()
        .unwrap_or_else(|| format!("https://edoardoderrico.com{}", post_metadata.image_path));

    let url = format!(
        "https://edoardoderrico.com/{}/{}",
        post_type,
        post_metadata.create_href()
    );

    view! {
        <Title text=seo_title.clone() />
        <Meta name="description" content=seo_description.clone() />
        <Meta name="author" content="Edoardo D'Errico" />

        {post_metadata
            .seo_keywords
            .as_ref()
            .map(|keywords| view! { <Meta name="keywords" content=keywords.clone() /> })}

        // Open Graph
        <Meta property="og:url" content=url.clone() />
        <Meta property="og:type" content="article" />
        <Meta property="og:title" content=seo_title.clone() />
        <Meta property="og:description" content=seo_description.clone() />
        <Meta property="og:image" content=seo_image.clone() />
        <Meta property="article:published_time" content=post_metadata.date.clone() />
        <Meta property="article:author" content="Edoardo D'Errico" />

        // Twitter
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta property="twitter:domain" content="edoardoderrico.com" />
        <Meta property="twitter:url" content=url.clone() />
        <Meta name="twitter:title" content=seo_title />
        <Meta name="twitter:description" content=seo_description />
        <Meta name="twitter:image" content=seo_image />

        // Canonical
        <Link rel="canonical" href=url />
    }
}
