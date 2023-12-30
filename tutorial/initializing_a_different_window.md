# Initializing A Different Window

We can use [window::Settings](https://docs.iced.rs/iced/window/settings/struct.Settings.html) to change the properties of the window (such as [position](https://docs.iced.rs/iced/window/settings/struct.Settings.html#structfield.position) and [size](https://docs.iced.rs/iced/window/settings/struct.Settings.html#structfield.size)) when we call [run](https://docs.iced.rs/iced/trait.Sandbox.html#method.run) of a [Sandbox](https://docs.iced.rs/iced/trait.Sandbox.html) or [Application](https://docs.iced.rs/iced/application/trait.Application.html).
Developers might be interested in reading the document of [window::Settings](https://docs.iced.rs/iced/window/settings/struct.Settings.html) for other properties.

```rust
use iced::{window, Sandbox, Settings};

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: (70, 20),
            position: window::Position::Specific(50, 60),
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

    fn view(&self) -> iced::Element<'_, Self::Message> {
        "Hello".into()
    }
}
```

![Initializing a different window](./pic/initializing_a_different_window.png)

:arrow_right:  Next: [Mouse Events](./mouse_events.md)
