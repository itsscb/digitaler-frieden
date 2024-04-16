use yew::prelude::*;

#[function_component]
fn App() -> Html {

    html! {
        <div class="flex flex-col justify-center items-center h-screen space-y-6">
            <img src="public/digitaler-frieden_logo.jpg" alt="logo digitaler frieden" class="w-96"/>
            <h3 class="text-2xl font-bold text-center uppercase ">{"coming soon"}</h3>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
