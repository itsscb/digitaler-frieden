use crate::pages::{Home, SignUp};

use yew_router::prelude::*;
use yew::{function_component, html, Html};
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/signup")]
    SignUp,
}

fn route(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        },
        Route::SignUp => {
            html! { <SignUp /> }
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
           
