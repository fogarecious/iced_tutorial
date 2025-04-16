# Dispatching Tasks at Startup

Sometimes you may want to dispatch a task when your application starts up. This is useful when you need to fetch some data to display on the screen, or to perform some initialization logic.

To accomplish that, you can't just run the application with [`iced::run`](https://docs.rs/iced/0.13.1/iced/fn.run.html), because this function doesn't allow you to customize the initialization logic of your application. Instead, you should use [`iced::application`](https://docs.rs/iced/0.13.1/iced/application/index.html). This function is called the same way as `iced::run`. But instead of running the application straightaway, it will return an [`Application`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html) instance.

With an instance of `Application`, you can customize styles, and use the [`run_with`](https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.run_with) function to dispatch a task.

The [`run_with`]((https://docs.rs/iced/0.13.1/iced/application/struct.Application.html#method.run_with)) function expects a function that returns a tuple with an initial state, and a task.

To keep things aligned with the previous example, in the example below, we dispatch a task that focuses the input field. Of course, tasks allow us to perform many more complex operations.

```rust
use iced::{
    Task,
    widget::{button, column, text_input},
};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

const MY_TEXT_ID: &str = "my_text";

#[derive(Debug, Clone)]
enum Message {
    EditText,
    UpdateText(String),
}

#[derive(Default)]
struct MyApp {
    some_text: String,
}

impl MyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditText => return text_input::focus(text_input::Id::new(MY_TEXT_ID)),
            Message::UpdateText(s) => self.some_text = s,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Edit text").on_press(Message::EditText),
            text_input("", &self.some_text)
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
        ]
        .into()
    }
}
```

![Dispatching Tasks at Startup](./pic/dispatching_tasks_at_startup.png)

:arrow_right:  Next: [Batch Tasks](./batch_tasks.md)

:blue_book: Back: [Table of contents](./../README.md)
