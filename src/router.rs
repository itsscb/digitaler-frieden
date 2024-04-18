use crate::pages::{Clues, Deceased, Home, Register, SignUp, Verify};

use yew_router::prelude::*;
use yew::{function_component, html, Html};
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/signup")]
    SignUp,
    #[at("/verify")]
    Verify,
    #[at("/deceased")]
    Deceased,
    #[at("/clues")]
    Clues,
    #[at("/register")]
    Register,
}

fn route(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        },
        Route::SignUp => {
            html! { <SignUp /> }
        },
        Route::Verify => {
            html! { <Verify /> }
        },
        Route::Clues => {
            html! { <Clues /> }
        },
        Route::Deceased => {
            html! { <Deceased /> }
        },
        Route::Register => {
            html! { <Register /> }
        },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route} />
        </BrowserRouter>
    }
}
           
