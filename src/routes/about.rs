use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax About"/>
        <Meta name="description" content="About me."/>
        <Body class="bg-[#080A21]"/>
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                    <h2>"Welcome to my blog, I am itehax, a developer from Italy."</h2>
                    <p>
                        "By the time you read this post probably, I will be attending "
                        <strong>"computer science "</strong> "college."
                    </p>
                    <p>
                        "I have several passions, although we can now call them " <i>"habits"</i>
                        ", including:"
                    </p>
                    <ol>
                        <li>
                            <p>
                                "Computer science,from the more practical point of view such as programming,but also from a more theoretical point of view 💻."
                            </p>
                        </li>
                        <li>
                            <p>"Boxe,I like to train every day 🥊."</p>
                        </li>
                        <li>
                            <p>
                                "Reading, either books on personal growth, or books on classical literature and philosophy 📚."
                            </p>
                        </li>
                    </ol>
                    <img src="/images/about_images/Hajime.jpg" alt=""/>
                    <h3>"What should be expected from this blog"</h3>
                    <ol>
                        <li>
                            <p>
                                "Programming post, I have several projects in mind and I will comment/write down what I have learnt useful."
                            </p>
                        </li>
                        <li>
                            <p>
                                "Posts on reverse engineering, malware analysis and game hacking, regarding these topics,I will always post several projects and open a youtube channel, especially on the use of rust in this field 🦀."
                            </p>
                        </li>
                        <li>
                            <p>"Finally, reviews of books read by me."</p>
                        </li>
                    </ol>
                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string()/>
        <HomeFooter/>
    }
}
