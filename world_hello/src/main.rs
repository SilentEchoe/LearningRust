fn main() {
    let s = String::from("hello");
    say_hello(&s);
}

fn say_hello(s: &str) {
    println!("{}", s);
}
