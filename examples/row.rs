use iced::{
    Alignment, Length,
    widget::{Row, column, row},
};

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
            Row::with_children(vec![
                "Construct from the with_children function.".into(),
                "another element".into(),
            ]),
            Row::new()
                .push("Construct from the new function and the push method.")
                .push("another element again"),
            row(vec!["Construct from function".into()]),
            row!["Construct from macro"],
            row!["With padding"].padding(20),
            row!["Space between elements", "Space between elements",].spacing(20),
            row!["Different alignment"]
                .height(Length::Fill)
                .align_y(Alignment::Center),
        ]
        .into()
    }
}
