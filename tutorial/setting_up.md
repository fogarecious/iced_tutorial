# Setting Up

Initialize a [Cargo](https://doc.rust-lang.org/cargo/guide/) project.

```sh
cargo new my_project
```

where `my_project` is the name of the project.

Add [Iced](https://iced.rs/) to the project dependencies.

```sh
cd my_project
cargo add iced
```

You should see the dependency in the end of `Cargo.toml` file.

```toml
[dependencies]
iced = "0.12.1"
```

Note: If you encounter `WGPU Error`, you can disable `wgpu` in `Cargo.toml` file.

```toml
[dependencies]
iced = { version = "0.12.1", default-features = false }
```

:arrow_right:  Next: [First App - Hello World!](./first_app.md)

:blue_book: Back: [Table of contents](./../README.md)
