use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn SignUp() -> Html {
    html! {
       <div class="h-full  px-6 mt-4">
                 <h3 class="text-primary 
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
                top-4
                left-4
                absolute
            ">
                <Link<Route> to={Route::Home}>
                    <div class="flex items-center"><svg class="w-8 " xmlns="http://www.w3.org/2000/svg" height="currentHeight" viewBox="0 0 24 24" width="currentWidth" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/></svg>{"Zurück"}</div>
                </Link<Route>>
            </h3>
                            <img src="public/digitaler-frieden_logo.jpg" alt="logo digitaler frieden" class="top-4 right-4 absolute md:min-w-56 md:w-56 w-0"/>
            <div class="min-height mt-64 md:flex md:flex-col md:items-center">
            <h3 class="text-3xl font-bold mb-6">{"Jetzt registrieren"}</h3>
               <p>{"Gib deine E-Mail Adresse ein."}</p> 
                <div class="group max-w-xl w-full mb-4">
                    <input class="duration-700 font-bold text-lg transition group-hover:cursor-pointer bg-transparent border-white hover:border-[#33d9b2] hover:curser-pointer focus-within:bg-[#33d9b2] active:bg-[#33d9b2] border-2 text-center text-primary focus-within:text-black w-full h-16 mt-4 rounded-md" type="text"/>
                </div>
                    <div class="flex justify-between max-w-xl w-full">
                        <div class="text-primary hover:text-white 
                hover:-translate-y-1 
                hover:cursor-pointer
                transition duration-150 
                ">
                            <Link<Route> to={Route::SignUp}>
                                {"Stattdessen anmelden"}
                             </Link<Route>>
                         </div>
                        <div class="bg-primary 
                hover:bg-primary-dark 
                hover:text-white 
                hover:-translate-y-1 
                hover:cursor-pointer
                transition duration-150 
                font-bold text-xl 
                max-w-36 
                rounded-md 
                text-black
                text-center
                w-full 
                min-h-12 
                h-12
                flex justify-center items-center"
            >
                <Link<Route> to={Route::Verify}>
                    {"Weiter"}
                </Link<Route>>
            </div>
                    </div>
            </div>
        </div>
    }
}
