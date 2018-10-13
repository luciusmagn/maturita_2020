# magnusi.tech
My website. Currently, it uses two templates - `template.html` and `raw_template.html` and fills them with HTML
generated from the Markdown files in the `web` directory. Files with the `.md` suffix are implanted into `template.html`
and files with the `.rmd` suffix are implanted into `raw_template.html`. Stylus stylesheets are compiled on-the-fly
into CSS and cached in memory.

### Installation
The project requires the nightly version of Rust. See <https://rustup.rs> for how to install Rust.
By default, magnusi.tech runs on port 80, to change it, edit this line in main.rs:

```rust
app.run("0.0.0.0:80");
```

The project can be ran using this command:
```bash
cargo run --release
```
