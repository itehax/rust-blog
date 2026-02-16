use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub image_path: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub project_link: String,
    pub seo_title: Option<String>,
    #[serde(default)]
    pub seo_description: Option<String>,
    #[serde(default)]
    pub seo_image: Option<String>,
    #[serde(default)]
    pub seo_keywords: Option<String>,
}

impl PostMetadata {
    pub fn create_href(&self) -> String {
        self.title.replace(' ', "-").to_lowercase()
    }
}

pub type PostContent = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_metadata: PostMetadata,
    pub post_content: PostContent,
    pub toc: Vec<TocItem>,
}

impl Post {
    pub fn new(post_metadata: PostMetadata, post_content: PostContent, toc: Vec<TocItem>) -> Self {
        Self {
            post_metadata,
            post_content,
            toc,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PostType {
    Blog,
    Project,
    Notes,
    Writing,
}

impl std::fmt::Display for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PostType::Blog => write!(f, "blog"),
            PostType::Project => write!(f, "projects"),
            PostType::Notes => write!(f, "notes"),
            PostType::Writing => write!(f, "writing"),
        }
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<HashMap<PostType, Vec<Post>>, ServerFnError> {
    let result = tokio::task::spawn_blocking(move || {
        let mut post_paths = HashMap::new();
        post_paths.insert(PostType::Blog, "posts/blog");
        post_paths.insert(PostType::Project, "posts/projects");
        post_paths.insert(PostType::Writing, "posts/writing");

        let mut all_posts = HashMap::new();

        for (post_type, path) in post_paths {
            let posts = process_posts(path);
            all_posts.insert(post_type, posts);
        }
        all_posts
    })
    .await;
    match result {
        Ok(posts) => Ok(posts),
        Err(e) => Err(ServerFnError::new(format!("Threading error: {}", e))),
    }
}
#[server(GetLastUpdate, "/api")]
pub async fn get_last_update() -> Result<String, ServerFnError> {
    Ok(std::env::var("LAST_UPDATED").unwrap_or_else(|_| "Date not Found".to_string()))
}
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{
            fs::{self, DirEntry},
            path::Path,
        };
        use chrono::NaiveDate;

        pub fn get_posts_file<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
            fs::read_dir(path)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| {
                    if let Ok(file_type) = entry.file_type() {
                        file_type.is_file() && entry.path().extension() == Some("md".as_ref())
                    } else {
                        false
                    }
                })
                .collect()
        }

        pub fn read_post_content(entry: DirEntry) -> Option<String> {
            fs::read_to_string(entry.path()).ok()
        }

        pub fn parse_post_content(content: &str) -> Option<Post> {
            use gray_matter::engine::YAML;
            use gray_matter::Matter;
            use pulldown_cmark::{html, Event, Tag, TagEnd, HeadingLevel, Options, Parser};

            let mut options = Options::empty();
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            let matter = Matter::<YAML>::new();

            let post_data = matter
                .parse_with_struct::<PostMetadata>(content)
                .expect("Unable to parse md frontmatter");
            let post_metadata = post_data.data;

            let content = post_data.content;

            let parser = Parser::new_ext(&content, options);

            // First pass: collect TOC and add IDs to headings
            let mut toc = Vec::new();
            let mut events = Vec::new();
            let mut current_heading_level = None;
            let mut current_heading_text = String::new();

            for event in parser {
                match &event {
                    Event::Start(Tag::Heading { level, .. }) => {
                        current_heading_level = Some(*level);
                        current_heading_text.clear();
                    }
                    Event::Text(text) if current_heading_level.is_some() => {
                        current_heading_text.push_str(text);
                    }
                    Event::End(TagEnd::Heading(_level)) => {
                        if let Some(heading_level) = current_heading_level {
                            // Create ID from heading text
                            let id = current_heading_text
                                .to_lowercase()
                                .chars()
                                .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
                                .collect::<String>()
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .join("-");

                            // Add to TOC (only h2)
                            let level_num = match heading_level {
                                HeadingLevel::H1 => 1,
                                HeadingLevel::H2 => 2,
                                HeadingLevel::H3 => 3,
                                HeadingLevel::H4 => 4,
                                HeadingLevel::H5 => 5,
                                HeadingLevel::H6 => 6,
                            };

                            if level_num == 2 {
                                toc.push(TocItem {
                                    id: id.clone(),
                                    text: current_heading_text.clone(),
                                    level: level_num,
                                });
                            }

                            // Replace heading with one that has an ID
                            events.push(Event::Html(format!(r#"<h{} id="{}">"#, level_num, id).into()));
                            events.push(Event::Text(current_heading_text.clone().into()));
                            events.push(Event::Html(format!(r#"</h{}>"#, level_num).into()));

                            current_heading_level = None;
                            continue;
                        }
                    }
                    _ => {}
                }

                // Don't add heading events as we're replacing them
                if current_heading_level.is_none() {
                    events.push(event);
                }
            }

            let mut html_output = String::new();
            html::push_html(&mut html_output, events.into_iter());

            Some(Post::new(post_metadata, html_output, toc))
        }

        pub fn sort_posts(posts: &mut [Post]) {
            posts.sort_by(|a, b| {
                let a_date = NaiveDate::parse_from_str(&a.post_metadata.date, "%Y-%m-%d").unwrap();
                let b_date = NaiveDate::parse_from_str(&b.post_metadata.date, "%Y-%m-%d").unwrap();
                b_date.cmp(&a_date)
            });
        }

        pub fn process_posts<P: AsRef<Path>>(path: P) -> Vec<Post> {
            let posts_text = get_posts_file(path);
            let mut posts = Vec::new();

            for entry in posts_text {
                if let Some(content) = read_post_content(entry) {
                    if let Some(post) = parse_post_content(&content) {
                        posts.push(post);
                    }
                }
            }

            sort_posts(&mut posts);

            posts
        }

    }
}
