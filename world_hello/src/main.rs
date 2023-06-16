fn main() {
    // 变量遮蔽
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // y = 5
        _ => println!("Default case, x = {:?}", x),
    }

    match y {
        1..=10 => println!("one through ten"), // 1..=10 匹配 1-10
        _ => println!("something else"),
    }
}
