# Producing Messages By Timers

For this tutorial, we will use timers. Since timers are an async feature, we need to enable it before usage.

Iced offers three mutually exclusive features for that. They need to be enabled before usage:
- [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio)
- [async-std](https://docs.rs/crate/iced/0.12.1/features#async-std)
- [smol](https://docs.rs/crate/iced/0.12.1/features#smol).

Depending on your choice, your must also add the dependency crate to your `Cargo.toml` file:
- [tokio](https://crates.io/crates/tokio)
- [async-std](https://crates.io/crates/async-std)
- [smol](https://crates.io/crates/smol)

The [tokio](https://crates.io/crates/tokio) crate is very popular, and we will use it as an example.
First, we enable [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio) feature and add [tokio](https://crates.io/crates/tokio) crate.

The dependencies of `Cargo.toml` should look like this:

```toml
[dependencies]
iced = { version = "0.13.1", features = ["tokio"] }
tokio = { version = "1.44.2", features = ["time"] }
```

We use [time::every](https://docs.rs/iced/0.13.1/iced/time/fn.every.html) function to obtain [Subscription](https://docs.rs/iced/0.13.1/iced/struct.Subscription.html)\<[Instant](https://docs.rs/iced/0.13.1/iced/time/struct.Instant.html)> struct.

Then we map the struct to [Subscription](https://docs.rs/iced/0.13.1/iced/struct.Subscription.html)\<`Message`> using [Subscription::map](https://docs.rs/iced/0.13.1/iced/struct.Subscription.html#method.map) method, and call our message to increment the counter.

```rust
use iced::{
    Task,
    time::{self, Duration},
    widget::text,
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Update,
}

#[derive(Default)]
struct MyApp {
    seconds: u32,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (Self { seconds: 0 }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update => self.seconds += 1,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }
}
```

![Producing messages by timers](./pic/producing_messages_by_timers.png)

:arrow_right:  Next: [Batch Subscriptions](./batch_subscriptions.md)

:blue_book: Back: [Table of contents](./../README.md)
