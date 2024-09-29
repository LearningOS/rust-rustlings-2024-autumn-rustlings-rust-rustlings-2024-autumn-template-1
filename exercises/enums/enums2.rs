// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    ChangeColor(u32,u32,u32),
    //i32：有符号整数（signed integer）
    //u32：无符号整数（unsigned integer）
    Move{x:i32,y:i32},
    Echo(String)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

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
