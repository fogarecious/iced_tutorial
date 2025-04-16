use iced::{
    Task,
    widget::{button, column, text_input},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

const MY_TEXT_ID: &str = "my_text";

#[derive(Debug, Clone)]
enum Message {
    EditText,
    UpdateText(String),
}

#[derive(Default)]
struct MyApp {
    some_text: String,
}

impl MyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditText => return text_input::focus(text_input::Id::new(MY_TEXT_ID)),
            Message::UpdateText(s) => self.some_text = s,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Edit text").on_press(Message::EditText),
            text_input("", &self.some_text)
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
        ]
        .into()
    }
}
