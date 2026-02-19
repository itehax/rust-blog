use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

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
    #[serde(default)]
    pub tags: Vec<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub href: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[server(GetGraphData, "/api")]
pub async fn get_graph_data() -> Result<GraphData, ServerFnError> {
    let result = tokio::task::spawn_blocking(move || {
        let mut post_paths = HashMap::new();
        post_paths.insert(PostType::Blog, "posts/blog");
        post_paths.insert(PostType::Writing, "posts/writing");

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (post_type, path) in &post_paths {
            let dir = std::path::Path::new(path);
            if !dir.exists() { continue; }
            let posts = process_posts(dir);
            for post in &posts {
                let href = format!("/{}/{}", post_type, post.post_metadata.create_href());
                nodes.push(GraphNode {
                    id: href.clone(),
                    title: post.post_metadata.title.clone(),
                    href,
                    tags: post.post_metadata.tags.clone(),
                });
            }
        }

        // Edges from shared tags
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        for node in &nodes {
            for tag in &node.tags {
                tag_map.entry(tag.clone()).or_default().push(node.id.clone());
            }
        }
        let mut edge_set: HashSet<(String, String)> = HashSet::new();
        for (_tag, ids) in &tag_map {
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    let a = ids[i].clone();
                    let b = ids[j].clone();
                    let key = if a < b { (a.clone(), b.clone()) } else { (b.clone(), a.clone()) };
                    if edge_set.insert(key) {
                        edges.push(GraphEdge {
                            source: a,
                            target: b,
                            label: _tag.clone(),
                        });
                    }
                }
            }
        }

        // Edges from internal links in post content
        let link_re = regex::Regex::new(r#"href="/(blog|writing|projects)/([^"]+)""#).unwrap();
        for (post_type, path) in &post_paths {
            let dir = std::path::Path::new(path);
            if !dir.exists() { continue; }
            let posts = process_posts(dir);
            for post in &posts {
                let source_href = format!("/{}/{}", post_type, post.post_metadata.create_href());
                for cap in link_re.captures_iter(&post.post_content) {
                    let target_href = format!("/{}/{}", &cap[1], &cap[2]);
                    if nodes.iter().any(|n| n.id == target_href) {
                        let key = if source_href < target_href {
                            (source_href.clone(), target_href.clone())
                        } else {
                            (target_href.clone(), source_href.clone())
                        };
                        if edge_set.insert(key) {
                            edges.push(GraphEdge {
                                source: source_href.clone(),
                                target: target_href,
                                label: "link".to_string(),
                            });
                        }
                    }
                }
            }
        }

        GraphData { nodes, edges }
    }).await;

    match result {
        Ok(data) => Ok(data),
        Err(e) => Err(ServerFnError::new(format!("Threading error: {}", e))),
    }
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

            // Post-process: transform [!author] blockquotes into styled callouts
            let author_re = regex::Regex::new(
                r"(?s)<blockquote>\s*<p>\[!author\]\s*(.*?)</p>\s*</blockquote>"
            ).unwrap();
            html_output = author_re.replace_all(&html_output, |caps: &regex::Captures| {
                let text = &caps[1];
                format!(
                    r#"<div class="flex flex-col sm:flex-row gap-4 items-center sm:items-start p-4 my-6 rounded-lg bg-[#161B22] border border-[#30363D] text-center sm:text-left"><img src="https://github.com/itehax.png" class="w-10 h-10 rounded-full shrink-0" alt="Author" /><div class="text-[#C9D1D9] text-sm leading-relaxed"><p>{}</p></div></div>"#,
                    text.trim()
                )
            }).to_string();

            // Post-process: [!code Title] + fenced code block → styled code panel
            //
            //   > [!code src/main.rs]
            //   ```rust
            //   fn main() { ... }
            //   ```
            //
            // Title is optional – omit it for an unlabelled code block.
            let code_re = regex::Regex::new(
                r#"(?s)<blockquote>\s*<p>\[!code(?:\s+([^\]]*))?\]</p>\s*</blockquote>\s*(<pre><code[^>]*>[\s\S]*?</code></pre>)"#
            ).unwrap();
            html_output = code_re.replace_all(&html_output, |caps: &regex::Captures| {
                let title = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
                let pre_block = &caps[2];
                let header = if title.is_empty() {
                    String::new()
                } else {
                    format!(
                        r#"<div class="itx-code-header flex items-center gap-2 px-4 py-2 bg-[#161B22] border-b border-[#30363D]"><svg class="w-3.5 h-3.5 text-[#8B949E] shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round"><path d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/></svg><span class="text-xs font-mono text-[#8B949E]">{}</span></div>"#,
                        title
                    )
                };
                format!(
                    r#"<div class="itx-code-block overflow-hidden rounded-lg border border-[#30363D] my-6">{}{}</div>"#,
                    header, pre_block
                )
            }).to_string();

            // Post-process: [!term Title] (or [!terminal Title]) + code block → terminal panel
            //
            //   > [!term cargo build]
            //   ```txt
            //   Compiling my_crate v0.1.0
            //   Finished release target
            //   ```
            //
            // Title is optional – defaults to "terminal".
            let term_re = regex::Regex::new(
                r#"(?s)<blockquote>\s*<p>\[!term(?:inal)?(?:\s+([^\]]*))?\]</p>\s*</blockquote>\s*(<pre><code[^>]*>[\s\S]*?</code></pre>)"#
            ).unwrap();
            html_output = term_re.replace_all(&html_output, |caps: &regex::Captures| {
                let title = caps.get(1)
                    .map(|m| m.as_str().trim())
                    .filter(|s| !s.is_empty())
                    .unwrap_or("terminal");
                let pre_block = &caps[2];
                format!(
                    r#"<div class="itx-term-block overflow-hidden rounded-lg my-6"><div class="itx-term-header flex items-center gap-1.5 px-4 py-2.5 bg-[#21262D]"><span class="w-3 h-3 rounded-full bg-[#FF5F57] shrink-0"></span><span class="w-3 h-3 rounded-full bg-[#FFBD2E] shrink-0"></span><span class="w-3 h-3 rounded-full bg-[#28C840] shrink-0"></span><span class="text-xs font-mono text-[#8B949E] ml-1">{}</span></div><div class="itx-term-body">{}</div></div>"#,
                    title, pre_block
                )
            }).to_string();

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
