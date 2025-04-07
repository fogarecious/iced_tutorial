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
