# On Pressed/Released Of Some Widgets

If we only consider mouse pressed or released events, we can use [MouseArea](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html).
The [MouseArea](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html) gives the widget being put in it the sense of mouse pressed/released events, even if the widget has no build-in support of the events.
For example, we can make a [Text](https://docs.rs/iced/0.12.1/iced/widget/type.Text.html) to respond to mouse pressed/released events.

```rust
use iced::{widget::mouse_area, Sandbox, Settings};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    Pressed,
    Released,
}

struct MyApp {
    state: String,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            state: "Start".into(),
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::Pressed => self.state = "Pressed".into(),
            MyAppMessage::Released => self.state = "Released".into(),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        mouse_area(self.state.as_str())
            .on_press(MyAppMessage::Pressed)
            .on_release(MyAppMessage::Released)
            .into()
    }
}
```

In addition to [on_press](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html#method.on_press) and [on_release](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html#method.on_release) methods, [MouseArea](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html) also supports [on_middle_press](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html#method.on_middle_press), [on_right_press](https://docs.rs/iced/0.12.1/iced/widget/struct.MouseArea.html#method.on_right_press), etc.

When the mouse is pressed:

![On pressed/released of some widgets A](./pic/on_pressed_released_of_some_widgets_a.png)

And when the mouse is released:

![On pressed/released of some widgets B](./pic/on_pressed_released_of_some_widgets_b.png)

:arrow_right:  Next: [Producing Messages By Mouse Events](./producing_messages_by_mouse_events.md)

:blue_book: Back: [Table of contents](./../README.md)
