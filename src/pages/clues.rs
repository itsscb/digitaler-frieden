use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn Clues() -> Html {
    html! {
        <div class="px-6 mt-4 md:mt-24">
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
                <Link<Route> to={Route::SignUp}>
                    <div class="flex items-center">
                        <svg
                            class="w-8 "
                            xmlns="http://www.w3.org/2000/svg"
                            height="currentHeight"
                            viewBox="0 0 24 24"
                            width="currentWidth"
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
            <div class="mt-16 md:mt-32 flex flex-col items-center justify-between">
                <div class="flex flex-col items-start">
                    <h3 class="text-3xl font-bold mb-4">{ "Spurhinweise hinzufügen" }</h3>
                    <p class="mb-6">
                        { "Füge alle dir bekannten E-Mail Adressen und Telefonnummern hinzu." }
                    </p>
                    <div
                        class="flex flex-col justify-between items-center w-full mb-10 space-y-4 md:space-y-4"
                    >
                        <div class="flex justify-between w-full relative">
                            <h3 class="font-bold text-2xl ">{ "E-Mail Adressen" }</h3>
                            <svg
                                class="text-primary w-10 absolute right-4"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                        </div>
                        <div class="flex justify-start items-center space-x-4 w-full">
                            <svg
                                class="w-10 text-danger"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M7 11v2h10v-2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                            <p class="text-md">{ "max.mustermann@example.com" }</p>
                        </div>
                        <div class="flex justify-start items-center w-full space-x-4">
                            <svg
                                class="w-10 text-danger"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M7 11v2h10v-2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                            <p class="text-md">{ "jon.doe@blub.com" }</p>
                        </div>
                    </div>
                    <div
                        class="flex flex-col justify-center items-center w-full mb-10 space-y-4 md:space-y-4"
                    >
                        <div class="flex justify-between w-full relative">
                            <h3 class="font-bold text-2xl ">{ "Telefonnummern" }</h3>
                            <svg
                                class="text-primary w-10 absolute right-4"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                        </div>
                        <div class="flex justify-start items-center space-x-4 w-full">
                            <svg
                                class="w-10 text-danger"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M7 11v2h10v-2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                            <p class="text-md">{ "+49 1234 567 890" }</p>
                        </div>
                        <div class="flex justify-start items-center w-full space-x-4">
                            <svg
                                class="w-10 text-danger"
                                xmlns="http://www.w3.org/2000/svg"
                                height="currentHeight"
                                viewBox="0 0 24 24"
                                width="currentWidth"
                                fill="currentColor"
                            >
                                <path d="M0 0h24v24H0z" fill="none" />
                                <path
                                    d="M7 11v2h10v-2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"
                                />
                            </svg>
                            <p class="text-md">{ "+49 9876 543 210" }</p>
                        </div>
                    </div>
                    <div class="flex justify-center max-w-xl w-full mb-6">
                        <div
                            class="bg-primary
                    hover:bg-primary-dark 
                    hover:text-white 
                    hover:-translate-y-1 
                    hover:cursor-pointer
                    transition duration-150 
                    font-bold text-xl 
                    max-w-xl 
                    rounded-md 
                    text-black
                    text-center
                    w-full 
                    min-h-16 
                    h-16
                    flex justify-center items-center mb-16"
                        >
                            <Link<Route> to={Route::Register}>{ "Weiter" }</Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
