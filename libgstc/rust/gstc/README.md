# gstc_rust (Rust)

Rust client library for RidgeRun GstD.

## Build and run

### Meson (project-integrated build)

From repo root:

```bash
meson setup build
meson compile -C build
```

Rust examples are built as Meson executables under `build/examples/libgstc/rust/`.

### Cargo

A `Cargo.toml` is maintained for crate workflows, for example, (`cargo run`, `cargo test`, `cargo fmt`, `cargo clippy`). The crate can be added as a dependency in another Cargo project:

```bash
[dependencies]
gstc_rust = { path = "<path_to_this_crate>" }
```


