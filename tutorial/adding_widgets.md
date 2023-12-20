# Adding widgets

Use [column!](https://docs.iced.rs/iced/widget/macro.column.html) and [row!](https://docs.iced.rs/iced/widget/macro.row.html) to group multiple widgets such as [text](https://docs.iced.rs/iced/widget/fn.text.html) and [button](https://docs.iced.rs/iced/widget/fn.button.html).

```rust
use iced::{
    widget::{button, column, row, text},
    Sandbox, Settings,
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
            text("Yes or no?"),
            row![
                button("Yes"),
                button("No"),
            ],
        ].into()
    }
}
```

![Adding widgets](./pic/adding_widgets.png)
