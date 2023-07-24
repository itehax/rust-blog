use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub image_path: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub project_link: String,
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
}

impl Post {
    pub fn new(post_metadata: PostMetadata, post_content: PostContent) -> Self {
        Self {
            post_metadata,
            post_content,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PostType {
    Blog,
    Project,
    Book,
}

impl std::fmt::Display for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PostType::Blog => write!(f, "blog"),
            PostType::Project => write!(f, "projects"),
            PostType::Book => write!(f, "books"),
        }
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<HashMap<PostType, Vec<Post>>, ServerFnError> {
    let mut post_paths = HashMap::new();
    post_paths.insert(PostType::Blog, "posts/blog");
    post_paths.insert(PostType::Project, "posts/projects");
    post_paths.insert(PostType::Book, "posts/books");

    let mut all_posts = HashMap::new();

    for (post_type, path) in post_paths {
        let posts = process_posts(path);
        all_posts.insert(post_type, posts);
    }
    Ok(all_posts)
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
            use pulldown_cmark::{html, Options, Parser};

            let mut options = Options::empty();
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            let matter = Matter::<YAML>::new();

            let post_data = matter
                .parse_with_struct::<PostMetadata>(content)
                .expect("Unable to parse md frontmatter");
            let post_metadata = post_data.data;

            let content = post_data.content;

            let parser = Parser::new_ext(&content, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            Some(Post::new(post_metadata, html_output))
        }

        pub fn sort_posts(posts: &mut [Post]) {
            posts.sort_by(|a, b| {
                let a_date = NaiveDate::parse_from_str(&a.post_metadata.date, "%Y-%m-%d").unwrap();
                let b_date = NaiveDate::parse_from_str(&b.post_metadata.date, "%Y-%m-%d").unwrap();
                //reverse sorting
                a_date.cmp(&b_date)
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
