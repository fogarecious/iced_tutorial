use iced::{
    Point, Task,
    event::{self, Event, Status},
    mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
    widget::text,
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run_with(MyApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    PointUpdated(Point),
}

#[derive(Default)]
struct MyApp {
    mouse_point: Point,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                mouse_point: Point::ORIGIN,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PointUpdated(p) => self.mouse_point = p,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        text(format!("{:?}", self.mouse_point)).into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, status, _| match (event, status) {
            (Event::Mouse(CursorMoved { position }), Status::Ignored)
            | (Event::Touch(FingerMoved { position, .. }), Status::Ignored) => {
                Some(Message::PointUpdated(position))
            }
            _ => None,
        })
    }
}
