use crate::components::footer::GoBack;
use crate::components::footer::HomeFooter;
use crate::components::graph::GraphView;
use crate::components::seo::PostSeo;
use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use crate::server_functions::posts::Post;
use crate::server_functions::posts::PostContent;
use crate::server_functions::posts::PostMetadata;
use crate::server_functions::posts::PostType;
use crate::server_functions::posts::TocItem;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use std::collections::HashMap;

#[component]
pub fn Post(post_type: PostType, post_description: String) -> impl IntoView {
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");
    view! {
        <Body class="bg-[#0D1117]" />
        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
            <div class="max-w-2xl mx-auto text-center mb-10 lg:mb-14">
                <h2 class="text-2xl font-bold md:text-4xl md:leading-tight text-[#E6EDF3]">
                    "Posts"
                </h2>
                <p class="mt-1  text-[#8B949E]">{post_description}</p>
            </div>
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || {
                        posts
                            .get()
                            .map(|posts| match posts {
                                Ok(posts) => {
                                    posts
                                        .get(&post_type)
                                        .expect("Unable to read the right post_type")
                                        .iter()
                                        .map(|post| {
                                            if post_type == PostType::Project {
                                                view! {
                                                    <LinkPostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        href=post.post_metadata.project_link.clone()
                                                    />
                                                }
                                            } else {
                                                view! {
                                                    <PostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        path=post_type.to_string()
                                                    />
                                                }
                                            }
                                        })
                                        .collect_view()
                                }
                                Err(e) => {

                                    view! {
                                        <pre class="error">"Server Error: " {e.to_string()}</pre>
                                    }
                                        .into_view()
                                }
                            })
                    }}

                </Transition>
            </div>
        </div>
        <GoBack content="Back to Home".to_string() url="".to_string() />
        <HomeFooter />
    }
}

#[component]
pub fn PostCard(post_metadata: PostMetadata, path: String) -> impl IntoView {
    view! {
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=format!("/{}/{}", path, post_metadata.create_href())
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path />
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#E6EDF3]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date.clone()}</h2>
                <p class="mt-5 text-[#8B949E]">{post_metadata.description.clone()}</p>
                {if !post_metadata.tags.is_empty() {
                    Some(
                        view! {
                            <div class="mt-3 flex flex-wrap gap-2">
                                {post_metadata
                                    .tags
                                    .iter()
                                    .map(|tag| {
                                        view! {
                                            <span class="text-xs px-2 py-0.5 rounded-full bg-[#161B22] text-[#58A6FF] border border-[#30363D]">
                                                {tag.clone()}
                                            </span>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        },
                    )
                } else {
                    None
                }}
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="https://github.com/itehax.png" />
                <h5 class="text-sm text-gray-200">"By Itehax."</h5>
            </div>
        </a>
    }
}

#[component]
pub fn LinkPostCard(post_metadata: PostMetadata, href: String) -> impl IntoView {
    view! {
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=href
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path />
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#E6EDF3]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date.clone()}</h2>
                <p class="mt-5 text-[#8B949E]">{post_metadata.description.clone()}</p>
                {if !post_metadata.tags.is_empty() {
                    Some(
                        view! {
                            <div class="mt-3 flex flex-wrap gap-2">
                                {post_metadata
                                    .tags
                                    .iter()
                                    .map(|tag| {
                                        view! {
                                            <span class="text-xs px-2 py-0.5 rounded-full bg-[#161B22] text-[#58A6FF] border border-[#30363D]">
                                                {tag.clone()}
                                            </span>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        },
                    )
                } else {
                    None
                }}
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="https://github.com/itehax.png" />
                <h5 class="text-sm text-gray-200">"By Itehax."</h5>
            </div>
        </a>
    }
}
#[component]
pub fn RenderPost(post_type: PostType) -> impl IntoView {
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");
    let params = use_params_map();
    let post_query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                posts
                    .get()
                    .map(|posts| match posts {
                        Ok(posts) => {
                            let post = posts
                                .get(&post_type)
                                .expect("Unable to read the right post_type")
                                .iter()
                                .find(|&p| p.post_metadata.create_href() == post_query());
                            if let Some(post) = post {
                                let post_href = format!(
                                    "/{}/{}",
                                    post_type,
                                    post.post_metadata.create_href(),
                                );
                                view! {
                                    <PostSeo
                                        post_metadata=post.post_metadata.clone()
                                        post_type=post_type
                                    />
                                    <PostLayout
                                        content=post.post_content.clone()
                                        toc=post.toc.clone()
                                        url=post_type.to_string()
                                        post_href=post_href
                                    />
                                }
                                    .into_view()
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);
                                view! { <ErrorTemplate outside_errors /> }.into_view()
                            }
                        }
                        Err(e) => {
                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                                .into_view()
                        }
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn TableOfContents(toc: Vec<TocItem>) -> impl IntoView {
    let active_id = create_rw_signal(String::new());
    let scroll_progress = create_rw_signal(0.0);
    let toc_items = create_rw_signal(toc);
    let dropdown_open = create_rw_signal(false);

    // Scroll tracking effect
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use std::cell::Cell;
        use std::rc::Rc;

        create_effect(move |_| {
            let ticking = Rc::new(Cell::new(false));

            let closure = {
                let ticking = ticking.clone();
                Closure::wrap(Box::new(move || {
                    if ticking.get() {
                        return;
                    }
                    ticking.set(true);

                    let ticking_inner = ticking.clone();
                    let toc_clone = toc_items.get();
                    let callback = Closure::once(move || {
                        let window = web_sys::window().expect("no window");
                        let document = window.document().expect("no document");

                        // Calculate scroll progress
                        let scroll_y = window.scroll_y().unwrap_or(0.0);
                        let doc_height = document
                            .document_element()
                            .map(|el| el.scroll_height())
                            .unwrap_or(0) as f64;
                        let window_height = window
                            .inner_height()
                            .ok()
                            .and_then(|h| h.as_f64())
                            .unwrap_or(0.0);

                        let max_scroll = doc_height - window_height;
                        let progress = if max_scroll > 0.0 {
                            (scroll_y / max_scroll * 100.0).min(100.0)
                        } else {
                            0.0
                        };
                        scroll_progress.set(progress);

                        // Find active heading
                        let mut current_id = String::new();
                        let mut min_distance = f64::MAX;

                        for item in &toc_clone {
                            if let Some(element) = document.get_element_by_id(&item.id) {
                                let rect = element.get_bounding_client_rect();
                                let top = rect.top() + scroll_y;
                                let distance = (top - scroll_y - 100.0).abs();

                                if top <= scroll_y + 150.0 && distance < min_distance {
                                    min_distance = distance;
                                    current_id = item.id.clone();
                                }
                            }
                        }

                        if !current_id.is_empty() && active_id.get() != current_id {
                            active_id.set(current_id);
                        }

                        ticking_inner.set(false);
                    });

                    let window = web_sys::window().expect("no window");
                    let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                    callback.forget();
                }) as Box<dyn FnMut()>)
            };

            let window = web_sys::window().expect("no window");
            let _ = window.add_event_listener_with_callback(
                "scroll",
                closure.as_ref().unchecked_ref(),
            );

            closure.forget();
        });
    }

    view! {
        <nav class=move || {
            if toc_items.get().is_empty() {
                "hidden"
            } else {
                "fixed top-0 left-0 right-0 z-50 bg-[#0D1117]/98 backdrop-blur-xl border-b border-[#30363D]/60 shadow-lg"
            }
        }>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-14">
                    <button
                        on:click=move |_| dropdown_open.update(|open| *open = !*open)
                        class="flex items-center gap-2 text-sm font-medium text-gray-300 hover:text-white transition-colors duration-150"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h16M4 18h16"
                            ></path>
                        </svg>
                        <span>"Contents"</span>
                        <svg
                            class=move || {
                                if dropdown_open.get() {
                                    "w-4 h-4 transform rotate-180 transition-transform duration-200"
                                } else {
                                    "w-4 h-4 transition-transform duration-200"
                                }
                            }

                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 9l-7 7-7-7"
                            ></path>
                        </svg>
                    </button>

                    <div class="text-xs text-gray-400 font-mono">
                        {move || format!("{}%", scroll_progress.get() as i32)}
                    </div>
                </div>

                // Scroll progress bar
                <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gray-800/50">
                    <div
                        class="h-full bg-[#58A6FF] shadow-sm"
                        style=move || format!("width: {}%", scroll_progress.get())
                    ></div>
                </div>
            </div>

            // Dropdown menu
            <div class=move || {
                if dropdown_open.get() {
                    "absolute top-full left-0 right-0 bg-[#0D1117] backdrop-blur-xl border-b border-[#30363D]/60 shadow-2xl"
                } else {
                    "hidden"
                }
            }>

                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {move || {
                            toc_items
                                .get()
                                .iter()
                                .map(|item| {
                                    let id = item.id.clone();
                                    let id_for_class = id.clone();
                                    let id_for_h3 = id.clone();
                                    let text = item.text.clone();
                                    view! {
                                        <a
                                            href=format!("#{}", id)
                                            on:click=move |_| dropdown_open.set(false)
                                            class=move || {
                                                let base = "group block p-3 rounded-xl border transition-all duration-300 hover:border-transparent hover:shadow-black/[.4]";
                                                if active_id.get() == id_for_class {
                                                    format!("{} border-gray-600 bg-gray-800/40", base)
                                                } else {
                                                    format!("{} border-gray-700", base)
                                                }
                                            }
                                        >

                                            <h3 class=move || {
                                                if active_id.get() == id_for_h3 {
                                                    "text-sm font-semibold text-[#E6EDF3]"
                                                } else {
                                                    "text-sm font-semibold text-gray-300 group-hover:text-[#E6EDF3]"
                                                }
                                            }>

                                                {text}
                                            </h3>
                                        </a>
                                    }
                                })
                                .collect_view()
                        }}

                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn PostLayout(
    content: PostContent,
    toc: Vec<TocItem>,
    url: String,
    #[prop(optional)] post_href: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-[#0D1117] min-h-screen w-full overflow-x-hidden">
            <TableOfContents toc=toc />
            <div class="max-w-3xl px-4 pt-24 pb-20 sm:px-6 lg:px-8 mx-auto">
                <div
                    class="prose prose-blog mx-auto md:prose-lg leading-relaxed prose-pre:m-0 prose-pre:rounded-none break-words"
                    inner_html=content
                ></div>
                {post_href
                    .map(|href| {
                        view! { <GraphView filter_post=href /> }
                    })}

            </div>
            <GoBack content="Back to Posts".to_string() url=url />
            <HomeFooter />
        </div>
    }
}
