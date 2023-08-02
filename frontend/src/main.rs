// src/main.rs

// dependencies
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to Yew!"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
