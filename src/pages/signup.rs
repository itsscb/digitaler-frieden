use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::Route;

#[function_component]
pub fn SignUp() -> Html {
    html! {
       <div class="h-screen min-height px-12 mt-6">
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
            ">
                <Link<Route> to={Route::Home}>
                    <div class="flex items-center"><svg class="w-8 " xmlns="http://www.w3.org/2000/svg" height="currentHeight" viewBox="0 0 24 24" width="currentWidth" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/></svg>{"Zurück"}</div>
                </Link<Route>>
            </h3>
        <div class="flex flex-col items-center h-screen min-height space-y-24 px-12 mt-20">
           </div>
        </div>
    }
}
