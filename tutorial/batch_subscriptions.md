# Batch Subscriptions

This tutorial follows from the previous two tutorials ([keyboard events](./producing_messages_by_keyboard_events.md) and [timers](./producing_messages_by_timers.md)).
We combine the two [Subscriptions](https://docs.iced.rs/iced/subscription/struct.Subscription.html) of keyboard events and timers.
This is done by [Subscription::batch](https://docs.iced.rs/iced/subscription/struct.Subscription.html#method.batch) function.

In the following app, press the space key to start or stop the timer.

```rust
use iced::{
    event::Status,
    executor,
    keyboard::KeyCode,
    subscription,
    time::{self, Duration},
    widget::text,
    Application, Command, Event, Settings, Subscription,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    StartOrStop,
    Update,
}

struct MyApp {
    seconds: u32,
    running: bool,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                seconds: 0,
                running: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::StartOrStop => self.running = !self.running,
            MyAppMessage::Update => self.seconds += 1,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let subscr_key = subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Space,
                    ..
                }),
                Status::Ignored,
            ) => Some(MyAppMessage::StartOrStop),
            _ => None,
        });
        
        if self.running {
            Subscription::batch(vec![
                subscr_key,
                time::every(Duration::from_secs(1)).map(|_| MyAppMessage::Update),
            ])
        } else {
            subscr_key
        }
    }
}
```

![Batch subscriptions](./pic/batch_subscriptions.png)

:arrow_right:  Next: [Drawing Shapes](./drawing_shapes.md)

:blue_book: Back: [Table of contents](./../README.md)
