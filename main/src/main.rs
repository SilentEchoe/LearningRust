use std::i32;

// 解决代码中的错误和 `panic`
fn main() {
    let v1 = 251_i32 + 8;
    let v2 = i32::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}