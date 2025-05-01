use iced::{
    Alignment, Border, Color, Element, Length, Rectangle, Shadow, Size,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
    widget::{Theme, container},
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
        container(MyWidgetOuter::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct MyWidgetInner;

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetInner
where
    Renderer: iced::advanced::Renderer,
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
        _viewport: &Rectangle,
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
    }
}

impl<'a, Message, Renderer> From<MyWidgetInner> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidgetInner) -> Self {
        Self::new(widget)
    }
}

struct MyWidgetOuter {
    inner_widget: MyWidgetInner,
}

impl MyWidgetOuter {
    fn new() -> Self {
        Self {
            inner_widget: MyWidgetInner,
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetOuter
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let inner_widget = &self.inner_widget as &dyn Widget<Message, Theme, Renderer>;
        let mut child_node = inner_widget.layout(tree, renderer, limits);

        let size_of_this_node = Size::new(200., 200.);

        child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

        layout::Node::with_children(size_of_this_node, vec![child_node])
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

        let inner_widget = &self.inner_widget as &dyn Widget<Message, Theme, Renderer>;
        inner_widget.draw(
            state,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetOuter> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidgetOuter) -> Self {
        Self::new(widget)
    }
}
