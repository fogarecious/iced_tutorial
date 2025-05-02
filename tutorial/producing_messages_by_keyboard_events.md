# Producing Messages By Keyboard Events

This tutorial follows from the [previous tutorial](./producing_messages_by_mouse_events.md).
Instead of capturing [Event::Mouse](https://docs.rs/iced/0.13.1/iced/event/enum.Event.html#variant.Mouse) and [Event::Touch](https://docs.rs/iced/0.13.1/iced/event/enum.Event.html#variant.Touch), we capture [Event::Keyboard](https://docs.rs/iced/0.13.1/iced/event/enum.Event.html#variant.Keyboard) in the [listen_with](https://docs.rs/iced/0.13.1/iced/event/fn.listen_with.html) function.

```rust
use iced::{
    Event, Task,
    event::{self, Status},
    keyboard::{Event::KeyPressed, Key, key::Named},
    widget::text,
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(String),
}

#[derive(Default)]
struct MyApp {
    pressed_key: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                pressed_key: "".into(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(s) => self.pressed_key = s,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        text(self.pressed_key.as_str()).into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, status, _| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Enter),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::KeyPressed("Enter".into())),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::KeyPressed("Space".into())),
            _ => None,
        })
    }
}
```

![Producing messages by keyboard events](./pic/producing_messages_by_keyboard_events.png)

:arrow_right:  Next: [Producing Messages By Timers](./producing_messages_by_timers.md)

:blue_book: Back: [Table of contents](./../README.md)
