use iced::{
    Element, Result, Size, Task,
    widget::{button, row, text_input},
    window,
};

fn main() -> Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    UpdateWidth(String),
    UpdateHeight(String),
    ResizeWindow,
}

struct MyApp {
    width: String,
    height: String,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                width: "1024".into(),
                height: "768".into(),
            },
            Task::none(),
        )
    }

    fn resize_window(width: f32, height: f32) -> impl Fn(window::Id) -> Task<Message> {
        move |window_id: window::Id| {
            println!("Calling resize_window");
            window::resize::<Message>(window_id, Size::new(width, height))
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateWidth(w) => self.width = w,
            Message::UpdateHeight(h) => self.height = h,
            Message::ResizeWindow => {
                let width = self.width.parse::<f32>().unwrap_or(1024.0);
                let height = self.height.parse::<f32>().unwrap_or(768.0);
                println!("Resizing window to {}x{}", width, height);
                return window::get_latest().and_then(Self::resize_window(width, height));
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        row![
            text_input("Width", &self.width).on_input(Message::UpdateWidth),
            text_input("Height", &self.height).on_input(Message::UpdateHeight),
            button("Resize window").on_press(Message::ResizeWindow),
        ]
        .into()
    }
}
