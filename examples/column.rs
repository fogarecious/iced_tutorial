use iced::{
    Alignment, Length,
    widget::{Column, column},
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
            Column::with_children(vec![
                "Construct from the with_children function".into(),
                "another element".into(),
            ]),
            Column::new()
                .push("Construct from the new function and the push method")
                .push("another element again"),
            column(vec!["Construct from function".into()]),
            column!["Construct from macro"],
            column!["With padding"].padding(20),
            column!["Different alignment"]
                .width(Length::Fill)
                .align_x(Alignment::Center),
            column!["Space between elements", "Space between elements",].spacing(20),
        ]
        .into()
    }
}
