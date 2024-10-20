// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 示例打印不同类型的消息
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello, world!")));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 0, 0));
}

