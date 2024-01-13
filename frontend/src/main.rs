// src/main.rs

// dependencies
use yew::prelude::*;

// a single app component, renders the text "Welcome to Yew!"
#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1 class="text-3xl">{ "Welcome to Yew!"}</h1>
        </div>
    }
}

// main function, renders the app component to the screen
fn main() {
    yew::Renderer::<App>::new().render();
}
