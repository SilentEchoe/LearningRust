
// 修复错误，让代码工作
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32>{
    fn distance_from_origin(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let p = Point{x: 5, y: 10};
    println!("{}",p.distance_from_origin())
}