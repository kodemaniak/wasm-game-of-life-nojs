Assumption: rust is present

Install:
https://rustwasm.github.io/wasm-pack/installer/

Install htpp server for development:

```
cargo install basic-http-server
```

Create project:
```
cargo new --lib wasm-game-of-life-nojs
cd wasm-game-of-life-nojs

```

Setup cargo for wasm-bindgen (https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html):

`Cargo.toml`:

```toml
[package]
name = "wasm-game-of-life-nojs"
version = "0.1.0"
authors = ["Me <me@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.63"


[dependencies.web-sys]
version = "0.3.4"
features = [
  'Document',
  'Element',
  'HtmlElement',
  'Node',
  'Window',
]
```

Change name, version, authors accordingly. This adds web-sys as dependency on top of default wasm-bindgen (https://rustwasm.github.io/docs/wasm-bindgen/examples/dom.html).

Change `src/lib.rs` to:

```rust
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
```

Create `index.html` with following contents:

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <style>
    body {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
    }
  </style>
  <title>Hello from Rust!</title>
</head>

<body>
  <noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
  <script type="module">

    init();
  </script>
</body>

</html>
```

Now we can check if everything works.

Build the wasm file using wasm-pack:

```bash
wasm-pack build --target=web
```

Run the development server:

```bash
basic-http-server
```

Visit http://localhost:4000/. You should see "Hello from Rust!".

Add `/pkg` to `.gitgnore`:
```
/target
Cargo.lock
/pkg
```

