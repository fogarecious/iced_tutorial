# Button

The [Button](https://docs.rs/iced/0.13.1/iced/widget/button/struct.Button.html) widget supports reactions to pressing/touching events.
It has two methods of constructions: the `button` function, and the `Button::new` constructor.
By default, the button is disabled if [on_press](https://docs.rs/iced/0.13.1/iced/widget/button/struct.Button.html#method.on_press) is not defined.
We can also set padding around the text of the button.

```rust
use iced::widget::{Button, button, column};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoSomething,
}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            Button::new("Disabled button"),
            button("Construct from function"),
            button("Enabled button").on_press(Message::DoSomething),
            button("With padding").padding(20),
        ]
        .into()
    }
}
```

![Button](./pic/button.png)

:arrow_right: Next: [TextInput](./text_input.md)

:blue_book: Back: [Table of contents](./../README.md)
