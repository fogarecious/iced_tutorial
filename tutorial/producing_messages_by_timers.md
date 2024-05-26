# Producing Messages By Timers

To use build-in timers, we need to enable one of the following features: [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio), [async-std](https://docs.rs/crate/iced/0.12.1/features#async-std), or [smol](https://docs.rs/crate/iced/0.12.1/features#smol).
In this tutorial, we use [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio) feature.
The dependencies of `Cargo.toml` should look like this:

```toml
[dependencies]
iced = { version = "0.12.1", features = ["tokio"] }
```

We use [time::every](https://docs.rs/iced/0.12.1/iced/time/fn.every.html) function to obtain [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html)\<[Instant](https://docs.rs/iced/0.12.1/iced/time/struct.Instant.html)> struct.
Then we map the struct to [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html)\<`MyAppMessage`> by [Subscription::map](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html#method.map) method.
The result will be returned in the [subscription](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#method.subscription) method of [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).
The corresponding `MyAppMessage` will be received in the [update](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#tymethod.update) method.

```rust
use iced::{
    executor,
    time::{self, Duration},
    widget::text,
    Application, Command, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    Update,
}

struct MyApp {
    seconds: u32,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { seconds: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::Update => self.seconds += 1,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_secs(1)).map(|_| MyAppMessage::Update)
    }
}
```

![Producing messages by timers](./pic/producing_messages_by_timers.png)

:arrow_right:  Next: [Batch Subscriptions](./batch_subscriptions.md)

:blue_book: Back: [Table of contents](./../README.md)
