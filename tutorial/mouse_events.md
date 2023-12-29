# Mouse Events

To have more control over our app, we can use [Application](https://docs.iced.rs/iced/application/trait.Application.html) trait, which is a generalization of [Sandbox](https://docs.iced.rs/iced/trait.Sandbox.html) trait.
There are two main differences between [Application](https://docs.iced.rs/iced/application/trait.Application.html) and [Sandbox](https://docs.iced.rs/iced/trait.Sandbox.html).
One thing is the [associated types](https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types).
We have to specify [Executor](https://docs.iced.rs/iced/application/trait.Application.html#associatedtype.Executor), [Theme](https://docs.iced.rs/iced/application/trait.Application.html#associatedtype.Theme) and [Flags](https://docs.iced.rs/iced/application/trait.Application.html#associatedtype.Flags) in addition to [Message](https://docs.iced.rs/iced/trait.Sandbox.html#associatedtype.Message) in [Sandbox](https://docs.iced.rs/iced/trait.Sandbox.html).
Basically, we use the suggested defaults for these associated types.
The other is that we have to return [Command](https://docs.iced.rs/iced/struct.Command.html) in [new](https://docs.iced.rs/iced/application/trait.Application.html#tymethod.new) method and [update](https://docs.iced.rs/iced/application/trait.Application.html#tymethod.update) method.
We just return [Command::none()](https://docs.iced.rs/iced/struct.Command.html#method.none) for both methods.

To capture events, we implement [subscription](https://docs.iced.rs/iced/application/trait.Application.html#method.subscription) method in [Application](https://docs.iced.rs/iced/application/trait.Application.html).
This method returns [Subscription](https://docs.iced.rs/iced/struct.Subscription.html) struct, which allows us to specify how to handle events.
We can use [events_with](https://docs.rs/iced/latest/iced/subscription/fn.events_with.html) function to construct a [Subscription](https://docs.iced.rs/iced/struct.Subscription.html).
The [events_with](https://docs.rs/iced/latest/iced/subscription/fn.events_with.html) function takes a function as its input.
The input function takes two parameters, [Event](https://docs.iced.rs/iced/event/enum.Event.html) and [Status](https://docs.iced.rs/iced/event/enum.Status.html), and returns [Option](https://doc.rust-lang.org/std/option/enum.Option.html)\<`MyAppMessage`>, which means this function is capable of transforming [Event](https://docs.iced.rs/iced/event/enum.Event.html) to `MyAppMessage`.
We then receive the transformed `MyAppMessage` in [update](https://docs.iced.rs/iced/application/trait.Application.html#tymethod.update) method.

In the input function, we only care about ignored events (i.e., events that is not handled by widgets) by checking if [Status](https://docs.iced.rs/iced/widget/canvas/event/enum.Status.html) is [Status::Ignored](https://docs.iced.rs/iced/widget/canvas/event/enum.Status.html#variant.Ignored).
Afterwards, we capture [Event::Mouse(...)](https://docs.iced.rs/iced/enum.Event.html#variant.Mouse) and [Event::Touch(...)](https://docs.iced.rs/iced/enum.Event.html#variant.Touch) and produce messages.

Note: [events_with](https://docs.rs/iced/latest/iced/subscription/fn.events_with.html) will be deprecated.
If you cannot find this function, try using [listen_with](https://docs.iced.rs/iced/event/fn.listen_with.html).

```rust
use iced::{
    event::Status, executor, mouse::Event::CursorMoved, subscription, touch::Event::FingerMoved,
    widget::text, Application, Event, Point, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    PointUpdated(Point),
}

struct MyApp {
    mouse_point: Point,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                mouse_point: Point::ORIGIN,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::PointUpdated(p) => self.mouse_point = p,
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text(format!("{:?}", self.mouse_point)).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events_with(|event, status| match (event, status) {
            (Event::Mouse(CursorMoved { position }), Status::Ignored)
            | (Event::Touch(FingerMoved { position, .. }), Status::Ignored) => {
                Some(MyAppMessage::PointUpdated(position))
            }
            _ => None,
        })
    }
}
```

![Mouse events](./pic/mouse_events.png)
