
// 修复所有错误，不要删除任何一行代码
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = format!("{}{}",s1,s2);
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}