use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn HireMe(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Hire Me"/>
        <Meta name="description" content="Hire Itehax as:"/>
        <Body class="bg-[#080A21]"/>
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            // <!-- Icon Blocks -->
            <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
                <div class="max-w-2xl mx-auto ">
                    // <!-- Grid -->
                    <div class="grid gap-12">
                        <div>
                            <h2 class="text-3xl font-bold lg:text-4xl text-[#F8F9FA]">
                                "Hire me as:"
                            </h2>
                            <p class="mt-3 text-[#CED4DA]">
                                "In case you need a programmer, I am currently available as a freelancer.
                                The languages in which I am specialised are " <b>"Rust"</b> ", "
                                <b>"C"</b> "& " <b>"Modern C++"</b> ", " <b>"Assembly x86"</b>
                                "and " <b>"Javascript"</b>
                                ", although in programming the hardest part is not learning the language itself, so don't hesitate to contact me if you need other languages."
                            </p>
                        </div>
                        <hr class="text-[#CED4DA]"/>
                        <div class="space-y-6 lg:space-y-10">
                            // <!-- Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Backend Developer"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "Backend web development is the part of web development that deals with the server-side of a web application. This includes creating and managing the server-side logic, connecting the application to a database, creating server-side APIs, handling user authentication and authorization, and processing and responding to user requests. "
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            // <!-- Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Reverse Engineer"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "Software reverse engineering is the practice of analyzing a software system, either in whole or in part, to extract design and implementation information. Reverse engineering skills are also used to detect and neutralize viruses and malware, and to protect intellectual property."
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            // <!-- Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Malware Analyst"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "
                                    Malware analysis is the study of the unique features, objectives, sources, and potential effects of harmful software and code, such as spyware, viruses, malvertising, and ransomware. It analyzes malware code to understand how it varies from other kinds."
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Game Hacker"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "Game hacking allows you to add functionality and change how games work."
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "System Programmer"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "Systems programming involves the development of the individual pieces of software that allow the entire system to function as a single unit. Systems programming involves many layers such as the operating system (OS), firmware, and the development environment."
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "WinApi Developer"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "Using the Windows API, you can develop applications that run successfully on all versions of Windows while taking advantage of the features and capabilities unique to each version. (Note that this was formerly called the Win32 API. The name Windows API more accurately reflects its roots in 16-bit Windows and its support on 64-bit Windows.)"
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            // <!-- Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Web Related Developer"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "In case you need, applications such as Discord or Telegram bots, or anything related, please do not hesitate to contact me."
                                </p>
                            </div>
                            // <!-- End Icon Block -->
                            // <!-- Icon Block -->
                            <div class="ml-5 sm:ml-8">
                                <h3 class="text-base sm:text-lg font-semibold text-[#F8F9FA]">
                                    "Private Teacher"
                                </h3>
                                <p class="mt-1 text-[#CED4DA]">
                                    "In case you need a private tutor, on subjects related to university or other topics concerning my specialisations, I am available."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string()/>
        <HomeFooter/>
    }
}
