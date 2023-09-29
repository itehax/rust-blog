cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::server_functions::posts::get_posts;
        use axum::{
            http::{header::CONTENT_TYPE, HeaderMap},
            response::IntoResponse,
        };
        use chrono::NaiveDate;

        pub async fn rss_feed() -> impl IntoResponse {
            let xml = build_rss_xml().await;
            let headers = build_headers();
            (headers, xml)
        }

        async fn build_rss_xml() -> String {
            let mut xml = String::new();
            xml.push_str(
                r#"<?xml version="1.0" encoding="utf-8"?>
                <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
                <channel><title>Itehax</title>
                <link>https://itehax.com</link>
                <description>Itehax website, coding, hacking, reading and much more.</description>
                <language>en-us</language>
                <atom:link href="https://itehax.com/rss.xml" rel="self" type="application/rss+xml"/>"#,
            );

            let posts = get_posts().await.unwrap();
            for (post_type, posts_vec) in posts {
                for post in posts_vec {
                    //todo add real time to rfc2822 conversion.
                    let parsed_date = NaiveDate::parse_from_str(&post.post_metadata.date, "%Y-%m-%d")
                        .expect("Failed to parse date string")
                        .format("%a, %d %b %Y 16:00:10 GMT")
                        .to_string();

                    let post_name = post.post_metadata.create_href();

                    let channel = format!(
                        r#"<item><title>{}</title><link>https://itehax.com/{}/{}</link><description>{}</description><language>en-us</language><pubDate>{}</pubDate><guid isPermaLink="false">{}</guid></item>"#,
                        post.post_metadata.title,
                        post_type.to_string(),
                        post_name,
                        post.post_metadata.description,
                        parsed_date,
                        post_name,
                    );

                    xml.push_str(&channel);
                }
            }

            xml.push_str(r#"</channel></rss>"#);
            xml
        }

        fn build_headers() -> HeaderMap {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());
            headers
        }
    }
}
