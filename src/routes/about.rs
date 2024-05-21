use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Itehax About"/>
        <Meta name="description" content="About Edoardo D'Errico."/>
        <Body class="bg-[#080A21]"/>
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                    <h2 id="currently-in-the-first-year-of-computer-science-">
                        "Currently in the first year of computer science."
                    </h2>
                    <p>
                        "I have several " <strong>"passions"</strong>
                        ", although we can now call them habits."
                    </p>
                    <ul>
                        <li>
                            <p>
                                "Computer science in general, in particular the "
                                <strong>"security "</strong>
                                "aspect of everything related to computing."
                            </p>
                        </li>
                        <li>
                            <p>
                                "Reading, in particular literature and philosophical essays. My favourite author is "
                                <a href="https://en.wikipedia.org/wiki/Infinite_Jest">
                                    "David Foster Wallace"
                                </a> ", if you like him, surely we can be good friends."
                            </p>
                        </li>
                    </ul>
                    <p>
                        "In short, " <strong>"the one and only things I like"</strong>
                        ", as expected of a hacker,  are those that can stimulate my "
                        <strong>"mind"</strong> "."
                    </p>

                    <img src="/images/about_images/me.jpeg" alt=""/>
                    <h3>"What should be expected from this blog"</h3>
                    <ol>
                        <li>
                            <p>
                                "Programming post, I have several projects in mind and I will comment/write down what I have learnt useful."
                            </p>
                        </li>
                        <li>
                            <p>
                                "CTF writeup and in general useful stuff for hackers, regarding these topics,I will always post several projects and open a youtube channel, especially on the use of rust in this field 🦀."
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
