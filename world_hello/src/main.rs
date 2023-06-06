fn main() {
    let dire = Direction::North;
    match dire {
        Direction::North => println!("North"),
        Direction::West | Direction::South => println!("North or East"),
    }
}

enum Direction {
    North,
    South,
    West,
}
