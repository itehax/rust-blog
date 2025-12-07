use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="Itehax About" />
        <Meta name="description" content="About Edoardo D'Errico." />
        <Body class="bg-[#080A21]" />
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                    <h1>"Manifesto"</h1>
                    <h2>"In Praise of the Ad Hoc"</h2>
                    <p>
                        "Escape the fucking Gaussian. Understand. Destroy. Transform.
                        
                        The real pleasure of life isn’t in the middle of that curve.
                        Most people think it is — music, entertainment, the whole modern world runs on that illusion.
                        
                        Do you really want to settle for what they feed you?
                        It’s time to understand that what is, is for a reason,
                        and that reason might be you.
                        
                        Find and create what speaks only to you.
                        Just for you.
                        Ad hoc. "
                    </p>
                    <h2>"Welcome to my world, you are now ready to be initiated"</h2>

                    <p>
                        "A 20-year-old guy who casually discovered during the lockdown that he had an inordinate passion for "
                        <strong>"computer science"</strong>", "<strong>"mathematics"</strong>
                        " ... and everything "<strong>"interesting"</strong>" to him (tautology)."
                    </p>
                    <p>
                        "I am currently attending university for computer science and mathematics (both).
                        
                        In my spare time, I enjoy reading, watching films and learning new things."
                    </p>
                    <p>
                        "In short, " <strong>"the one and only things I like"</strong>
                        ", as expected of a hacker,  are those that can stimulate my "
                        <strong>"mind"</strong> "."
                    </p>
                    <h2>"Welcome to my world, you are now ready to be initiated"</h2>

                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string() url="".to_string() />
        <HomeFooter />
    }
}
