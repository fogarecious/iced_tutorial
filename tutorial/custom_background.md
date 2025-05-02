# Custom Background

We can also draw an image on a [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html).

Similar to the [Image](https://docs.rs/iced/0.13.1/iced/widget/image/struct.Image.html) widget, we have to enable the [image](https://docs.rs/crate/iced/latest/features#image) feature.

```toml
[dependencies]
iced = { version = "0.13.1", features = ["image", "advanced"] }
```

Assume we have an image, named `ferris.png`,  in the Cargo root directory.
We load this image when we initialize our widget.

```rust
struct MyWidgetWithImage {
    handle: Handle,
}

impl MyWidgetWithImage {
    fn new() -> Self {
        Self {
            handle: Handle::from_path("ferris.png"),
        }
    }
}
```

Then, we use the [iced::widget::image::layout](https://docs.rs/iced/0.13.1/iced/widget/image/fn.layout.html) function to determine the [layout](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html#tymethod.layout) of our widget.

```rust
fn layout(&self, _tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
    iced::widget::image::layout(
        renderer,
        limits,
        &self.handle,
        Length::Fixed(200.),
        Length::Fixed(200.),
        iced::ContentFit::Contain,
    )
}
```

And we draw the image by the [iced::widget::image::draw](https://docs.rs/iced/0.13.1/iced/widget/image/fn.draw.html) function.

```rust
fn draw(
    &self,
    _state: &Tree,
    renderer: &mut Renderer,
    _theme: &Renderer::Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor: mouse::Cursor,
    _viewport: &Rectangle,
) {
    // ...

    iced::widget::image::draw(
        renderer,
        layout,
        &self.handle,
        iced::ContentFit::Contain,
        iced::widget::image::FilterMethod::Linear,
    );
}
```

Both functions require the `Renderer` to implement [iced::advanced::image::Renderer](https://docs.rs/iced/0.13.1/iced/advanced/image/trait.Renderer.html).

```rust
impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
```

The full code is as follows:

```rust
use iced::{
    Border, Color, Element, Length, Rectangle, Rotation, Shadow, Size,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
    widget::{Theme, container, image::Handle},
};
use std::env;

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
        container(MyWidgetWithImage::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct MyWidgetWithImage {
    handle: Handle,
}

impl MyWidgetWithImage {
    fn new() -> Self {
        println!("Current path: {}", env::current_dir().unwrap().display());
        Self {
            handle: Handle::from_path("../tutorial/pic/ferris.png"),
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        iced::widget::image::layout(
            renderer,
            limits,
            &self.handle,
            Length::Fixed(200.),
            Length::Fixed(200.),
            iced::ContentFit::Contain,
            Rotation::default(),
        )
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(1.0, 0.66, 0.6),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::BLACK,
        );

        iced::widget::image::draw(
            renderer,
            layout,
            &self.handle,
            iced::ContentFit::Contain,
            iced::widget::image::FilterMethod::Linear,
            Rotation::default(),
            1.0,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetWithImage> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn from(widget: MyWidgetWithImage) -> Self {
        Self::new(widget)
    }
}
```

![Custom Background](./pic/custom_background.png)

:arrow_right:  Next: [Widgets With Children](./widgets_with_children.md)

:blue_book: Back: [Table of contents](./../README.md)
