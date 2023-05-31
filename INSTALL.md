# Installation of RustQuant

If you haven't already, you obviously need to install Rust.
See here: https://www.rust-lang.org/tools/install

Create a new Rust project:

```bash
cargo new my_project && cd my_project
```

In your Rust project's root directory, simply run: 

```bash
cargo add RustQuant
```

This will add the latest version to your project.

If you require a specific version, add the following to your Cargo.toml file:

```toml
[dependencies]
RustQuant = "*"
```

replacing `"*"` with the version number you require, such as `"0.0.17"`.

