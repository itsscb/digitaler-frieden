use yew::{classes, function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn SignUp() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center h-full space-y-16 px-8 m-0">
            <section id="navigation" class="mb-12">
                <h3
                    class="text-primary
                hover:bg-text-dark 
                hover:text-white 
                hover:-translate-y-1 
                hover:cursor-pointer
                transition duration-150 
                font-bold text-xl 
                rounded-md 
                text-black
                top-4
                left-4
                absolute
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
                    class="top-4 right-4 absolute md:min-w-56 md:w-56 w-0"
                />
            </section>
            <section id="content" class="w-full fade-in">
                <div class="min-height mt-64 md:flex md:flex-col md:items-center">
                    <h3 class="text-3xl font-bold mb-6">{ "Jetzt registrieren" }</h3>
                    <p>{ "Gib deine E-Mail Adresse ein." }</p>
                    <div class="group max-w-xl w-full mb-4">
                        <input
                            class="duration-700 font-bold text-lg transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-2 text-center text-primary focus-within:text-black w-full h-16 mt-4 rounded-md"
                            type="text"
                        />
                    </div>
                    <div class="flex justify-between max-w-xl w-full">
                        <div
                            class="text-primary hover:text-white
                hover:-translate-y-1 
                hover:cursor-pointer
                transition duration-150 
                "
                        >
                            <Link<Route> to={Route::SignUp}>{ "Stattdessen anmelden" }</Link<Route>>
                        </div>
                        <Link<Route>
                            to={Route::Verify}
                            classes={classes!(
    "bg-primary",
                "hover:bg-primary-dark",
                   "hover:text-white",
                   "hover:-translate-y-1",
                   "hover:cursor-pointer",
                   "transition", "duration-150",
                "font-bold", "text-xl",
                "max-w-36",
                "rounded-md",
                "text-black",
                "text-center",
                "w-full",
                "min-h-12",
                "h-12",
                "flex", "justify-center", "items-center","mb-12"
                                )}
                        >
                            { "Weiter" }
                        </Link<Route>>
                    </div>
                </div>
            </section>
        </div>
    }
}
