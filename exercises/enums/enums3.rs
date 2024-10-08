// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

// As a first step, you can define enums to compile this code without errors.
// and then create a match expression in `process()`.
// Note that you need to deconstruct some message variants
// in the match expression to get value in the variant.



enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u8,u8,u8),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {    // Type 为 Message 的参数 message
        // TODO: create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))
        // 对 message 进行模式匹配  if message == Message::ChangeColor , 从而直接修改state的值
        match message {
            Message::ChangeColor(r,g,b) => self.change_color((r,g,b)), //结构体实例化方法
            Message::Echo(text) => self.echo(text),  
            Message::Move(point) => self.move_position(point),
            Message::Quit => self.quit(),
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
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);   

        assert_eq!(state.color, (255, 0, 255));   
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
