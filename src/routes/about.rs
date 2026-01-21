use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn About() -> impl IntoView {
    let title = "About | Cybersecurity & CTF Writeups by Edoardo D'Errico";
    let description = "Cybersecurity writeups and CTF solutions by Edoardo D'Errico. Technical posts on cryptography, exploit development, and security research.";
    let url = "https://edoardoderrico.com/about";

    view! {
        <Title text=title />
        <Meta name="description" content=description />

        // Open Graph / Facebook
        <Meta property="og:url" content=url />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=title />
        <Meta property="og:description" content=description />
        // <Meta property="og:image" content={image}/>

        // Twitter
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta property="twitter:domain" content="edoardoderrico.com" />
        <Meta property="twitter:url" content=url />
        <Meta name="twitter:title" content=title />
        <Meta name="twitter:description" content=description />
        // <Meta name="twitter:image" content={image}/>

        <Body class="bg-[#080A21]" />
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                    <h1>"About (whoami)"</h1>
                    <h2>"Welcome to my world, you are now ready to be initiated."</h2>

                    <p>
                        "A 21-year-old who during the 2020 lockdown discovered an inordinate passion for "
                        <strong>"computer science"</strong>", "<strong>"mathematics"</strong>
                        " and... everything "<strong>"interesting"</strong>" to him (tautology)."
                    </p>
                    <hr />
                    <p>
                        "I am currently finishing a computer science degree and working full time in the cybersecurity/intelligence sector."
                    </p>
                    <p>
                        "
                        Despite trying to learn and "<strong>"create new stuff"</strong>
                        ", which actually takes up a good chunk of my mental energy, I love:"
                    </p>
                    <ul>
                        <li>
                            "Playing Capture the Flag (international competitions between hackers)."
                        </li>
                        <li>
                            "Reading, in particular hard and for-the-state-of-brain-of-most-people-in-2020s boring stuff."
                        </li>
                        <li>"Martial arts, i have practiced judo, boxing and Muay Thai."</li>
                        <li>
                            "Nature and discovering new places, especially abandoned and aesthetic ones ^_^"
                        </li>
                    </ul>
                    <h3>"Ok, but what to expect from this blog?"</h3>
                    <p>
                        "
                        Dense stuff that requires "<strong>"silence and concentration."</strong>"
                        We will delve deep into each topic to understand how it really works under the hood. Topics can range from mathematics and computer science to electronics, chemistry, literature, politics and reflections on life.
                        "
                    </p>

                    <p>"Just two constants:"</p>
                    <ul>
                        <li>
                            "Understanding the "<strong>"mental process"</strong>
                            " that led the creators of a certain concept to discover it."
                        </li>
                        <li>
                            <strong>"No rules"</strong>
                            ", just stuff i find interesting."
                        </li>
                    </ul>
                    <p>"That's all, hope you can enjoy :)"</p>
                    <hr />
                    <blockquote>

                        <p>
                            "Note: I am very open to know like-minded people, if you think you are, feel free to "
                            <a href="mailto:edoardoderrico@outlook.com">"contact me"</a>" !"
                        </p>
                    </blockquote>
                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string() url="".to_string() />
        <HomeFooter />
    }
}
