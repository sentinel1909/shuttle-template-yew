# Yew and Axum Starter Template (hosted with Shuttle)

## Overview

This is a template that allows you to start an app with the [Yew](https://yew.rs) web framework for Rust, and host it on [Shuttle](https://shuttle.rs). It is minimalist and ready for you to add what you need. The Yew frontend web files are contained in the `/frontend` folder and are built with the [trunk](https://trunkrs.dev) deployment and packaging tool. The Yew frontend is served up from an Axum web server, contained in the `/server` folder. The Rocket server has two routes:

- `/` : which serves up the Yew website
- `/health_check` : returns an empty body and 200 OK message

This app template uses client side rendering, which is the default in Yew. In the client side rendering scheme, when the user visits a website, the server sends a skeleton HTML file without any content and a WebAssembly bundle, to the browser. Everything is then rendered client side by the WebAssembly bundle.

Tailwind is incorporated for styles. The trunk build tool incorporates the Tailwind CLI, so very little additional work is needed. The template provides a pre-made `tailwind.config.js` in the root of the `frontend` folder, which can serve as a starting point. Simply style your Yew components with Tailwind classes and trunk will take care of the rest.

## Setup

After cloning this repo and making it your own, you'll need to get the following basic tooling installed.

- install Rust
- install WebAssembly target
- install Trunk
- install the Just command runner

### Install Rust

Head to the official Rust installation instructions [here](https://www.rust-lang.org/tools/install).

### Install WebAssembly Target

From your command line, type:

```bash
rustup target add wasm32-unknown-unknown
```

### Install Trunk

Install the trunk build tool by typing the following in your command line:

```bash
cargo install --locked trunk
```

### Install Just

Install the Just command runner tool by typing the following in your command line:

```bash
cargo install just
```

## Usage

If you're using this template as a starter and want to get more familiar with the Yew framework, I recommend working through the tutorials on the Yew website. There are also many examples hosted in their [GitHub repo](https://github.com/yewstack/yew).

This template leverages the [Just](https://github.com/casey/just) command runner to help build the frontend for deployment. In the root directory, the configuration file for just (justfile) has the following starter recipes:

`just dev` : starts a hot loading development server (with trunk)

`just build-release` : uses trunk to build the frontend for deployment, default output location is /server/dist (per Trunk.toml)

`just shuttle-run` : serves the built frontend via Axum using a local shuttle environment

`just shuttle-deploy` : deploys the built frontend, with its Axum server, to the shuttle cloud environment for deployment

## Notes

Per the [discussion](https://docs.shuttle.rs/resources/shuttle-static-folder) in the Shuttle docs regarding the shuttle-static-folder resource, because this template is built as a cargo workspace, the `/dist` folder containing the built Yew frontend must live in the root of the workspace. The build artifacts are output to `/server/dist` (via the Trunk.toml configuration file located in the `/frontend` directory) by default, to support running locally with `cargo shuttle run`. The `just shuttle-deploy` recipe copies the build artifacts to `/dist` in the root of the crate workspace.

## Building from Here

This template has just enough to get you going. You'll likely want to add:

- yew-router (for routing between pages of your app)
- [telemetry](https://docs.shuttle.rs/introduction/telemetry) to the server portion of the app
- wasm-logger crate for logging from WebAssembly to the console
- web-sys crate (bindings for all Web APIs)
