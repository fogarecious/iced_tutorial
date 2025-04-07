use iced::{
    ContentFit,
    widget::{Image, column, image, text},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            text("Construct from struct"),
            Image::new("tutorial/pic/ferris.png").width(100).height(100),
            text("Construct from function"),
            image("tutorial/pic/ferris.png").width(100).height(100),
            text("Different content fit"),
            image("tutorial/pic/ferris.png")
                .content_fit(ContentFit::Cover)
                .width(100)
                .height(100),
        ]
        .into()
    }
}
