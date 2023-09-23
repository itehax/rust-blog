use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::blog::blog_article::RenderBlogPost,
    routes::{
        about::About,
        blog::blog_section::BlogPost,
        books::{books_article::RenderBooksPost, books_section::BooksPost},
        hire_me::HireMe,
        home::Home,
        projects::{projects_article::RenderProjectsPost, projects_section::ProjectsPost}, feed::{ RssFeed},
    },
    server_functions::posts::get_posts,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let posts = create_resource(cx, || (), |_| async move { get_posts().await });
    provide_context(cx, posts);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/itehax-website.css"/>
        <Link rel="shortcut icon" type_="image/png" href="/favicon.png"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@200;500;700&display=swap"
            rel="stylesheet"
        />
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <Routes>
                <Route
                    path=""
                    view=move |cx| {
                        view! { cx, <Home/> }
                    }
                />

                <Route
                    path="/blog"
                    view=move |cx| {
                        view! { cx, <BlogPost/> }
                    }
                />

                <Route
                    path="/blog/:post"
                    view=move |cx| {
                        view! { cx,
                            <Link rel="stylesheet" href="/highlighter/styles/github.min.css"/>
                            <script src="/highlighter/load_highlight.js"></script>
                            <RenderBlogPost/>
                        }
                    }
                />

                <Route
                    path="/about"
                    view=move |cx| {
                        view! { cx, <About/> }
                    }
                />

                <Route
                    path="/hire-me"
                    view=move |cx| {
                        view! { cx, <HireMe/> }
                    }
                />

                <Route
                    path="/books"
                    view=move |cx| {
                        view! { cx, <BooksPost/> }
                    }
                />

                <Route
                    path="/books/:post"
                    view=move |cx| {
                        view! { cx, <RenderBooksPost/> }
                    }
                />

                <Route
                    path="/projects"
                    view=move |cx| {
                        view! { cx, <ProjectsPost/> }
                    }
                />

                <Route
                    path="/projects/:post"
                    view=move |cx| {
                        view! { cx, <RenderProjectsPost/> }
                    }
                />
            </Routes>
            <script src="/preline/preline.js"></script>
        </Router>
    }
}
