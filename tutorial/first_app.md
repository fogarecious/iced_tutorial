# First App - Hello World!

> **Info:** The Sandbox trait has been dropped on 0.13.1.

The most basic app should have a struct with implementations of an update and view functions. Then, the [run](https://docs.rs/iced/0.13.1/iced/fn.run.html) function should be called passing the application title, and the update and view functions. Notice that we define the Message as a simple type just to make sure the signature of update and view are correct. Later we'll see more uses for it.

```rust
fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

type Message = ();

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        "Hello World!".into()
    }
}
```

![First app](./pic/first_app.png)

:arrow_right: Next: [Explanation of App Structure](./explanation_of_app_structure.md)

:blue_book: Back: [Table of contents](./../README.md)
