use crate::pages::{Clues, Deceased, Documents, Home, Register, Relationship, SignUp, Verify};

use yew::{function_component, html, Html};
use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/signup")]
    SignUp,
    #[at("/verify")]
    Verify,
    #[at("/register")]
    Register,
    #[at("/deceased")]
    Deceased,
    #[at("/clues")]
    Clues,
    #[at("/relationship")]
    Relationship,
    #[at("/documents")]
    Documents,
}

fn route(routes: Route) -> Html {
    match routes {
        Route::Documents => {
            html! { <Documents /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::SignUp => {
            html! { <SignUp /> }
        }
        Route::Verify => {
            html! { <Verify /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
        Route::Deceased => {
            html! { <Deceased /> }
        }
        Route::Clues => {
            html! { <Clues /> }
        }
        Route::Relationship => {
            html! { <Relationship /> }
        }
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
