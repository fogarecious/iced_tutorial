# More Than One Page

To have multiple pages, we can use a simple trick of rendering the pages based on state.
We can add a state field `page` to the main struct `MyApp`.
Then, depending on the value of `page`, we can render different widgets in the `view` method.

```rust
use iced::widget::{button, column, text};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Default)]
enum Page {
    #[default]
    A,
    B,
}

#[derive(Debug, Clone)]
enum Message {
    GoToBButtonPressed,
    GoToAButtonPressed,
}

#[derive(Default)]
struct MyApp {
    page: Page,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        self.page = match message {
            Message::GoToBButtonPressed => Page::B,
            Message::GoToAButtonPressed => Page::A,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match self.page {
            Page::A => column![
                text("Page A"),
                button("Go to B").on_press(Message::GoToBButtonPressed),
            ],
            Page::B => column![
                text("Page B"),
                button("Go to A").on_press(Message::GoToAButtonPressed),
            ],
        }
        .into()
    }
}
```

Page A:

![Page A](./pic/more_than_one_page_a.png)

And page B:

![Page B](./pic/more_than_one_page_b.png)

:arrow_right: Next: [Memoryless Pages](./memoryless_pages.md)

:blue_book: Back: [Table of contents](./../README.md)
