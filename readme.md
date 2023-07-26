# My personal blog written using rust!

This is my personal website i built using [leptos](https://leptos.dev) and [preline](https://preline.co)
---

[Preview](https://itehax.com)

## How to run

To run the project first of all you need to have `cargo-leptos` installed on your machine

`cargo install --locked cargo-leptos`

Then run

`npm run watch` (This is a script which basically run the CLI tool to scan your template files for classes and build your CSS.)

and

`cargo leptos watch`

in this directory.

Open browser on [http://localhost:3000/](http://localhost:3000/)

You can begin editing your app at `src/app.rs`.

## Installing Tailwind

You can install Tailwind using `npm`:

```bash
npm install -D tailwindcss
```
Also [Tailwind Typography](https://tailwindcss.com/docs/typography-plugin#installation) and [Preline](https://www.preline.co/docs/index.html)


## Setting up with VS Code and Additional Tools

If you're using VS Code, add the following to your `settings.json`

```json
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false,
```

Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

    Install "VS Browser" extension, a browser at the right window.
    Allow vscode Ports forward: 3000, 3001.

## Notes about Tooling

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site.


# Using the docker image

```bash
docker build ./ -t itehax-website
```

After it's builded.

```bash
docker run -p 3000:3000 itehax-website
```

