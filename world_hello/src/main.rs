struct Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("arr = {:?}", arr);
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("integer.x = {}", integer.x);

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [4, 5];
    display_array(arr);
}
