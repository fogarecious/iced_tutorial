# First App - Hello World!

We need a struct to implement [Sandbox](https://docs.iced.rs/iced/trait.Sandbox.html), and call its [run](https://docs.iced.rs/iced/trait.Sandbox.html#method.run) method from `main`.
All widgets should be placed inside the [view](https://docs.iced.rs/iced/trait.Sandbox.html#tymethod.view) method.

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

:arrow_right:  Next: [Adding Widgets](./adding_widgets.md)
