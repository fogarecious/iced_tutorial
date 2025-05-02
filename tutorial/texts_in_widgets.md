# Texts In Widgets

In addition to draw a [Quad](https://docs.rs/iced/0.13.1/iced/advanced/renderer/struct.Quad.html), we can also draw texts in our widgets.

For example, suppose we would like to draw a string slice named `CONTENT`.

```rust
struct MyWidgetWithText;

impl MyWidgetWithText {
    const CONTENT: &'static str = "  My Widget  ";

    fn new() -> Self {
        Self
    }
}
```

We use the [Renderer](https://docs.rs/iced/0.13.1/iced/advanced/text/trait.Renderer.html)'s [fill_text](https://docs.rs/iced/0.13.1/iced/advanced/text/trait.Renderer.html#tymethod.fill_text) method to draw the text.

```rust
fn draw(
    &self,
    _state: &Tree,
    renderer: &mut Renderer,
    _theme: &Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor: mouse::Cursor,
    viewport: &Rectangle,
) {
    // ...

    let bounds = layout.bounds();

    renderer.fill_text(
        Text {
            content: Self::CONTENT,
            bounds: bounds.size(),
            size: renderer.default_size(),
            line_height: LineHeight::default(),
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            shaping: Shaping::default(),
        },
        bounds.center(),
        Color::from_rgb(0.6, 0.8, 1.0),
        *viewport,
    );
}
```

The [fill_text](https://docs.rs/iced/0.13.1/iced/advanced/text/trait.Renderer.html#tymethod.fill_text) method needs the `Renderer` type to implement [iced::advanced::text::Renderer](https://docs.rs/iced/0.13.1/iced/advanced/text/trait.Renderer.html).
Thus we have to require this in our [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html) implementation.

```rust
impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithText
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
```

Since the requirement of the `Renderer` type is changed, we have to change the requirement in `From<MyWidgetWithText>` for [Element](https://docs.rs/iced/0.13.1/iced/type.Element.html), too.

```rust
impl<'a, Message, Renderer> From<MyWidgetWithText> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
```

The full code is as follows:

```rust
use iced::{
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Text, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
    alignment::{Horizontal, Vertical},
    widget::{
        container,
        text::{LineHeight, Shaping, Wrapping},
    },
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

    fn view(&self) -> iced::Element<'_, Message> {
        container(MyWidgetWithText::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct MyWidgetWithText;

impl MyWidgetWithText {
    const CONTENT: &'static str = "My Widget";

    fn new() -> Self {
        Self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithText
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
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
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(iced::Size {
            width: 100.0,
            height: 100.0,
        })
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.2, 0.4),
        );

        let bounds = layout.bounds();

        renderer.fill_text(
            Text {
                content: Self::CONTENT.to_string(),
                bounds: bounds.size(),
                size: renderer.default_size(),
                line_height: LineHeight::default(),
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::default(),
                wrapping: Wrapping::default(),
            },
            bounds.center(),
            Color::from_rgb(0.6, 0.8, 1.0),
            *viewport,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetWithText> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(widget: MyWidgetWithText) -> Self {
        Self::new(widget)
    }
}
```

![Texts In Widgets](./pic/texts_in_widgets.png)

:arrow_right:  Next: [Custom Background](./custom_background.md)

:blue_book: Back: [Table of contents](./../README.md)
