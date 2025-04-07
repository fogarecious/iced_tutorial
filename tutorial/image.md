# Image

The [Image](https://docs.rs/iced/0.13.1/iced/widget/image/struct.Image.html) widget is able to display an image.
It has two methods of construction: the `image` function and the `Image::new` constructor.
We can set how to fit the image content into the widget bounds and the size of the image.

To use the widget, we have to enable the [image](https://docs.rs/crate/iced/0.12.1/features#image) feature.
The `Cargo.toml` dependencies should look like this:

```toml
[dependencies]
iced = { version = "0.12.1", features = ["image"] }
```

The image path is resolved relative to the current working directory. Since we run the application with cargo, the root directory is the project root. In our example below, the image `ferris.png` is in the `tutorial/pic` directory.

```rust
use iced::{
    ContentFit,
    widget::{Image, column, image, text},
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
            text("Construct from struct"),
            Image::new("tutorial/pic/ferris.png").width(100).height(100),
            text("Construct from function"),
            image("tutorial/pic/ferris.png").width(100).height(100),
            text("Different content fit"),
            image("tutorial/pic/ferris.png")
                .content_fit(ContentFit::Cover)
                .width(100)
                .height(100),
        ]
        .into()
    }
}
```

![Image](./pic/image.png)

:arrow_right: Next: [Svg](./svg.md)

:blue_book: Back: [Table of contents](./../README.md)
