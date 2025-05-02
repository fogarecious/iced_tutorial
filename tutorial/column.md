# Column

[Column](https://docs.rs/iced/0.13.1/iced/widget/struct.Column.html) helps us placing widgets vertically.
It can leave some space between its boundary and its inner content.
It can also add spaces among its inner widgets.
The inner widgets can be aligned left, middle or right.

```rust
use iced::{
    Alignment, Length,
    widget::{Column, column},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            Column::with_children(vec![
                "Construct from the with_children function".into(),
                "another element".into(),
            ]),
            Column::new()
                .push("Construct from the new function and the push method")
                .push("another element again"),
            column(vec!["Construct from function".into()]),
            column!["Construct from macro"],
            column!["With padding"].padding(20),
            column!["Different alignment"]
                .width(Length::Fill)
                .align_x(Alignment::Center),
            column!["Space between elements", "Space between elements",].spacing(20),
        ]
        .into()
    }
}
```

![Column](./pic/column.png)

:arrow_right: Next: [Row](./row.md)

:blue_book: Back: [Table of contents](./../README.md)
