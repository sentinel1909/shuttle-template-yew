# Yew and Axum - with Shuttle

## Overview
This is a template that allows you to start an app with the [Yew](https://yew.rs) web framework for Rust, and host it on Shuttle. It is relatively unopinionated and ready for you to add what you need. The Yew frontend web files are contained in the `/frontend` folder and are built with the [trunk](https://trunkrs.dev) deployment and packaging tool. The Yew frontend is served up from an Axum web server, contained in the `/server` folder.  The Axum server has two routes:

- / : which serves up the Yew website
- /health_check : which returns a 200 OK response with no body

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






