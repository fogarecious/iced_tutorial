use iced::{
    Task,
    widget::{Slider, column, slider, text, vertical_slider},
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Update3(u32),
    Update4(u32),
    Update5(u32),
    Update6(u32),
    Update7(u32),
    Release7,
    Update8(u32),
}

#[derive(Default)]
struct MyApp {
    value3: u32,
    value4: u32,
    value5: u32,
    value6: u32,
    value7: u32,
    released_value_7: u32,
    value8: u32,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                value3: 50,
                value4: 50,
                value5: 50,
                value6: 50,
                value7: 50,
                released_value_7: 50,
                value8: 50,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Update3(v) => self.value3 = v,
            Message::Update4(v) => self.value4 = v,
            Message::Update5(v) => self.value5 = v,
            Message::Update6(v) => self.value6 = v,
            Message::Update7(v) => self.value7 = v,
            Message::Release7 => self.released_value_7 = self.value7,
            Message::Update8(v) => self.value8 = v,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text("Construct from struct"),
            Slider::new(0..=100, 50, |_| Message::DoNothing),
            text("Construct from function"),
            slider(0..=100, 50, |_| Message::DoNothing),
            text("Functional slider"),
            slider(0..=100, self.value3, |v| Message::Update3(v)),
            text("Shorter parameter"),
            slider(0..=100, self.value4, Message::Update4),
            text("Different step"),
            slider(0..=100, self.value5, Message::Update5).step(10u32),
            text("Different step when a shift key is pressed"),
            slider(0..=100, self.value6, Message::Update6).shift_step(10u32),
            text(format!("React to mouse release: {}", self.released_value_7)),
            slider(0..=100, self.value7, Message::Update7).on_release(Message::Release7),
            text("Press Ctrl (or Command) and click to return to the default value"),
            slider(0..=100, self.value8, Message::Update8).default(30u32),
            text("Vertical slider"),
            vertical_slider(0..=100, 50, |_| Message::DoNothing),
        ]
        .into()
    }
}
