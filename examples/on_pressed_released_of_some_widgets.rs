use iced::{Task, widget::mouse_area};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Pressed,
    Released,
}

#[derive(Default)]
struct MyApp {
    state: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: "Start".into(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Pressed => self.state = "Pressed".into(),
            Message::Released => self.state = "Released".into(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        mouse_area(self.state.as_str())
            .on_press(Message::Pressed)
            .on_release(Message::Released)
            .into()
    }
}
