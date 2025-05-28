#[derive(Debug)]
enum Direction {
    Up,
    Down
}
 
#[derive(Debug)]
enum UIEvent {
    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char)
}

impl UIEvent {
    fn describe(&self) {
        println!("{:?}", self);
    }
}

use crate::UIEvent::*;

fn call(event : UIEvent) {
    match event {
        ButtonClicked => println!("Button clicked"), // simple match
        Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
        KeyPressed(ch) => { // whole block
            let up_ch = ch.to_uppercase();
            println!("Key pressed: {}", up_ch);
        }
    }
}
// fn main() {
//     let clicked = ButtonClicked;
//     let scroll = Scroll(Direction::Down);
//     let key_pressed = KeyPressed('b');
//     call(clicked);
//     call(scroll);
//     call(key_pressed);
//     let x = 2u8;
 
//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("other number")
//     }
// }

struct Point {
    x : i32,
    y : i32
}
 
fn main() {
 
    let p = Point { x: 0, y: 0 };
 
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}