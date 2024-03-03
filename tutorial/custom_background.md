# Custom Background

We can also draw an image on a [Widget](https://docs.rs/iced/latest/iced/advanced/widget/trait.Widget.html).

Similar to the [Image](https://docs.rs/iced/latest/iced/widget/image/struct.Image.html) widget, we have to enable the [image](https://docs.rs/crate/iced/latest/features#image) feature.

```toml
[dependencies]
iced = { version = "0.10.0", features = ["image", "advanced"] }
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

Then, we use the [iced::widget::image::layout](https://docs.rs/iced/latest/iced/widget/image/fn.layout.html) function to determine the [layout](https://docs.rs/iced/latest/iced/advanced/widget/trait.Widget.html#tymethod.layout) of our widget.

```rust
fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
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

And we draw the image by the [iced::widget::image::draw](https://docs.rs/iced/latest/iced/widget/image/fn.draw.html) function.

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
    renderer.fill_quad(
        Quad {
            bounds: layout.bounds(),
            border_radius: 10.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(1.0, 0.66, 0.6),
        },
        Color::BLACK,
    );

    iced::widget::image::draw(renderer, layout, &self.handle, iced::ContentFit::Contain);
}
```

Both functions require the `Renderer` to implement [iced::advanced::image::Renderer](https://docs.rs/iced/latest/iced/advanced/image/trait.Renderer.html).

```rust
impl<Message, Renderer> Widget<Message, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
```

The full code is as follows:

```rust
use iced::{
    advanced::{
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
        Layout, Widget,
    },
    widget::{container, image::Handle},
    Color, Element, Length, Rectangle, Sandbox, Settings,
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
        container(MyWidgetWithImage::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

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

impl<Message, Renderer> Widget<Message, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        iced::widget::image::layout(
            renderer,
            limits,
            &self.handle,
            Length::Fixed(200.),
            Length::Fixed(200.),
            iced::ContentFit::Contain,
        )
    }

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
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(1.0, 0.66, 0.6),
            },
            Color::BLACK,
        );

        iced::widget::image::draw(renderer, layout, &self.handle, iced::ContentFit::Contain);
    }
}

impl<'a, Message, Renderer> From<MyWidgetWithImage> for Element<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn from(widget: MyWidgetWithImage) -> Self {
        Self::new(widget)
    }
}
```

![Custom Background](./pic/custom_background.png)

<!-- :arrow_right:  Next:  -->

:blue_book: Back: [Table of contents](./../README.md)
