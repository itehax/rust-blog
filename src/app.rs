use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::{about::About, blog::{blog_article::RenderBlogPost, blog_section::BlogPost}, home::Home, writing::{writing_article::RenderWritingPost, writing_section::WritingPost}},
    server_functions::posts::{get_last_update, get_posts},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let posts = create_blocking_resource(|| (), |_| async move { get_posts().await });
    provide_context(posts);

    let last_update = create_resource(|| (), |_| async move { get_last_update().await });
    provide_context(last_update);
    view! {
        <Stylesheet id="leptos" href="/pkg/itehax-website.css" />
        <Link rel="shortcut icon" type_="image/png" href="/phrack_icon1.png" />
        <Link
            href="https://fonts.googleapis.com/css2?family=Anonymous+Pro:ital,wght@0,400;0,700;1,400;1,700&display=swap"
            rel="stylesheet"
        />
        // <Link href="/fonts/ibm.css" rel="stylesheet"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <Routes>
                <Route path="" view=Home />

                <Route path="/blog" view=BlogPost />

                <Route
                    path="/blog/:post"
                    view=move || {
                        view! {
                            <Link rel="stylesheet" href="/highlighter/styles/github.min.css" />
                            <Link
                                rel="stylesheet"
                                href=r#"https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"#
                                integrity="sha384-nB0miv6/jRmo5UMMR1wu3Gz6NLsoTkbqJghGIsx//Rlm+ZU03BU6SQNC66uf4l5+"
                                crossorigin="anonymous"
                            />
                            <Link rel="stylesheet" href="/highlighter/styles/katex.css" />

                            <script defer src="/highlighter/load_highlight.js"></script>
                            <RenderBlogPost />
                        }
                    }
                />

                <Route path="/about" view=About />
                // <Route path="/manifesto" view=Manifesto /> TBA

                <Route path="/writing" view=WritingPost />
                <Route path="/writing/:post" view=RenderWritingPost />

            //    <Route path="/projects" view=ProjectsPost />
            //    <Route path="/projects/:post" view=RenderProjectsPost />
            </Routes>
            <script src="/preline/preline.js"></script>
        </Router>
    }
}
