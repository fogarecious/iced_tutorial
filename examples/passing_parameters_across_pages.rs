use iced::{
    Task,
    widget::{button, column, text, text_input},
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    PageA(PageAMessage),
    PageB(PageBMessage),
}

trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<Message>;
}

struct MyApp {
    page: Box<dyn Page>,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(PageA::new()),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        let page = self.page.update(message);
        if let Some(p) = page {
            self.page = p;
        }
    }

    fn view(&self) -> iced::Element<Message> {
        self.page.view()
    }
}

// Page A
#[derive(Debug, Clone)]
enum PageAMessage {
    TextChanged(String),
    ButtonPressed,
}
type Ma = PageAMessage;

struct PageA {
    name: String,
}

impl PageA {
    fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Page for PageA {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PageA(msg) = message {
            match msg {
                PageAMessage::TextChanged(s) => self.name = s,
                PageAMessage::ButtonPressed => {
                    return Some(Box::new(PageB::new(self.name.clone())));
                }
            }
        }
        None
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text_input("Name", &self.name).on_input(|s| Message::PageA(Ma::TextChanged(s))),
            button("Log in").on_press(Message::PageA(Ma::ButtonPressed)),
        ]
        .into()
    }
}

// Page B
#[derive(Debug, Clone)]
enum PageBMessage {
    ButtonPressed,
}
type Mb = PageBMessage;

struct PageB {
    name: String,
}

impl PageB {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Page for PageB {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PageB(msg) = message {
            match msg {
                PageBMessage::ButtonPressed => return Some(Box::new(PageA::new())),
            }
        }
        None
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text(format!("Hello {}!", self.name)),
            button("Log out").on_press(Message::PageB(Mb::ButtonPressed)),
        ]
        .into()
    }
}
