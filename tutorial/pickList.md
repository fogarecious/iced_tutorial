# PickList

The [PickList](https://docs.rs/iced/0.12.1/iced/widget/pick_list/struct.PickList.html) widget represents a choice among multiple values.
It has two methods of constructions.
It supports reactions to option selections.
A placeholder can be set when options are not selected yet.
It is able to change styles of the text.
We can add padding around the text inside.
We can also change the icon of the handle.

```rust
use iced::{
    font::Family,
    widget::{column, pick_list, pick_list::Handle, text::Shaping, PickList},
    Font, Pixels, Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    DoNothing,
    Update3(String),
}

struct MyApp {
    pick_list_3: Option<String>,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            pick_list_3: Some("Functional pick list".into()),
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::DoNothing => {}
            MyAppMessage::Update3(s) => self.pick_list_3 = Some(s),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            PickList::new(
                vec!["Construct from struct"],
                Some("Construct from struct"),
                |_| MyAppMessage::DoNothing
            ),
            pick_list(
                vec!["Construct from function"],
                Some("Construct from function"),
                |_| MyAppMessage::DoNothing
            ),
            pick_list(
                ["Functional pick list", "Other choices 1", "Other choices 2"]
                    .map(|s| s.to_string())
                    .to_vec(),
                self.pick_list_3.clone(),
                |s| MyAppMessage::Update3(s)
            ),
            pick_list(vec!["A", "B", "C"], None::<&str>, |_| MyAppMessage::DoNothing)
                .placeholder("Placeholder"),
            pick_list(vec!["Different font"], Some("Different font"), |_| {
                MyAppMessage::DoNothing
            })
            .font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            pick_list(vec!["Larger text"], Some("Larger text"), |_| {
                MyAppMessage::DoNothing
            })
            .text_size(24),
            pick_list(
                vec!["Special character 😊"],
                Some("Special character 😊"),
                |_| MyAppMessage::DoNothing
            )
            .text_shaping(Shaping::Advanced),
            pick_list(vec!["With padding"], Some("With padding"), |_| {
                MyAppMessage::DoNothing
            })
            .padding(20),
            pick_list(vec!["Different handle"], Some("Different handle"), |_| {
                MyAppMessage::DoNothing
            })
            .handle(Handle::Arrow { size: Some(Pixels(24.)) }),
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
```

![PickList](./pic/picklist.png)

:arrow_right:  Next: [ComboBox](./combobox.md)

:blue_book: Back: [Table of contents](./../README.md)
