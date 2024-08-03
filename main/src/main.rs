
fn main() {
    // 1. 通过引用来避免所有权转移
    // let x = String::from("hello, world");
    // let y = &x;
    // println!("{},{}",x,y);

    // 2.深拷贝
    let x = String::from("hello, world");
    let y = x.clone();  
    println!("{},{}",x,y);
}