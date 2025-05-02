# Text

The [Text](https://docs.rs/iced/0.13.1/iced/widget/type.Text.html) widget displays texts.
It has three methods of constructions: raw `&str`, the `text` function, and the `Text::new` constructor.
You can change the font, set different sizes, set its width and height, and align vertically or horizontally.

```rust
use iced::{
    Font, Length,
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{Text, column, text, text::Shaping},
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
            "Construct from &str",
            text("Construct from function"),
            Text::new("Construct from struct"),
            text("Different font").font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            text("Larger text").size(24),
            text("Special character 😊").shaping(Shaping::Advanced),
            text("Center")
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            text("Vertical center")
                .height(Length::Fill)
                .align_y(Vertical::Center),
        ]
        .into()
    }
}
```

![Text](./pic/text.png)

:arrow_right: Next: [Button](./button.md)

:blue_book: Back: [Table of contents](./../README.md)
