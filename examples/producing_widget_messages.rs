use iced::{
    Border, Color, Element, Event, Length, Rectangle, Shadow, Size, Task, Theme,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
    widget::{column, container, text},
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    MyWidgetPressed,
}

#[derive(Default)]
struct MyApp {
    count: u32,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (Self { count: 0 }, Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MyWidgetPressed => self.count += 1,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        container(
            column![MyWidget::new(Message::MyWidgetPressed), text(self.count)]
                .spacing(20)
                .align_x(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

struct MyWidget<Message> {
    pressed_message: Message,
}

impl<Message> MyWidget<Message> {
    fn new(pressed_message: Message) -> Self {
        Self { pressed_message }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidget<Message>
where
    Renderer: iced::advanced::Renderer,
    Message: Clone,
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

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if cursor.is_over(layout.bounds()) {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    shell.publish(self.pressed_message.clone());
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            }
        } else {
            event::Status::Ignored
        }
    }
}

impl<'a, Message, Renderer> From<MyWidget<Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidget<Message>) -> Self {
        Self::new(widget)
    }
}
