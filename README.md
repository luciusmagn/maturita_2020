# magnusi.tech
My website.

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
