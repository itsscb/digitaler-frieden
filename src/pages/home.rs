use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="flex flex-col items-center h-full space-y-24 px-12 mt-20 mb-20">
            <div class="flex flex-col items-center space-y-4">
                <svg class="text-white w-96" fill="currentColor" version="1.1" viewBox="0 0 3e3 2e3" xmlns="http://www.w3.org/2000/svg">
                    <style type="text/css"></style>
                    <path class="logo-fadein-right" d="m1199.2 993.15 308.76-519.02-0.28-0.29-184.43-153.02h-393.98l282.78 474.48-43.11 72.38-370.14-621.09h551.29l254.22 211.26-35.97 60.53h471.2l-44.25 74.23h-471.2l-238.53 400.54zm444.65-746.56-62.38 104.92 57.67 47.96 46.96-78.8h471.2l44.11-74.08z"></path>
                    <path class="logo-fadein-left" d="m2072.1 1753.3h14.43v-107.56l84.7 107.56h11.82v-131.18h-14.45v105.13l-82.64-105.13h-13.86zm-200.57-117.69h80.02v-13.49h-94.82v131.18h95.75v-13.49h-80.95v-45.92h71.59v-13.49h-71.59zm-207.72-13.4h-45.54v131.18h45.54c41.11 0 69.66-28.83 69.66-65.81 0-37.11-28.55-65.37-69.66-65.37zm0 117.33h-30.83v-103.78h30.83c33.12 0 54.24 22.98 54.24 52.1 0 29.27-21.12 51.68-54.24 51.68zm-246.06-103.93h80.02v-13.49h-94.83v131.18h95.77v-13.49h-80.96v-45.92h71.58v-13.49h-71.58zm-160.07-13.49h14.81v131.18h-14.81zm-160 77.6c20.56-3.85 35.54-16.56 35.54-38.4 0-23.7-18.7-39.11-47.39-39.11h-56.39v131.18h14.7v-50.96h37.54l37.97 50.96h18.27zm-53.53-10.71v-53.24h40.54c21.13 0 33.55 9.85 33.55 25.98 0 17.13-14.13 27.26-33.69 27.26zm-141.13-6.36h-71.03v-46.85h79.45v-13.68h-94.26v131.18h14.81v-57.16h71.03zm1644.4-314.39h3c45.82 0 73.8-25.27 73.8-59.52 0-34.4-27.98-58.38-73.8-58.38h-108.63v220.83h28.26l0.29-102.92h44.11l75.51 102.63h36.54zm-77.08-24.69v-69.23h80.08c28.55 0 44.11 12.42 44.11 34.12 0 21.27-15.56 35.12-44.11 35.12h-80.08zm-277.07 103.06v-75.37h119.76v-23.98h-119.76v-72.94h139.18v-24.27h-167.73v220.83h170.72v-24.27zm-258.94 0v-196.56h-28.41v220.83h158.31v-24.27zm-230.1-196.56h-32.97l-101.21 220.83h31.83l25.98-55.53h120.76l25.27 55.53h31.55zm-65.09 141.46 48.82-112.91 49.82 112.91zm-309.62-141.46v24.27h83.65v196.56h28.55v-196.56h84.08v-24.27zm-128.33 0v220.83h28.41v-220.83zm-108.48 103.92h-125.05v23.84l94.64 0.43c-2.28 10.56-6 20.13-11.13 29.26-3 5.57-7.28 10.56-11.7 15.27l-0.57 0.57c-15.27 14.85-36.54 24.55-62.24 26.55-3 0.43-6.28 0.43-9.56 0.43h-3.71c-35.83-1.14-64.38-16.99-79.65-42.25h-0.43c-7.85-13.56-12.56-29.55-12.56-47.82 0-52.82 39.54-89.64 96.35-89.64 37.83 0 68.38 15.99 84.08 42.82l32.12-0.43c-18.56-41.11-62.09-67.09-116.19-67.09-72.37 0-125.9 46.96-125.9 114.34 0 66.8 51.82 113.63 122.91 114.62h9.56c3.71-0.28 7.71-0.71 11.42-0.71l6.28-1c11.85-2 23.55-5.28 33.55-9.56 13.27-5.71 25.12-13.28 34.83-22.55 0.29-0.43 0.71-0.71 0.71-1.14 4.85-4.71 9.56-9.85 13.27-15.56l0.86-1c11.42-16.56 17.7-36.11 18.7-58.1-0.02-6.57-0.45-8.57-0.59-11.28zm-391.55-103.92v220.83h28.55v-220.83zm-238.53 0h-89.07v220.83h89.07c72.8 0 126.61-42.97 126.61-110.63 0-67.38-53.81-110.2-126.61-110.2zm0 196.13h-60.38v-171.58h60.38c58.81 0 97.07 34.12 97.07 85.65 0 51.82-38.26 85.93-97.07 85.93z" />
                </svg>
                <h3 class="text-3xl font-bold text-center ">
                    { "Digitale Spuren entfernen per Knopfdruck" }
                </h3>
                <p class="text-center">
                    { "Mit uns finden Sie Ihre digitalen Spuren und können diese entfernen." }
                </p>
            </div>
            <Link<Route>
                to={Route::SignUp}
                classes={classes!(
                 "bg-primary",
                "hover:bg-primary-dark",
                "hover:text-white",
                "hover:-translate-y-1",
                "hover:cursor-pointer",
                "transition",
                "duration-150",
                "font-bold",
                "text-xl",
                "max-w-xl",
                "rounded-md",
                "text-black",
                "text-center",
                "w-full",
                "min-h-16",
                "h-16",
                "flex",
                "justify-center",
                "items-center",
                        )}
            >
                { "Weiter" }
            </Link<Route>>
            <div class="flex flex-col items-center space-y-4">
                <p class="text-center italic">
                    { "Mit der weiteren Nutzung stimmst du den folgenden Bedingungen zu:" }
                </p>
                <div class="text-center flex flex-col items-cneter space-y-1">
                    <a class="text-primary italic" href="#">
                        { "Allgemeine Geschäftsbedingungen" }
                    </a>
                    <a class="text-primary italic" href="#">{ "Datenschutzerklärung" }</a>
                </div>
            </div>
        </div>
    }
}
