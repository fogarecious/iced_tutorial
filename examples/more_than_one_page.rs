use iced::widget::{button, column, text};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Default)]
enum Page {
    #[default]
    A,
    B,
}

#[derive(Debug, Clone)]
enum Message {
    GoToBButtonPressed,
    GoToAButtonPressed,
}

#[derive(Default)]
struct MyApp {
    page: Page,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        self.page = match message {
            Message::GoToBButtonPressed => Page::B,
            Message::GoToAButtonPressed => Page::A,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match self.page {
            Page::A => column![
                text("Page A"),
                button("Go to B").on_press(Message::GoToBButtonPressed),
            ],
            Page::B => column![
                text("Page B"),
                button("Go to A").on_press(Message::GoToAButtonPressed),
            ],
        }
        .into()
    }
}
