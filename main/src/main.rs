
// 填空
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move {x,y} => { // 这里匹配 Message::Move
            assert_eq!(x, 1);
            assert_eq!(y, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0)
        }
        __ => println!("no data in these variants")
    }
}