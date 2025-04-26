# Drawing With Caches

Previously, we mentioned that [Program](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html) has the method [draw](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html#tymethod.draw) that every time the corresponding [Canvas](https://docs.rs/iced/0.13.1/iced/widget/canvas/struct.Canvas.html) needed to be re-drawn, the method is called.

However, if the shapes of the [Canvas](https://docs.rs/iced/0.13.1/iced/widget/canvas/struct.Canvas.html) remain unchanged, it is not performant to re-draw all these shapes.

What we can do instead, is to store an image of these shapes in a [Cache](https://docs.rs/iced/0.13.1/iced/widget/canvas/type.Cache.html), and draw this cache when the [draw](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html#tymethod.draw) method of [Program](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html) is called.

To do so, we declare a field of type [Cache](https://docs.rs/iced/0.13.1/iced/widget/canvas/type.Cache.html) in our app

```rust
struct MyApp {
    cache: Cache,
}
```

and initialize it.

```rust
fn new() -> (Self, Task<Message>) {
    (
        Self {
            cache: Cache::new(),
        },
        Task::none()
    )
}
```

In the [draw](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html#tymethod.draw) method, instead of creating a new [Frame](https://docs.rs/iced/0.13.1/iced/widget/canvas/type.Frame.html)

```rust
let mut frame = Frame::new(renderer, bounds.size());
// ...
vec![frame.into_geometry()]
```

we use [cache.draw](https://docs.rs/iced/0.13.1/iced/widget/canvas/type.Cache.html#method.draw) to construct the [Geometry](https://docs.rs/iced/0.13.1/iced/widget/canvas/type.Geometry.html).

```rust
let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
    // ...
});

vec![geometry]
```

The closure `|frame| { /* ... */ }` is only called when the dimensions of the `frame` change or the cache is explicitly cleared.

In addition, previously, we implement [Program](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html) for the struct `MyProgram`. But because we need to access the `cache` field of `MyApp`, we have to implement [Program](https://docs.rs/iced/0.13.1/iced/widget/canvas/trait.Program.html) for `MyApp`.

The full code is as follows:

```rust
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
```

![Drawing With Caches](./pic/drawing_with_caches.png)

:arrow_right:  Next: [Drawing Widgets](./drawing_widgets.md)

:blue_book: Back: [Table of contents](./../README.md)
