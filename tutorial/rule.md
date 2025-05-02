# Rule

The [Rule](https://docs.rs/iced/0.13.1/iced/widget/rule/struct.Rule.html) widget is a horizontal (or vertical) line for separating widgets clearly.
It has two methods of construction: the `horizontal_rule` function and the `Rule::horizontal` constructor.
For the vertical rule, we can use the `vertical_rule` function or the `Rule::vertical` constructor.
We can change the space around it,

```rust
use iced::widget::{Rule, column, horizontal_rule, text, vertical_rule};

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
            Rule::horizontal(0),
            text("Construct from function"),
            horizontal_rule(0),
            text("Different space"),
            horizontal_rule(50),
            text("Vertical rule"),
            vertical_rule(100),
        ]
        .into()
    }
}
```

![Rule](./pic/rule.png)

:arrow_right: Next: [Image](./image.md)

:blue_book: Back: [Table of contents](./../README.md)
