use std::fmt;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quit => write!(f, "quit"),
            Self::Move{ x, y} => write!(f, "move to ({},{})", x, y),
            Self::Write(message) => write!(f, "write {}", message),
            Self::ChangeColor(r,g,b) => write!(f, "change to RGB({},{},{})", r, g, b),
        }
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 100 },
        Message::Write(String::from("uooooooooooooooooo")),
        Message::ChangeColor(255, 255, 0),
        Message::Quit,
    ];

    for message in messages.iter() {
        println!("{}", message);
        if let Message::Quit = message {
            return;
        }
    }
}
