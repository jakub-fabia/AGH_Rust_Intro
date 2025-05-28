#[derive(Debug)]
enum Message {
    Move{x: i16, y: i16},
    Echo(String),
    ChangeColor(i16, i16, i16),
    Quit
}
 
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

use crate::Message::*;
 
fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
 
    for message in &messages {
        message.call();
    }
}