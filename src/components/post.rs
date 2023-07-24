use crate::components::footer::Footer;
use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use crate::server_functions::posts::Post;
use crate::server_functions::posts::PostContent;
use crate::server_functions::posts::PostMetadata;
use crate::server_functions::posts::PostType;
use leptos::Scope;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use std::collections::HashMap;

#[component]
pub fn Post(cx: Scope, post_type: PostType, post_description: String) -> impl IntoView {
    let posts =
        use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>(cx)
            .expect("unable to find context");
    view! { cx,
        <Body class="bg-[#080A21]"/>
        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
            <div class="max-w-2xl mx-auto text-center mb-10 lg:mb-14">
                <h2 class="text-2xl font-bold md:text-4xl md:leading-tight text-[#F8F9FA]">
                    "Posts"
                </h2>
                <p class="mt-1  text-[#CED4DA]">{post_description}</p>
            </div>
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
                <Transition fallback=move || {
                    view! { cx, <p>"Loading..."</p> }
                }>
                    {move || {
                        posts
                            .read(cx)
                            .map(|posts| match posts {
                                Ok(posts) => {
                                    posts
                                        .get(&post_type)
                                        .expect("Unable to read the right post_type")
                                        .iter()
                                        .map(|post| {
                                            if post_type == PostType::Project {
                                                view! { cx,
                                                    <ProjectPostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        href=post.post_metadata.project_link.clone()
                                                    />
                                                }
                                            } else {
                                                view! { cx,
                                                    <PostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        path=post_type.to_string()
                                                    />
                                                }
                                            }
                                        })
                                        .collect_view(cx)
                                }
                                Err(e) => {
                                    view! { cx,
                                        <pre class="error">"Server Error: " {e.to_string()}</pre>
                                    }
                                        .into_view(cx)
                                }
                            })
                    }}
                </Transition>
            </div>
            <Footer/>
        </div>
    }
}

#[component]
pub fn PostCard(cx: Scope, post_metadata: PostMetadata, path: String) -> impl IntoView {
    view! { cx,
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=format!("/{}/{}", path, post_metadata.create_href())
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date}</h2>
                <p class="mt-5 text-[#CED4DA]">{post_metadata.description}</p>
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="https://github.com/itehax.png"/>
                <h5 class="text-sm text-gray-200">"By Itehax."</h5>
            </div>
        </a>
    }
}

#[component]
pub fn ProjectPostCard(cx: Scope, post_metadata: PostMetadata, href: String) -> impl IntoView {
    view! { cx,
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=href
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date}</h2>
                <p class="mt-5 text-[#CED4DA]">{post_metadata.description}</p>
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="https://github.com/itehax.png"/>
                <h5 class="text-sm text-gray-200">"By Itehax."</h5>
            </div>
        </a>
    }
}
#[component]
pub fn RenderPost(cx: Scope, post_type: PostType) -> impl IntoView {
    let posts =
        use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>(cx)
            .expect("unable to find context");
    let params = use_params_map(cx);
    let post_query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            {move || {
                posts
                    .read(cx)
                    .map(|posts| match posts {
                        Ok(posts) => {
                            let post = posts
                                .get(&post_type)
                                .expect("Unable to read the right post_type")
                                .iter()
                                .find(|&p| p.post_metadata.create_href() == post_query());
                            if let Some(post) = post {
                                view! { cx, <PostLayout content=post.post_content.clone()/> }
                                    .into_view(cx)
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);
                                view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
                            }
                        }
                        Err(e) => {
                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre> }
                                .into_view(cx)
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostLayout(cx: Scope, content: PostContent) -> impl IntoView {
    view! { cx,
        <Link rel="stylesheet" href="/highlighter/styles/github.min.css"/>
        <Script>
            "function loadScript(scriptUrl) {
            const script = document.createElement('script');
            script.src = scriptUrl;
            document.body.appendChild(script);
            
            return new Promise((res, rej) => {
            script.onload = function() {
            res();
            }
            script.onerror = function () {
            rej();
            }
            });
            }
            
            // use
            loadScript('/highlighter/highlight.min.js')
            .then(() => {
            hljs.highlightAll();
            })
            .catch(() => {
            console.error('Script loading failed! Handle this error');
            });"
        </Script>

        <div class="bg-[#080A21]">
            <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
                <div class="max-w-3xl">
                    <div
                        class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none"
                        inner_html=content
                    ></div>
                </div>
            </div>
        </div>
    }
}
