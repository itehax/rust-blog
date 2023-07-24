use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="mt-auto w-full max-w-[85rem] py-10 px-4 sm:px-6 lg:px-8 mx-auto">
            <div class="text-center">
                <div class="mt-3">
                    <p class="text-[#CED4DA]">
                        "Made with ❤️ using "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://www.rust-lang.org/"
                        >
                            "Rust"
                        </a> ", "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://www.leptos.dev/"
                        >
                            "Leptos "
                        </a> "& "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://preline.co/"
                        >
                            "Preline"
                        </a> "."
                    </p>
                    <p class="text-[#CED4DA]">"© Itehax. 2023 "</p>
                </div>
            </div>
        </footer>
    }
}
