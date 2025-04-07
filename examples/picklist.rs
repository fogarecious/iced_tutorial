use iced::{
    Font, Pixels,
    font::Family,
    widget::{PickList, column, pick_list, pick_list::Handle, row, text, text::Shaping},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Update3(String),
    Open10,
    Close11,
}

#[derive(Default)]
struct MyApp {
    pick_list_3: Option<String>,
    info_10: String,
    info_11: String,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Update3(s) => self.pick_list_3 = Some(s),
            Message::Open10 => self.info_10 = "Open".into(),
            Message::Close11 => self.info_11 = "Close".into(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            PickList::new(
                vec!["Construct from struct"],
                Some("Construct from struct"),
                |_| Message::DoNothing
            ),
            pick_list(
                vec!["Construct from function"],
                Some("Construct from function"),
                |_| Message::DoNothing
            ),
            pick_list(
                ["Functional pick list", "Other choices 1", "Other choices 2"]
                    .map(|s| s.to_string())
                    .to_vec(),
                self.pick_list_3.clone(),
                |s| Message::Update3(s)
            ),
            pick_list(vec!["A", "B", "C"], None::<&str>, |_| {
                Message::DoNothing
            })
            .placeholder("Placeholder"),
            pick_list(vec!["Different font"], Some("Different font"), |_| {
                Message::DoNothing
            })
            .font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            pick_list(vec!["Larger text"], Some("Larger text"), |_| {
                Message::DoNothing
            })
            .text_size(24),
            pick_list(
                vec!["Special character 😊"],
                Some("Special character 😊"),
                |_| Message::DoNothing
            )
            .text_shaping(Shaping::Advanced),
            pick_list(vec!["With padding"], Some("With padding"), |_| {
                Message::DoNothing
            })
            .padding(20),
            pick_list(vec!["Different handle"], Some("Different handle"), |_| {
                Message::DoNothing
            })
            .handle(Handle::Arrow {
                size: Some(Pixels(24.))
            }),
            row![
                pick_list(vec!["Respond to open"], Some("Respond to open"), |_| {
                    Message::DoNothing
                })
                .on_open(Message::Open10),
                text(&self.info_10),
            ],
            row![
                pick_list(vec!["Respond to close"], Some("Respond to close"), |_| {
                    Message::DoNothing
                })
                .on_close(Message::Close11),
                text(&self.info_11),
            ],
        ]
        .into()
    }
}
