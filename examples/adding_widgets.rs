use iced::widget::{button, column, row, text};

fn main() -> iced::Result {
    iced::run("MyApp", MyApp::update, MyApp::view)
}

#[derive(Default)]
struct MyApp;

#[derive(Debug, Clone)]
enum Message {}

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![text("Yes or no?"), row![button("Yes"), button("No")],].into()
    }
}
