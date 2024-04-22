use yew::{classes, function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn Relationship() -> Html {
    let classes = classes!("fade-in");

    html! {
        <div class="flex flex-col justify-center items-center h-full space-y-16 px-8 m-0">
            <section id="navigation" class="mb-2 md:mb-32">
                <div class="flex justify-between">
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
                absolute
                text-center
                top-4
                left-4
            "
                    >
                        <Link<Route> to={Route::Clues}>
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
                        class="absolute top-4 right-4 md:min-w-56 md:w-56 w-0"
                    />
                </div>
            </section>
            <section id="content" class={classes}>
                <div class="flex flex-col items-center ">
                    <h3 class="text-3xl font-bold mb-6 text-center">
                        { "In welcher Beziehung stehst du zu der verstorbenen Person?" }
                    </h3>
                    <div class="flex flex-col items-start min-w-72 md:w-auto mb-32">
                        <div class="flex items-center mb-4">
                            <input
                                id="relationship-widow"
                                type="radio"
                                value=""
                                name="relationship"
                                class="w-4 h-4 accent-primary"
                            />
                            <label for="relationship-widow" class="ms-2 ">
                                { "Ich bin die Witwe / der Witwer der verstorbenen Person" }
                            </label>
                        </div>
                        <div class="flex items-center mb-4">
                            <input
                                id="relationship-soleheir"
                                type="radio"
                                value=""
                                name="relationship"
                                class="w-4 h-4 text-primary accent-primary"
                            />
                            <label for="relationship-soleheir" class="ms-2">
                                <p>
                                    { "Ich bin notarielle(r) " }
                                    <u>
                                        <strong>{ "Alleinerbin / Alleinerbe" }</strong>
                                    </u>
                                </p>
                            </label>
                        </div>
                        <div class="flex items-center mb-4">
                            <input
                                id="relationship-child"
                                type="radio"
                                value=""
                                name="relationship"
                                class="w-4 h-4 accent-primary"
                            />
                            <label for="relationship-child" class="ms-2">
                                { "Ich bin ein Kind der verstorbenen Person" }
                            </label>
                        </div>
                    </div>
                    <div class="flex justify-center max-w-xl w-full mb-6">
                        <Link<Route>
                            to={Route::Relationship}
                            classes={classes!("bg-primary",
                "hover:bg-primary-dark",
                "hover:text-white",
                "hover:-translate-y-1",
                "hover:cursor-pointer",
                "transition",
                "duration-150",
                "font-bold",
                "text-xl",
                "max-w-xl",
                "rounded-md",
                "text-black",
                "text-center",
                "w-full",
                "min-h-16",
                "h-16",
                "flex",
                "justify-center",
                "items-center",
                "mb-12",
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
