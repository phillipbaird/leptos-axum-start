# Leptos-Axum-Start

A template for creating a hydrating web application using [Leptos](https://github.com/leptos-rs/leptos), [Axum](https://github.com/tokio-rs/axum) and [Tailwind CSS](https://tailwindcss.com).

## Prerequisites

- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) - version 0.1.5 or higher
- [Tailwind CSS](https://tailwindcss.com) or [Tailwind CSS binary](https://github.com/tailwindlabs/tailwindcss/releases)
- On Linux I'm using [Mold](https://github.com/rui314/mold) for linking.  If this is not your jam then delete the `./cargo/config.toml` file to revert to the standard linker.
- This template assumes a nightly Rust toolchain.

## Warnings

- Leptos is under rapid development so this project currently takes leptos dependencies from git rather than crates.io.

## Usage

### Development

In two terminals...
```bash
./tw.sh watch
```
```bash
cargo leptos watch
```

### Release

```bash
./tw.sh build
cargo leptos build
```

# References

This template is based on [cargo-leptos example project](https://github.com/leptos-rs/cargo-leptos/tree/main/examples/project) & [leptos hackernews-axum](https://github.com/leptos-rs/leptos/tree/main/examples/hackernews_axum) examples.
