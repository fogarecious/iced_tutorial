# Container

[Container](https://docs.rs/iced/0.13.1/iced/widget/struct.Container.html) is another way to help us laying out widgets, especially when we need to algin a widget center both horizontally and vertically.
[Container](https://docs.rs/iced/0.13.1/iced/widget/struct.Container.html) accepts any widget including [Column](https://docs.rs/iced/0.13.1/iced/widget/struct.Column.html) and [Row](https://docs.rs/iced/0.13.1/iced/widget/struct.Row.html).

```rust
use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{Container, column, container},
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
            Container::new("Construct from struct"),
            container("Construct from function"),
            container("With padding").padding(20),
            container("Different alignment")
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            container("Different alignment for vertical axis")
                .height(Length::Fill)
                .align_y(Vertical::Center),
            container("Center")
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        ]
        .into()
    }
}
```

![Container](./pic/container.png)

:arrow_right: Next: [Scrollable](./scrollable.md)

:blue_book: Back: [Table of contents](./../README.md)
