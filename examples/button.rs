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
