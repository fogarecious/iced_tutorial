use iced::{Point, Size, window};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .window(window::Settings {
            size: Size {
                width: 70.,
                height: 30.,
            },
            position: window::Position::Specific(Point { x: 50., y: 60. }),
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        "Hello".into()
    }
}
