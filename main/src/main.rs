
// 修复错误
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    // 只能修改这一行
    // 这里的 f 是不可变的，所以不能修改 f.data,因为当结构体中某个字段的所有权被移动后，整个结构体的所有权也被移动了
    //println!("{},{:?}",f.data, f);
} 