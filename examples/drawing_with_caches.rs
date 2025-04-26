use iced::{
    Alignment, Color, Length, Point, Rectangle, Renderer, Task, Theme, Vector, mouse,
    widget::{
        Canvas,
        canvas::{Cache, Geometry, Path, Program, Stroke},
        column,
    },
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run_with(MyApp::new)
}

#[derive(Clone, Debug)]
enum Message {}

#[derive(Default)]
struct MyApp {
    cache: Cache,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                cache: Cache::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            "A Canvas",
            Canvas::new(self).width(Length::Fill).height(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

impl<Message> Program<Message> for MyApp {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.0, 0.2, 0.4));

            frame.fill(
                &Path::circle(frame.center(), frame.width().min(frame.height()) / 4.0),
                Color::from_rgb(0.6, 0.8, 1.0),
            );

            frame.stroke(
                &Path::line(
                    frame.center() + Vector::new(-250.0, 100.0),
                    frame.center() + Vector::new(250.0, -100.0),
                ),
                Stroke {
                    style: Color::WHITE.into(),
                    width: 50.0,
                    ..Default::default()
                },
            );
        });

        vec![geometry]
    }
}
