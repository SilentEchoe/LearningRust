
// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: Box<str>) {
    println!("{}",s)
}