use iced::{
    Task,
    widget::{button, row},
    window,
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    CloseWindow,
}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn close(id: window::Id) -> Task<Message> {
        window::close(id)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CloseWindow => window::get_latest().and_then(Self::close),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        row![button("Close window").on_press(Message::CloseWindow),].into()
    }
}
