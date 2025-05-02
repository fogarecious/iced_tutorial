# Batch Tasks

Sometimes, we want to execute two or more `Tasks` at the same time.
In this case, we can use `Task::batch` function.

```rust
use iced::{
    Task,
    widget::{button, column, text_input},
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

const MY_TEXT_ID: &str = "my_text";

#[derive(Debug, Clone)]
enum Message {
    UpdateText(String),
    SelectAll,
}

#[derive(Default)]
struct MyApp {
    some_text: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            some_text: "abc".to_string(),
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateText(s) => self.some_text = s,
            Message::SelectAll => {
                return Task::batch(vec![
                    text_input::focus(text_input::Id::new(MY_TEXT_ID)),
                    text_input::select_all(text_input::Id::new(MY_TEXT_ID)),
                ]);
            }
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text_input("", &self.some_text)
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
            button("Select all").on_press(Message::SelectAll),
        ]
        .into()
    }
}
```

![Batch commands](./pic/batch_tasks.png)

:arrow_right:  Next: [Executing Custom Tasks](./executing_custom_tasks.md)

:blue_book: Back: [Table of contents](./../README.md)
