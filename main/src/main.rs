
// 填空
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move{x:1,y:2}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write(String::from( "hello,world")); // 使用 "hello, world!" 来初始化
}