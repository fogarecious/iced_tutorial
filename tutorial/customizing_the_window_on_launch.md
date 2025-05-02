# Customizing The Window On Launch

By using the [`window`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.window) function of [`Application`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html), we can pass a [window::Settings](https://docs.rs/iced/0.13.1/iced/window/settings/struct.Settings.html) struct to change the properties of the window (such as [position](https://docs.rs/iced/0.13.1/iced/window/settings/struct.Settings.html#structfield.position) and [size](https://docs.rs/iced/0.13.1/iced/window/settings/struct.Settings.html#structfield.size)).

Other useful functions in [`Application`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html) are [`window_size`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.window_size), which is a syntactic sugar for [`window`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.window) that only changes the size of the window, [`centered`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.centered), which centers the window on the screen, and [`position`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.position), which allows us to specify the position of the window.

```rust
use iced::{Point, Size, window};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .window(window::Settings {
            size: Size {
                width: 70.,
                height: 30.,
            },
            position: window::Position::Specific(Point { x: 50., y: 60. }),
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        "Hello".into()
    }
}
```

![Customizing The Window On Launch](./pic/customizing_the_window_on_launch.png)

:arrow_right:  Next: [Changing The Window Dynamically](./changing_the_window_dynamically.md)

:blue_book: Back: [Table of contents](./../README.md)
