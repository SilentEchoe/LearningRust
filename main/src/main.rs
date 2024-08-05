
// 填空
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South  => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        _ => println!("Other"),
    };
}