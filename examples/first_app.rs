fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

type Message = ();

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        "Hello World!".into()
    }
}
