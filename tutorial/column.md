# Column

[Column](https://docs.rs/iced/latest/iced/widget/struct.Column.html) helps us placing widgets vertically.
It can leave some space between its boundary and its inner content.
It can also add spaces among its inner widgets.
The inner widgets can be aligned left, middle or right.

```rust
use iced::{
    widget::{column, Column},
    Alignment, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
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
                .align_items(Alignment::Center),
            column!["Space between elements", "Space between elements",].spacing(20),
        ]
        .into()
    }
}
```

![Column](./pic/column.png)

:arrow_right:  Next: [Row](./row.md)

:blue_book: Back: [Table of contents](./../README.md)
