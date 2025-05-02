# Width And Height

Most widgets have the `width` and `height` methods to control their sizes.
The methods accept a parameter [Length](https://docs.rs/iced/0.13.1/iced/enum.Length.html).

- [Shrink](https://docs.rs/iced/0.13.1/iced/enum.Length.html#variant.Shrink): occupy the least space.
- [Fill](https://docs.rs/iced/0.13.1/iced/enum.Length.html#variant.Fill): occupy all the rest of space.
- [FillPortion](https://docs.rs/iced/0.13.1/iced/enum.Length.html#variant.FillPortion): occupy the space relative to other widgets with FillPortion.
- [Fixed](https://docs.rs/iced/0.13.1/iced/enum.Length.html#variant.Fixed): occupy a fixed space.

```rust
use iced::{
    Length,
    widget::{button, column, row},
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
            button("Shrink").width(Length::Shrink),
            button("Fill").width(Length::Fill),
            row![
                button("FillPortion 2").width(Length::FillPortion(2)),
                button("FillPortion 1").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            button("Fixed").width(Length::Fixed(100.)),
            button("Fill (height)").height(Length::Fill),
        ]
        .spacing(10)
        .into()
    }
}
```

![Width And Height](./pic/width_and_height.png)

:arrow_right: Next: [Column](./column.md)

:blue_book: Back: [Table of contents](./../README.md)
