// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.



// 定义 Message 枚举及其变体
enum Message {
    ChangeColor(u8, u8, u8), // 改变颜色
    Echo(String),           // 回显消息
    Move(Point),           // 移动位置
    Quit,                  // 退出
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // 使用 match 语句来处理不同的 message 变体
        match message {
            Message::ChangeColor(r, g, b) => {
                self.change_color((r, g, b)); // 调用 change_color 方法
            }
            Message::Echo(s) => {
                self.echo(s); // 调用 echo 方法
            }
            Message::Move(p) => {
                self.move_position(p); // 调用 move_position 方法
            }
            Message::Quit => {
                self.quit(); // 调用 quit 方法
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        
        // 处理不同的消息
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        // 验证状态是否正确
        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
