# Row

Similar to [Column](https://docs.rs/iced/0.13.1/iced/widget/struct.Column.html), [Row](https://docs.rs/iced/0.13.1/iced/widget/struct.Row.html) helps us placing widgets horizontally.
It can leave some space between its boundary and its inner content.
It can also add spaces among its inner widgets.
The inner widgets can be aligned top, middle or bottom.

```rust
use iced::{
    Alignment, Length,
    widget::{Row, column, row},
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
            Row::with_children(vec![
                "Construct from the with_children function.".into(),
                "another element".into(),
            ]),
            Row::new()
                .push("Construct from the new function and the push method.")
                .push("another element again"),
            row(vec!["Construct from function".into()]),
            row!["Construct from macro"],
            row!["With padding"].padding(20),
            row!["Space between elements", "Space between elements",].spacing(20),
            row!["Different alignment"]
                .height(Length::Fill)
                .align_y(Alignment::Center),
        ]
        .into()
    }
}
```

![Row](./pic/row.png)

:arrow_right: Next: [Space](./space.md)

:blue_book: Back: [Table of contents](./../README.md)
