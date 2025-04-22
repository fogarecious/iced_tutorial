# Executing Custom Tasks

[Tasks](https://docs.rs/iced/0.12.1/iced/struct.Command.html) can run concurrently, and this allows us to perform asynchronous operations, like fetching data.

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

For this example using tokio, we will use the [tokio::time::sleep](https://docs.rs/tokio/latest/tokio/time/fn.sleep.html) function. So the `time` feature is enabled.

We use [Task::perform](https://docs.rs/iced/0.13.1/iced/struct.Task.html#method.perform) to execute an asynchronous function. The first parameter of [Task::perform](https://docs.rs/iced/0.13.1/iced/struct.Task.html#method.perform) is an asynchronous function, and the second parameter is a function that returns `Message`. The `Message` will be produced once the asynchronous function is done.

In the following code, we use a simple asynchronous function [tokio::time::sleep](https://docs.rs/tokio/latest/tokio/time/fn.sleep.html). When the asynchronous function finished, we will receive `Message::Done`. Don't worry too much about the details of the asynchronous function.

```rust
use iced::{
    Task,
    widget::{button, column, text},
};
use std::time::Duration;

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Execute,
    Done,
}

#[derive(Default)]
struct MyApp {
    state: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: "Ready".into(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Execute => {
                self.state = "Executing".into();
                return Task::perform(tokio::time::sleep(Duration::from_secs(1)), |_| {
                    Message::Done
                });
            }
            Message::Done => self.state = "Done".into(),
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Execute").on_press(Message::Execute),
            text(self.state.as_str()),
        ]
        .into()
    }
}
```

![Executing custom tasks](./pic/executing_custom_tasks.png)

:arrow_right:  Next: [Windows](./windows.md)

:blue_book: Back: [Table of contents](./../README.md)
