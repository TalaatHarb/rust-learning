pub enum Signal {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub fn describe_signal(signal: Signal) -> String {
    match signal {
        Signal::Quit => String::from("quit"),
        Signal::Move { x, y } => format!("move to {x}, {y}"),
        Signal::Write(text) => format!("write: {text}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Signal, describe_signal};

    #[test]
    fn describes_quit() {
        assert_eq!(describe_signal(Signal::Quit), "quit");
    }

    #[test]
    fn describes_move_coordinates() {
        assert_eq!(
            describe_signal(Signal::Move { x: 3, y: -2 }),
            "move to 3, -2"
        );
    }

    #[test]
    fn describes_write_text() {
        assert_eq!(
            describe_signal(Signal::Write(String::from("hello"))),
            "write: hello"
        );
    }
}
