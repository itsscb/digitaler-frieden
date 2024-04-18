use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn Verify() -> Html {
    html! {
       <div class="px-6 mt-4 md:mt-24">
           <div class="flex justify-between">
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
                absolute
                top-4
                left-4
            ">
                <Link<Route> to={Route::SignUp}>
                    <div class="flex items-center"><svg class="w-8 " xmlns="http://www.w3.org/2000/svg" height="currentHeight" viewBox="0 0 24 24" width="currentWidth" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/></svg>{"Zurück"}</div>
                </Link<Route>>
            </h3>
            <img src="public/digitaler-frieden_logo.jpg" alt="logo digitaler frieden" class="absolute top-4 right-4 md:min-w-56 md:w-56 w-0"/>
           </div>
            <div class="mt-24 md:mt-32 flex flex-col items-center">
            <h3 class="text-3xl font-bold mb-6">{"Verifizieren"}</h3>
            <div class="flex flex-col items-start">
               <p class="mb-3">{"Bitte verifiziere deine E-Mail Adresse."}</p> 
               <p class="mb-12">{"Wir haben dir eine E-Mail geschickt."}</p> 
               <p class="mb-8">{"Dann geht es hier weiter."}</p> 
           </div>
            <div class="flex justify-center max-w-xl w-full mb-6">
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
                    flex justify-center items-center mb-16"
                >
                    <Link<Route> to={Route::Register}>
                        {"Weiter"}
                    </Link<Route>>
                </div>
            </div>
            <div class="flex flex-col items-start">
                <p class="mb-1">{"Noch keine E-Mail erhalten?"}</p>
                <p class="mb-4">{"Spam Ordner schon überprüft?"}</p>
                <div class="text-primary hover:text-white 
                    hover:-translate-y-1 
                    hover:cursor-pointer
                    transition duration-150 
                    ">
                        <Link<Route> to={Route::SignUp}>
                            {"Erneut senden"}
                         </Link<Route>>
                 </div>
            </div>
         </div>
    </div>
    }
}
