# First App - Hello World!

We need a struct to implement [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html), and call its [run](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.run) method from `main`.
All widgets should be placed inside the [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) method.

```rust
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        "Hello World!".into()
    }
}
```

![First app](./pic/first_app.png)

:arrow_right:  Next: [Explanation of Sandbox Trait](./explanation_of_sandbox_trait.md)

:blue_book: Back: [Table of contents](./../README.md)
