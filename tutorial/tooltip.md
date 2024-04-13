# Tooltip

The [Tooltip](https://docs.rs/iced/0.12.1/iced/widget/tooltip/struct.Tooltip.html) widget displays a text when the mouse is over a specified widget.
It has two methods of constructions.
It is able to change styles of the text.
We can add padding around the text inside.
We can also change the space between the tooltip and the target widget.
If the tooltip is allowed to be out of the window, the parts outside are clipped.

```rust
use iced::{
    font::Family,
    widget::{button, column, tooltip, tooltip::Position, Tooltip},
    Font, Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
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
```

![Tooltip](./pic/tooltip.png)

:arrow_right:  Next: [Rule](./rule.md)

:blue_book: Back: [Table of contents](./../README.md)
