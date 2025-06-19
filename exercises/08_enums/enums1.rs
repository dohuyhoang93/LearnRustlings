#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32),
    Move(String),
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit(bool),
}

fn main() {
    let message = Message::Move("Recycle".to_string());
    match message {
        Message::Resize(size) => println!("{:?}", size),
        Message::Move(position) => println!(" The Message moved to {:?}", position),
        Message::Echo(text) => println!("{:?}", text),
        Message::ChangeColor(r, g, b) => println!("{:?} {:?} {:?}",r, g, b),
        Message::Quit(status) => println!("{:?}", status),
    }

}
