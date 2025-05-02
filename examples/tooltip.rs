use iced::widget::{Tooltip, button, column, tooltip, tooltip::Position};

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            Tooltip::new(
                button("Mouseover to see the tooltip"),
                "Construct from struct",
                Position::Right
            ),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Construct from function",
                Position::Right
            ),
            tooltip(
                button("Mouseover to see the tooltip"),
                "With padding",
                Position::Right
            )
            .padding(20),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Far away from the widget",
                Position::Right
            )
            .gap(50),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Parts out of the window are clipped",
                Position::Right
            )
            .snap_within_viewport(false),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Follow the cursor",
                Position::FollowCursor
            )
        ]
        .into()
    }
}
