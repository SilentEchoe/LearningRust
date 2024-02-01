#[warn(unused_imports)]

// 不要修改 main 中的代码
fn main() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (s1 ,s2) = (&t.0 ,&t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

