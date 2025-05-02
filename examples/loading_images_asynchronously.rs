use iced::{
    Task,
    widget::{Image, button, column, container, image::Handle},
};
use tokio::{fs::File, io::AsyncReadExt};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Loaded(Vec<u8>),
}

#[derive(Default)]
struct MyApp {
    image_handle: Option<Handle>,
    show_container: bool,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                image_handle: None,
                show_container: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.show_container = true;
                return Task::perform(
                    async {
                        let mut file = File::open("../tutorial/pic/ferris.png").await.unwrap();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).await.unwrap();
                        buffer
                    },
                    Message::Loaded,
                );
            }
            Message::Loaded(data) => self.image_handle = Some(Handle::from_bytes(data)),
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Load").on_press(Message::Load),
            if self.show_container {
                match &self.image_handle {
                    Some(h) => container(Image::new(h.clone())),
                    None => container("Loading..."),
                }
            } else {
                container("")
            },
        ]
        .padding(20)
        .into()
    }
}
