use iced::{
    Task,
    time::{self, Duration},
    widget::text,
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Update,
}

#[derive(Default)]
struct MyApp {
    seconds: u32,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (Self { seconds: 0 }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update => self.seconds += 1,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }
}
