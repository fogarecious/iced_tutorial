# TextInput

The [TextInput](https://docs.rs/iced/0.13.1/iced/widget/struct.TextInput.html) widget let users to input texts.
It has two methods of construction: the `text_input` function and the `TextInput::new` constructor.
It is disabled by default if [on_input](https://docs.rs/iced/0.13.1/iced/widget/struct.TextInput.html#method.on_input) is not defined.
It supports reactions to pasting texts or keyboard submissions.
You can change fonts, text sizes, add password masks, add padding around the inner text, and add an icon.

```rust
use iced::{
    Font,
    font::Family,
    widget::{
        TextInput, column, text, text_input,
        text_input::{Icon, Side},
    },
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    Update3(String),
    Update4(String),
    Update5(String),
    Paste5(String),
    Update6(String),
    Submit6,
    Update7(String),
    Update11(String),
}

#[derive(Default)]
struct MyApp {
    text3: String,
    text4: String,
    text5: String,
    info5: String,
    text6: String,
    info6: String,
    text7: String,
    text11: String,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Update3(s) => self.text3 = s,
            Message::Update4(s) => self.text4 = s,
            Message::Update5(s) => {
                self.text5 = s;
                self.info5 = "".into();
            }
            Message::Paste5(s) => {
                self.text5 = s;
                self.info5 = "Pasted".into();
            }
            Message::Update6(s) => {
                self.text6 = s;
                self.info6 = "".into();
            }
            Message::Submit6 => self.info6 = "Submitted".into(),
            Message::Update7(s) => self.text7 = s,
            Message::Update11(s) => self.text11 = s,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text_input("Construct from function", ""),
            TextInput::new("Construct from struct", ""),
            text_input("Enabled text input", self.text3.as_str()).on_input(|s| Message::Update3(s)),
            text_input("Shorter on_input", self.text4.as_str()).on_input(Message::Update4),
            text_input("Press Ctrl/Cmd + V", self.text5.as_str())
                .on_input(Message::Update5)
                .on_paste(Message::Paste5),
            text(self.info5.as_str()),
            text_input("Press enter", self.text6.as_str())
                .on_input(Message::Update6)
                .on_submit(Message::Submit6),
            text(self.info6.as_str()),
            text_input("Password", self.text7.as_str())
                .secure(true)
                .on_input(Message::Update7),
            text_input("Different font", "").font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            text_input("Larger text", "").size(24),
            text_input("With padding", "").padding(20),
            text_input("Icon", self.text11.as_str())
                .icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '\u{2705}',
                    size: None,
                    spacing: 10.,
                    side: Side::Left,
                })
                .on_input(Message::Update11),
        ]
        .into()
    }
}
```

![TextInput](./pic/text_input.png)

:arrow_right: Next: [Checkbox](./checkbox.md)

:blue_book: Back: [Table of contents](./../README.md)
