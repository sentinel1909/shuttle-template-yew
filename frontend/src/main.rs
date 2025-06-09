// src/main.rs

// dependencies
use yew::prelude::*;

// a single app component, renders the text "Welcome to Yew!"
#[function_component]
fn App() -> Html {
    html! {
        <>
            <header>
                <h1>{ "Hello Yew!" }</h1>
            </header>
            <main>
                <section>
                    <h2>{ "Template last update: 2025-06-09" }</h2>
                    <article>
                        <p>{ "Site compiled to WebAssembly, using the Rust frontend framework called Yew" }</p>
                    </article>
                </section>
            </main>
            <footer>
            <a href="https://yew.rs" target="_blank" rel="noreferrer">{ "Learn more about Yew" }</a>
            </footer>
        </>
    }
}

// main function, renders the app component to the screen
fn main() {
    // initialize console error panic hook
    console_error_panic_hook::set_once();

    // initialize wasm logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    // render the main app
    yew::Renderer::<App>::new().render();
}
