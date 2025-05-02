# ProgressBar

The [ProgressBar](https://docs.rs/iced/0.13.1/iced/widget/progress_bar/struct.ProgressBar.html) widget represents a value in a given range.
It has two methods of construction: the `progress_bar` function and the `ProgressBar::new` constructor.

```rust
use iced::widget::{ProgressBar, column, progress_bar, text};

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
            text("Construct from struct"),
            ProgressBar::new(0.0..=100.0, 50.),
            text("Construct from function"),
            progress_bar(0.0..=100.0, 30.),
        ]
        .into()
    }
}
```

![ProgressBar](./pic/progressbar.png)

:arrow_right: Next: [Tooltip](./tooltip.md)

:blue_book: Back: [Table of contents](./../README.md)
