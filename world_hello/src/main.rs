fn main() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
