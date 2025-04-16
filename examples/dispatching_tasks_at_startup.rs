use iced::{
    Task, Theme,
    widget::{column, text_input},
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .theme(|_| Theme::TokyoNight)
        .run_with(MyApp::new)
}

const MY_TEXT_ID: &str = "my_text";

#[derive(Debug, Clone)]
enum Message {
    UpdateText(String),
}

#[derive(Default)]
struct MyApp {
    some_text: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        let app = Self::default();
        (app, text_input::focus(text_input::Id::new(MY_TEXT_ID)))
        // Try returning (app, Task::none()) instead and see what happens
        // (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateText(s) => self.some_text = s,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text_input("", &self.some_text)
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
        ]
        .into()
    }
}
