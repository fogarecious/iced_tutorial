use iced::widget::{button, column, text};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

#[derive(Default)]
struct MyApp {
    counter: usize,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => self.counter += 1,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text(self.counter),
            button("Increase").on_press(Message::ButtonPressed),
        ]
        .into()
    }
}
