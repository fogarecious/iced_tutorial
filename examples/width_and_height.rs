use iced::{
    Length,
    widget::{button, column, row},
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
            button("Shrink").width(Length::Shrink),
            button("Fill").width(Length::Fill),
            row![
                button("FillPortion 2").width(Length::FillPortion(2)),
                button("FillPortion 1").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            button("Fixed").width(Length::Fixed(100.)),
            button("Fill (height)").height(Length::Fill),
        ]
        .spacing(10)
        .into()
    }
}
