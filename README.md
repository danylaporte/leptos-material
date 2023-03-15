# Material & leptos

This crate reimplement material components in leptos.

## What is Material Component Web
[Material](https://github.com/material-components/material-components-web) is a collection of component to improve speed and uniformity when developping a web application.

This crate intend wrap all components.

## What is leptos

[Leptos](https://github.com/leptos-rs/leptos) is a framework like react / solidjs for building spa application with rust.


## Installation

```bash
# Add the wasm target
rustup target add wasm32-unknown-unknown

# install trunk via cargo
cargo install --locked trunk
```

## Running / testing

```bash
# should open a browser on http://localhost:8080 with a demonstration of the components.
trunk serve --open
```

## License

The MIT License.
