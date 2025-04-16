use iced::{
    Task,
    widget::{button, column, text},
};
use std::time::Duration;

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Execute,
    Done,
}

#[derive(Default)]
struct MyApp {
    state: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: "Ready".into(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Execute => {
                self.state = "Executing".into();
                return Task::perform(tokio::time::sleep(Duration::from_secs(1)), |_| {
                    Message::Done
                });
            }
            Message::Done => self.state = "Done".into(),
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Execute").on_press(Message::Execute),
            text(self.state.as_str()),
        ]
        .into()
    }
}
