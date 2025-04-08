use iced::{
    Background, Color,
    widget::{button, column, row, text},
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
            text("A solid color").style(|_| text::Style {
                color: Some(Color::from_rgb(0.5, 0.5, 0.0))
            }),
            text("A color from the theme").style(text::danger),
            row![
                button("Cancel").style(button::danger),
                button("Go!~~").style(button::primary),
                button("Save").style(|_, _| button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.0, 0.5, 0.5))),
                    ..Default::default()
                })
            ],
        ]
        .into()
    }
}
