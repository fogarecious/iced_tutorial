# Closing The Window On Demand

This tutorial follows the [previous tutorial](./changing_the_window_dynamically.md).
We use the [close](https://docs.rs/iced/0.13.1/iced/window/fn.close.html) function in [window](https://docs.rs/iced/0.13.1/iced/window/index.html) module to close the window.
This is also done by returning the [Task](https://docs.rs/iced/0.13.1/iced/struct.Task.html) obtained by the [close](https://docs.rs/iced/0.13.1/iced/window/fn.close.html) function.

Similar to the [resize](https://docs.rs/iced/0.13.1/iced/window/fn.resize.html) function, the [close](https://docs.rs/iced/0.13.1/iced/window/fn.close.html) function needs an ID of the window we are going to close. To do that, the window module has a [`get_latest`](https://docs.rs/iced/0.13.1/iced/window/fn.get_latest.html) function. This function also returns a task that needs to be resolved to the Id of the latest window. In our case, since we have a single window, it will always return the Id of the main window. Once the `close` function is called, the window will be closed, and, because it's the only window, the application will exit.

```rust
use iced::{
    Task,
    widget::{button, row},
    window,
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    CloseWindow,
}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn close(id: window::Id) -> Task<Message> {
        window::close(id)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CloseWindow => window::get_latest().and_then(Self::close),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        row![button("Close window").on_press(Message::CloseWindow),].into()
    }
}
```

![Closing the window on demand](./pic/closing_the_window_on_demand.png)

:arrow_right:  Next: [On Pressed/Released Of Some Widgets](./on_pressed_released_of_some_widgets.md)

:blue_book: Back: [Table of contents](./../README.md)
