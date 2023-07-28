---
image_path: "/images/blog_images/writing_website.jpg"
title: Writing a Blog in Rust
date: 2023-07-22
description: How i wrote my personal Website and what i learned.
project_link: https://github.com/itehax/itehax-website
---

# Writing a blog using rust

**Foreword**: This is my first blog post ever made, in this site I will write several articles, from the more technical to the lighter, one thing they will both have in common is that I will try to write in the simplest way possible.

---

## Let's start

### Why
First I will talk about why I decided to use [Rust](https://www.rust-lang.org/it), [Leptos](https://leptos.dev/)  and [Preline](https://www.preline.co/).

#### Why **Rust**
I decided to use Rust, because in addition to the usual reasons for using it (not at all obvious), such as memory Safety, zero-cost abstractions, concurrency without data races,
performance, functional and procedural Paradigms,
ergonomics, great tooling,community and documentation, etc. , it has the possibility of being compiled in [wasm](https://webassembly.org/), a world unknown to me and one that I wanted to discover and hear a lot about in recent times, especially because of its excellent performance.
In addition, Rust is my favorite language and as such I wanted to see what the current ecosystem was like in the frontend and backend areas and whether it was really worth using.

#### Why **Leptos**
After watching some [videos](https://www.youtube.com/watch?v=4KtotxNAwME) of the main developer of this project, who I thank for helping me to fix some bugs, I inquired about other libraries available to develop a fullstack application.
Besides being the best performing (Dioxus has similar if not equal performance) it is the fastest growing library, with the most contributors and I really enjoyed the community.

[<img src="/images/blog_images/frontend-comparison.png">](https://github.com/flosse/rust-web-framework-comparison) 
Very important fact that made me decide to use it is the possibility to do ssr, that is server-side-rendering, coming from next.js I was looking for something similar,react like, and that thanks to this feature would give me good performance and good SEO.

#### Why **Preline**
I looked for several ui component libraries that used tailwindcss and this one was the most appealing, both in look and use, as well as being free.

---

## How i build the website

Leaving aside the frontend part, the logic of the blog is to read the md files in a folder and parse them, and then go on to create pages with these contents.

In particular, these md files consist of two parts.

The first is the [front matter](https://frontmatter.codes/docs/markdown), which is a way of identifying metadata in a markdown file.
Metadata is highly flexible and can encompass any data you desire, frequently utilised for storing page-specific information that doesn't need direct display. Common metadata includes elements such as the post's title or post's description.

The second is the content that will then be converted into html.

One thing I liked about leptos are the [**server functions**](https://leptos-rs.github.io/leptos/server/25_server_functions.html), i.e. functions that can use server-only dependencies, like sqlx, and can access server-only resources, like our database or our md files.

This is how my server function looks like!

<div class="bg-blue-950 overflow-hidden rounded-md">
                <div class="flex justify-between px-4 items-center text-xs text-white">
                    <p class="text-sm">Rust</p>
                </div>

```rust
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

```
</div>

As you can see from the code, the function returns if everything went correctly a hashmap that has as its key the PostType type and as its value a Vec of Post.

In particular, since the logic of the projects and books section is the same as that of the blog, I decided to avoid duplicating code and associate each post type with its corresponding files.

You can find the definition of these types here. 

<div class="bg-blue-950 overflow-hidden rounded-md">
                <div class="flex justify-between px-4 items-center text-xs text-white">
                    <p class="text-sm">Rust</p>
                </div>

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub image_path: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub project_link: String,
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

```
</div>

Finally, here is the part that deals with parsing md files.

<div class="bg-blue-950 overflow-hidden rounded-md">
                <div class="flex justify-between px-4 items-center text-xs text-white">
                    <p class="text-sm">Rust</p>
                </div>

```rust
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

```
</div>

---

## Talking about deployment

First of all, it should be specified that depending on the rendering option used (csr or ssr), the deployment process varies.
In my case, I had to create a **dockerfile** and then upload it to a hosting that would allow its use.

That's the dockerfile.

<div class="bg-blue-950 overflow-hidden rounded-md">
                <div class="flex justify-between px-4 items-center text-xs text-white">
                    <p class="text-sm">Dockerfile</p>
                </div>

```dockerfile
FROM rustlang/rust:nightly-bullseye as builder

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/public /app/public

COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/itehax-website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="itehax-website"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD [ "/app/itehax-website" ]

```
</div>

Nothing really hard, i actually tried to use [cargo chef](https://github.com/LukeMathWalker/cargo-chef) to speed up the building process, but it didn't worked, probably because of the way cargo.toml is structured, however I didn't investigate all that much at the moment.

### CI/CD

I decided to deploy on [fly.io](fly.io), mainly because it has a free plan, plus I had heard a lot of good things about it so I decided to try it and I was very happy with it, especially the speed of deployment and abstraction.

This is the code that takes advantage of **github actions** to make sure that each time the code is pushed into the main, it is automatically deployed

<div class="bg-blue-950 overflow-hidden rounded-md">
                <div class="flex justify-between px-4 items-center text-xs text-white">
                    <p class="text-sm">Yml</p>
                </div>

```yml
name: Fly Deploy
on:
  push:
    branches:
      - main
jobs:
  deploy:
    name: Deploy app
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}

```
</div>

---

## Conclusion and considerations

It was fun to write my blog in rust, I learned various things and found in leptos all the features I was looking for.
Two problems I found are the fact that by using the macros provided by leptos to write the code, I could not take advantage of the features provided by the language server and therefore wasting a little more time.
Also, in the case of server-side-rendering to build a leptos application there is a need to use an external tool called [cargo-leptos](https://github.com/leptos-rs/cargo-leptos), no problem except for the fact that it takes a while to compile the files, so wanting to write the ui directly in leptos will make you lose a lot more time than expected, so I created an html project and then ported it to rust.

In **conclusion**, I recommend leptos to those who specialize in rust and want a strong type system and use webassembly.
Definitely leptos has a medium difficulty learning path, once you get used to it you will become __more productive__.