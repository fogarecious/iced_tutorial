# Producing Widget Messages

Our custom widgets are able to send Messages.

To do so, we need to store the `Message` we are going to send in the widget.

```rust
struct MyWidget<Message> {
    pressed_message: Message,
}

impl<Message> MyWidget<Message> {
    fn new(pressed_message: Message) -> Self {
        Self { pressed_message }
    }
}
```

We use a generic type `Message` for `MyWidget`, so that the sent `pressed_message` in `MyWidget` will match the associated type `Message` of our application.

The message `pressed_message` will be sent when the widget is pressed.

```rust
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
```

We use [`shell.publish`](https://docs.rs/iced/0.13.1/iced/advanced/struct.Shell.html#method.publish) to send `pressed_message` to our app.
To ensure the mouse pressed event happens within the range of the widget, we use [`cursor.is_over`](https://docs.rs/iced/0.13.1/iced/advanced/mouse/enum.Cursor.html#method.is_over) and pass to it [`layout.bounds()`](https://docs.rs/iced/0.13.1/iced/advanced/struct.Layout.html#method.bounds) to check the mouse position and match the `event` to `Event::Mouse(mouse::Event::ButtonPressed(_))` to check the mouse button state.

Finally, we pass our `Message` to the widget.

```rust
#[derive(Debug, Clone)]
enum Message {
    MyWidgetPressed,
}

impl MyApp {
    fn view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                MyWidget::new(Message::MyWidgetPressed),
                // ...
            ]
            // ...
        )
        // ...
    }
}
```

The full code is as follows:

```rust
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
```

![Producing Widget Messages](./pic/producing_widget_messages.png)

:arrow_right:  Next: [Mouse Pointer Over Widgets](./mouse_pointer_over_widgets.md)

:blue_book: Back: [Table of contents](./../README.md)
