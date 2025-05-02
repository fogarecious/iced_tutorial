# Adding Widgets

Use [column!](https://docs.rs/iced/0.13.1/iced/widget/macro.column.html) and [row!](https://docs.rs/iced/0.13.1/iced/widget/macro.row.html) to group multiple widgets such as [text](https://docs.rs/iced/0.13.1/iced/widget/fn.text.html) and [button](https://docs.rs/iced/0.13.1/iced/widget/fn.button.html).

```rust
use iced::widget::{button, column, row, text};

fn main() -> iced::Result {
    iced::run("MyApp", MyApp::update, MyApp::view)
}

#[derive(Default)]
struct MyApp;

#[derive(Debug, Clone)]
enum Message {}

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![text("Yes or no?"), row![button("Yes"), button("No"),],].into()
    }
}
```

![Adding widgets](./pic/adding_widgets.png)

:arrow_right: Next: [Widgets](./widgets.md)

:blue_book: Back: [Table of contents](./../README.md)
