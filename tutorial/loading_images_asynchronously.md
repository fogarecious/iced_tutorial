# Loading Images Asynchronously

In the previous tutorials, we introduced how to use [`Tasks`](./tasks.md) to execute asynchronous operations, and how to display [images](./images.md).
This tutorial combines both and demonstrates how to load an image asynchronously.

Iced offers three mutually exclusive features for asynchronous operations. They need to be enabled before usage:
- [tokio](https://docs.rs/crate/iced/0.13.1/features#tokio)
- [async-std](https://docs.rs/crate/iced/0.13.1/features#async-std)
- [smol](https://docs.rs/crate/iced/0.13.1/features#smol).

Depending on your choice, your must also add the dependency crate to your `Cargo.toml` file:
- [tokio](https://crates.io/crates/tokio)
- [async-std](https://crates.io/crates/async-std)
- [smol](https://crates.io/crates/smol)

The [tokio](https://crates.io/crates/tokio) crate is very popular, and we will use it as an example.
First, we enable [tokio](https://docs.rs/crate/iced/0.13.1/features#tokio) feature and add [tokio](https://crates.io/crates/tokio) crate with `fs` and `io-util` features.

The dependencies of `Cargo.toml` should look like this:

```toml
[dependencies]
iced = { version = "0.13.1", features = ["tokio"] }
tokio = { version = "1.44.2", features = ["fs", "io-util"] }
```

Our app will have three states: *start*, *loading* and *loaded*.
We use two fields to encode the three states.

```rust
struct MyApp {
    image_handle: Option<Handle>,
    show_container: bool,
}
```

When the state is

* **start**: `image_handle` is `None` and `show_container` is false;
* **loading**: `image_handle` is `None` and `show_container` is true;
* **loaded**: `image_handle` is `Some(...)` and `show_container` is true.

The app begins in the *start* state.

The app always shows a button that is for loading the `ferris.png` image.
In the *start* state, the app shows no additional widget.
In the *loading* state, the app shows the text `Loading...`.
And in the *loaded* state, the app shows the image.

```rust
fn view(&self) -> iced::Element<Self::Message> {
    column![
        button("Load").on_press(MyMessage::Load),
        if self.show_container {
            match &self.image_handle {
                Some(h) => container(image(h.clone())),
                None => container("Loading..."),
            }
        } else {
            container("")
        },
    ]
    .padding(20)
    .into()
}
```

We have two messages for the app:

```rust
#[derive(Debug, Clone)]
enum MyMessage {
    Load,
    Loaded(Vec<u8>),
}
```

When the button is pressed, the app triggers a `Load` message to load the image.
And when the image is loaded, the app triggers a `Loaded(...)` message.

The image will be loaded asynchronously.

```rust
fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
    match message {
        MyMessage::Load => {
            self.show_container = true;
            return Command::perform(
                async {
                    let mut file = File::open("../tutorial/pic/ferris.png").await.unwrap();
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).await.unwrap();
                    buffer
                },
                MyMessage::Loaded,
            );
        }
        MyMessage::Loaded(data) => self.image_handle = Some(Handle::from_memory(data)),
    }
    Command::none()
}
```

The full code is as follows:

```rust
use iced::{
    Task,
    widget::{Image, button, column, container, image::Handle},
};
use tokio::{fs::File, io::AsyncReadExt};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Loaded(Vec<u8>),
}

#[derive(Default)]
struct MyApp {
    image_handle: Option<Handle>,
    show_container: bool,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                image_handle: None,
                show_container: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.show_container = true;
                return Task::perform(
                    async {
                        let mut file = File::open("../tutorial/pic/ferris.png").await.unwrap();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).await.unwrap();
                        buffer
                    },
                    Message::Loaded,
                );
            }
            Message::Loaded(data) => self.image_handle = Some(Handle::from_bytes(data)),
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Load").on_press(Message::Load),
            if self.show_container {
                match &self.image_handle {
                    Some(h) => container(Image::new(h.clone())),
                    None => container("Loading..."),
                }
            } else {
                container("")
            },
        ]
        .padding(20)
        .into()
    }
}
```

State of *start*:

![Loading Images Asynchronously 1](./pic/loading_images_asynchronously_1.png)

State of *loading*:

![Loading Images Asynchronously 2](./pic/loading_images_asynchronously_2.png)

State of *loaded*:

![Loading Images Asynchronously 3](./pic/loading_images_asynchronously_3.png)

:arrow_right:  Next: [The end](./the_end.md)

:blue_book: Back: [Table of contents](./../README.md)
