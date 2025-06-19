struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit, //Quit la bien the don vị hay unit variant
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        match message {
            Message::Resize {width, height} => println!("Resize: witdth: {}, height: {}", width, height),
            Message::Move (Point {x , y}) => println!("Move {} {}", x, y),
            Message::Echo(s) => println!("echo {}", s),
            Message::ChangeColor(r, g, b) => println!("Change Color {} {} {}", r, g, b),
            Message::Quit => println!("Quit"),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("Hello world!"),
            color: (0, 0, 0),
            quit: true,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 0, y: 0 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(0, 0, 0));
        state.process(Message::Quit);

        assert_eq!(state.width, 0);
        assert_eq!(state.height, 0);
        assert_eq!(state.position.x, 0);
        assert_eq!(state.position.y, 0);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (0, 0, 0));
        assert!(state.quit);
    }
}
