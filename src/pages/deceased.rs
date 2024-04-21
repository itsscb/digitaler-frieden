use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn Deceased() -> Html {
    html! {
        <div class="px-6 mt-20">
            <h3
                class="text-primary
                hover:bg-text-dark
                hover:text-white 
                hover:-translate-y-1 
                hover:cursor-pointer
                transition duration-150 
                font-bold text-xl 
                max-w-xl 
                rounded-md 
                text-black
                w-full
                absolute
                top-4
                left-4
            "
            >
                <Link<Route> to={Route::Home}>
                    <div class="flex items-center">
                        <svg
                            class="w-8 "
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M0 0h24v24H0z" fill="none" />
                            <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z" />
                        </svg>
                        { "Zurück" }
                    </div>
                </Link<Route>>
            </h3>
            <img
                src="public/digitaler-frieden_logo.jpg"
                alt="logo digitaler frieden"
                class="absolute top-4 right-4 md:-w-56 md:w-56 md:h-auto w-0 h-0"
            />
            <div class="-height mt-12 flex flex-col items-center">
                <h3 class="text-3xl font-bold mb-6">{ "Verstorbene Person" }</h3>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Vorname" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2]  active:bg-[#33d9b2] border-b text-center focus-within:text-black w-full h-10 rounded-md"
                        type="text"
                    />
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Nachname" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                        type="text"
                    />
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <div class="flex justify-between space-x-4">
                        <div class="w-full">
                            <p class="text-start text-md mb-1">{ "Straße" }</p>
                            <input
                                class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                                type="text"
                            />
                        </div>
                        <div class="w-24">
                            <p class="text-start text-md mb-1 w-16">{ "Nr." }</p>
                            <input
                                class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                                type="text"
                            />
                        </div>
                    </div>
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Postleitzahl" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                        type="text"
                    />
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Ort" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                        type="text"
                    />
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Geburtsort" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md"
                        type="text"
                    />
                </div>
                <div class="group max-w-xl w-full mb-4">
                    <p class="text-start text-md mb-1">{ "Geburtstag" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md cx-date-picker"
                        type="date"
                    />
                </div>
                <div class="group max-w-xl w-full mb-5">
                    <p class="text-start text-md mb-1">{ "Todestag" }</p>
                    <input
                        class="duration-700 font-bold text-md transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-b text-center  focus-within:text-black w-full h-10 rounded-md cx-date-picker"
                        type="date"
                    />
                </div>
                <Link<Route>
                    to={Route::Clues}
                    classes={yew::classes!(
    "bg-primary",
                   "hover:bg-primary-dark",
                "hover:text-white",
                "hover:-translate-y-1",
                "hover:cursor-pointer",
                   "transition", "duration-150",
                   "font-bold", "text-xl",
                   "max-w-sm",
                   "rounded-md",
                   "text-black",
                   "text-center",
                   "w-full",
                   "h-16",
                   "mt-8", "flex", "justify-center", "items-center",
                               )}
                >
                    { "Weiter" }
                </Link<Route>>
            </div>
        </div>
    }
}
