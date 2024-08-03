
fn main() {
    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    let mut s1 = s.clone();

    s1.push_str("world")
}