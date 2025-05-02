use iced::{
    Font, Task,
    font::Family,
    widget::{
        ComboBox, column, combo_box,
        combo_box::State,
        text,
        text_input::{Icon, Side},
    },
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Select4(String),
    Select5(String),
    Select6(String),
    Input6(String),
    Select7(String),
    Hover7(String),
    Select8(String),
    Hover8(String),
    Close8,
}

#[derive(Default)]
struct MyApp {
    state1: State<u32>,
    state2: State<u32>,
    state3: State<String>,
    state4: State<String>,
    select4: Option<String>,
    state5: State<String>,
    select5: Option<String>,
    state6: State<String>,
    select6: Option<String>,
    input6: String,
    state7: State<String>,
    select7: Option<String>,
    state8: State<String>,
    select8: Option<String>,
    info8: String,
    state9: State<u32>,
    state10: State<u32>,
    state11: State<u32>,
    state12: State<u32>,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state1: State::new(vec![]),
                state2: State::new(vec![]),
                state3: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                state4: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                select4: None,
                state5: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                select5: None,
                state6: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                select6: None,
                input6: "".into(),
                state7: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                select7: None,
                state8: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
                select8: None,
                info8: "".into(),
                state9: State::new(vec![]),
                state10: State::new(vec![]),
                state11: State::new(vec![]),
                state12: State::new(vec![]),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Select4(s) => self.select4 = Some(s),
            Message::Select5(s) => self.select5 = Some(s),
            Message::Select6(s) => self.select6 = Some(s),
            Message::Input6(s) => self.input6 = s,
            Message::Select7(s) => self.select7 = Some(s),
            Message::Hover7(s) => self.select7 = Some(s),
            Message::Select8(s) => self.select8 = Some(s),
            Message::Hover8(s) => self.select8 = Some(s),
            Message::Close8 => self.info8 = "Done!".into(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            ComboBox::new(&self.state1, "Construct from struct", None, |_| {
                Message::DoNothing
            }),
            combo_box(&self.state2, "Construct from function", None, |_| {
                Message::DoNothing
            }),
            combo_box(&self.state3, "With list of items", None, |_| {
                Message::DoNothing
            }),
            combo_box(
                &self.state4,
                "Functional combobox (Press Enter or click an option)",
                self.select4.as_ref(),
                |s| Message::Select4(s)
            ),
            combo_box(
                &self.state5,
                "Shorter parameter (Press Enter or click an option)",
                self.select5.as_ref(),
                Message::Select5
            ),
            text(&self.input6),
            combo_box(
                &self.state6,
                "Respond to input",
                self.select6.as_ref(),
                Message::Select6
            )
            .on_input(Message::Input6),
            combo_box(
                &self.state7,
                "Respond to option hover",
                self.select7.as_ref(),
                Message::Select7
            )
            .on_option_hovered(Message::Hover7),
            text(&self.info8),
            combo_box(
                &self.state8,
                "Respond to closing menu",
                self.select8.as_ref(),
                Message::Select8
            )
            .on_option_hovered(Message::Hover8)
            .on_close(Message::Close8),
            combo_box(&self.state9, "Different font", None, |_| {
                Message::DoNothing
            })
            .font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            combo_box(&self.state10, "Larger text", None, |_| {
                Message::DoNothing
            })
            .size(24.),
            combo_box(&self.state11, "With padding", None, |_| {
                Message::DoNothing
            })
            .padding(20),
            combo_box(&self.state12, "Icon", None, |_| Message::DoNothing).icon(Icon {
                font: Font::DEFAULT,
                code_point: '\u{2705}',
                size: None,
                spacing: 10.,
                side: Side::Left,
            }),
        ]
        .into()
    }
}
