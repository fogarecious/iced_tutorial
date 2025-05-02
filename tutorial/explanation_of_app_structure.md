# Explanation of the App Structure

> **Info:** On 0.13.1, the Sandbox trait has been dropped.

The application consists of defining a struct, which represents the application itself, and a Message enum for the message passing. The application struct should have two basic implementations: `update` and `view`. Optionally, a `new` implementation can also be provided. More on that later.

See below an example:

```rust
#[derive(Debug, Clone)]
enum Message {
    // The messages that are going to mutate the app state
}

struct MyApp {
    // Some fields to control the app state
}

impl MyApp {
    fn update(&mut self, message: Message) {
        // Handle state mutations based on the message passed
    }

    fn view(&self) -> Element<Message> {
        // Returns the UI of the application
    }
}
```

:arrow_right: Next: [Application Lifecycle](./application_lifecycle.md)

:blue_book: Back: [Table of contents](./../README.md)
