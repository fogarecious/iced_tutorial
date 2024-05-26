# Producing Messages By Keyboard Events

This tutorial follows from the [previous tutorial](./producing_messages_by_mouse_events.md).
Instead of capturing [Event::Mouse](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Mouse) and [Event::Touch](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Touch), we capture [Event::Keyboard](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Keyboard) in the [listen_with](https://docs.rs/iced/0.12.1/iced/event/fn.listen_with.html) function.

```rust
use iced::{
    event::{self, Status},
    executor,
    keyboard::{key::Named, Event::KeyPressed, Key},
    widget::text,
    Application, Event, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    KeyPressed(String),
}

struct MyApp {
    pressed_key: String,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                pressed_key: "".into(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::KeyPressed(s) => self.pressed_key = s,
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        text(self.pressed_key.as_str()).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen_with(|event, status| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Enter),
                    ..
                }),
                Status::Ignored,
            ) => Some(MyAppMessage::KeyPressed("Enter".into())),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(MyAppMessage::KeyPressed("Space".into())),
            _ => None,
        })
    }
}
```

![Producing messages by keyboard events](./pic/producing_messages_by_keyboard_events.png)

:arrow_right:  Next: [Producing Messages By Timers](./producing_messages_by_timers.md)

:blue_book: Back: [Table of contents](./../README.md)
