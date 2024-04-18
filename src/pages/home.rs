use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::router::Route;


#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="flex flex-col items-center h-full space-y-24 px-12 mt-20 mb-20">
            <div class="flex flex-col items-center space-y-4">
                <img src="public/digitaler-frieden_logo.jpg" alt="logo digitaler frieden" class="min-w-56 w-96"/>
                <h3 class="text-3xl font-bold text-center ">{"Digitale Spuren entfernen per Knopfdruck"}</h3>
                <p class="text-center">{"Mit uns finden Sie Ihre digitalen Spuren und können diese entfernen."}</p>
            </div>
           <div class="bg-primary 
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
                flex justify-center items-center"
            >
                <Link<Route> to={Route::SignUp}>
                    {"Weiter"}
                </Link<Route>>
            </div>
            <div class="flex flex-col items-center space-y-4">
                <p class="text-center italic">{"Mit der weiteren Nutzung stimmst du den folgenden Bedingungen zu:"}</p>
                <div class="text-center flex flex-col items-cneter space-y-1">
                    <a class="text-primary italic" href="#">{"Allgemeine Geschäftsbedingungen"}</a>
                    <a class="text-primary italic" href="#">{"Datenschutzerklärung"}</a>
                </div>
            </div>
        </div>
   }
}
