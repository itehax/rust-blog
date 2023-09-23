use leptos::*;

#[component]
pub fn HomeFooter(cx: Scope) -> impl IntoView {
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

                    <a class="flex justify-center items-center space-x-2 text-blue-500 hover:text-blue-400 text-base mt-4" href="./feed">
                        <span class="text-[#CED4DA]">"Rss"</span>
                        <svg
                            viewBox="0 0 48 48"
                            class="h-6 w-6"
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="currentColor"
                        >
                            <path d="M0 0h48v48H0z" fill="none"></path>
                            <circle cx="12.36" cy="35.64" r="4.36"></circle>
                            <path d="M8 8.89v5.66c14.06 0 25.46 11.4 25.46 25.46h5.66C39.11 22.82 25.18 8.89 8 8.89zM8 20.2v5.66c7.81 0 14.14 6.34 14.14 14.14h5.66c0-10.93-8.87-19.8-19.8-19.8z"></path>
                        </svg>
                    </a>

                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn GoBack(cx: Scope, content: String) -> impl IntoView {
    view! { cx,
        <footer class="mt-auto w-full max-w-[85rem] px-4 sm:px-6 lg:px-8 mx-auto">
            <div class="text-center">
                <a
                    class="text-base w-full sm:w-auto inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold text-blue-500 hover:text-blue-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all  px-4 ring-offset-slate-900"
                    href="./"
                >
                    <svg class="w-2.5 h-2.5" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path
                            d="M11.2792 1.64001L5.63273 7.28646C5.43747 7.48172 5.43747 7.79831 5.63273 7.99357L11.2792 13.64"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                        ></path>
                    </svg>
                    {content}
                </a>
            </div>
        </footer>
    }
}
