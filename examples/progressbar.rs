use iced::widget::{ProgressBar, column, progress_bar, text};

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
            ProgressBar::new(0.0..=100.0, 50.),
            text("Construct from function"),
            progress_bar(0.0..=100.0, 30.),
        ]
        .into()
    }
}
