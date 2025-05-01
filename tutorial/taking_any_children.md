# Taking Any Children

Since all [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html) can be transformed to [Element](https://docs.rs/iced_core/0.13.1/iced_core/struct.Element.html), our custom widget is able to take any [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html) as its children.

This time, our `MyWidgetOuter` will take an [Element](https://docs.rs/iced_core/0.13.1/iced_core/struct.Element.html) as its inner widget when it is initialized.

```rust
struct MyWidgetOuter<'a, Message, Renderer> {
    inner_widget: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Renderer> MyWidgetOuter<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn new(inner_widget: Element<'a, Message, Theme, Renderer>) -> Self {
        Self { inner_widget }
    }
}
```

When we draw or layout the `inner_widget`, we will use its methods from [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html).
Yet, the `inner_widget` is of type [Element](https://docs.rs/iced_core/0.13.1/iced_core/struct.Element.html).
So, we have to cast it as [Widget](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html) by the [as_widget](https://docs.rs/iced_core/0.13.1/iced_core/struct.Element.html#method.as_widget) method.

```rust
fn layout(
    &self,
    tree: &mut Tree,
    renderer: &Renderer,
    limits: &layout::Limits,
) -> layout::Node {
    let mut child_node =
        self.inner_widget
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits);

    let size_of_this_node = child_node.size().expand(Size::new(50., 50.));

    child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

    layout::Node::with_children(size_of_this_node, vec![child_node])
}
```

In the code above, we make the size of `MyWidgetOuter` relative to its `inner_widget`.
More precisely, we retrieve the size of `inner_widget` and [pad](https://docs.rs/iced_core/0.13.1/iced_core/struct.Size.html#method.pad) the size as the size of `MyWidgetOuter`.

Then, in the [draw](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html#tymethod.draw) method of `MyWidgetOuter`, we also draw the `inner_widget`.

```rust
fn draw(
    &self,
    state: &Tree,
    renderer: &mut Renderer,
    theme: &Theme,
    style: &renderer::Style,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    viewport: &Rectangle,
) {
    renderer.fill_quad(
        Quad {
            bounds: layout.bounds(),
            border: Border {
                color: Color::from_rgb(0.6, 0.93, 1.0),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
        },
        Color::from_rgb(0.0, 0.33, 0.4),
    );

    self.inner_widget.as_widget().draw(
        &state.children[0],
        renderer,
        theme,
        style,
        layout.children().next().unwrap(),
        cursor,
        viewport,
    );
}
```

Note that we have to pass the child state `&state.children[0]` to `inner_widget` since the anonymous widget may need the information about its state.

To make the underlying system aware of the child state, we have to explicitly tell the system the existence of the child.
Otherwise, `state.children` in [draw](https://docs.rs/iced/0.13.1/iced/advanced/widget/trait.Widget.html#tymethod.draw) will be empty.

```rust
fn children(&self) -> Vec<Tree> {
    vec![Tree::new(self.inner_widget.as_widget())]
}

fn diff(&self, tree: &mut Tree) {
    tree.diff_children(std::slice::from_ref(&self.inner_widget));
}
```

The full code is as follows:

```rust
use iced::{
    Alignment, Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
    widget::{button, container},
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
        container(MyWidgetOuter::new(button("Other widget").into()))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct MyWidgetOuter<'a, Message, Renderer> {
    inner_widget: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Renderer> MyWidgetOuter<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn new(inner_widget: Element<'a, Message, Theme, Renderer>) -> Self {
        Self { inner_widget }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetOuter<'_, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.inner_widget));
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let mut child_node =
            self.inner_widget
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits);

        let size_of_this_node = child_node.size().expand(Size::new(50., 50.));

        child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

        layout::Node::with_children(size_of_this_node, vec![child_node])
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.inner_widget.as_widget())]
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.93, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.33, 0.4),
        );

        self.inner_widget.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetOuter<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(widget: MyWidgetOuter<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}
```

![Taking Any Children](./pic/taking_any_children.png)

:arrow_right:  Next: [Loading Images Asynchronously](./loading_images_asynchronously.md)

:blue_book: Back: [Table of contents](./../README.md)
