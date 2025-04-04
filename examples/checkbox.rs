use iced::{
    Font,
    font::Family,
    widget::{
        Checkbox, checkbox,
        checkbox::Icon,
        column,
        text::{LineHeight, Shaping},
    },
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Update4(bool),
    Update5(bool),
}

#[derive(Default)]
struct MyApp {
    checkbox4: bool,
    checkbox5: bool,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Update4(b) => self.checkbox4 = b,
            Message::Update5(b) => self.checkbox5 = b,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            Checkbox::new("Construct from struct", false),
            checkbox("Construct from function", false),
            checkbox("Enabled checkbox", false).on_toggle(|_| Message::DoNothing),
            checkbox("Functional checkbox", self.checkbox4).on_toggle(|b| Message::Update4(b)),
            checkbox("Shorter parameter", self.checkbox5).on_toggle(Message::Update5),
            checkbox("Larger box", false)
                .on_toggle(|_| Message::DoNothing)
                .size(30),
            checkbox("Different icon", true)
                .on_toggle(|_| Message::DoNothing)
                .icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '*',
                    size: None,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default()
                }),
            checkbox("Different font", false)
                .on_toggle(|_| Message::DoNothing)
                .font(Font {
                    family: Family::Fantasy,
                    ..Font::DEFAULT
                }),
            checkbox("Larger text", false)
                .on_toggle(|_| Message::DoNothing)
                .text_size(24),
            checkbox("Special character 😊", false)
                .on_toggle(|_| Message::DoNothing)
                .text_shaping(Shaping::Advanced),
            checkbox("Space between box and text", false)
                .on_toggle(|_| Message::DoNothing)
                .spacing(30),
        ]
        .into()
    }
}
