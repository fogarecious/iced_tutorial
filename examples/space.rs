use iced::{
    Alignment, Length,
    widget::{Space, button, column, horizontal_space, row, vertical_space},
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
            row![
                button("Horizontal space 1A"),
                Space::with_width(50),
                button("Horizontal space 1B"),
            ],
            row![
                button("Horizontal space 2A"),
                Space::with_width(Length::Fill),
                button("Horizontal space 2B"),
            ],
            row![
                button("Horizontal space 3A"),
                horizontal_space(),
                button("Horizontal space 3B"),
            ],
            button("Vertical space 1A"),
            Space::with_height(50),
            button("Vertical space 1B"),
            Space::with_height(Length::Fill),
            button("Vertical space 2A"),
            vertical_space(),
            button("Vertical space 2B"),
            button("Diagonal space A"),
            row![Space::new(50, 50), button("Diagonal space B"),].align_y(Alignment::End)
        ]
        .into()
    }
}
