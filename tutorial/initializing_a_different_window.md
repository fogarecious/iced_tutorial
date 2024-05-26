# Initializing A Different Window

We can use [window::Settings](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html) to change the properties of the window (such as [position](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html#structfield.position) and [size](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html#structfield.size)) when we call [run](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.run) of a [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html) or [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).
Developers might be interested in reading the document of [window::Settings](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html) for other properties.

```rust
use iced::{window, Point, Sandbox, Settings, Size};

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 70.,
                height: 30.,
            },
            position: window::Position::Specific(Point { x: 50., y: 60. }),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
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

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello".into()
    }
}
```

![Initializing a different window](./pic/initializing_a_different_window.png)

:arrow_right:  Next: [Changing The Window Dynamically](./changing_the_window_dynamically.md)

:blue_book: Back: [Table of contents](./../README.md)
