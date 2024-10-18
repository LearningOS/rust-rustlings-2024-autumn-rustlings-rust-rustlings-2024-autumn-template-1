// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 }, // 定义 Move 变体，包含两个字段 x 和 y
    Echo(String),           // 定义 Echo 变体，包含一个字符串字段
    ChangeColor(i32, i32, i32), // 定义 ChangeColor 变体，包含三个整数字段，表示 RGB 颜色
    Quit,                  // 定义 Quit 变体
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

