use leptos::*;

fn main() {
    
        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root));
    mount_to_body(|| view!{
        <div class="flex flex-col justify-center items-center">
            <div class="flex flex-col justify-center items-center"><img src="/public/digitaler-frieden.jpg" class="w-34 h-34" alt="">
<h1 class="font-bold text-3xl">"Digitaler Frieden"</h1>
            </div>

        </div>
    })
}
