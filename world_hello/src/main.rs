// 实现下面的泛型函数 sum
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
    let p = Point {
        x: 5,
        y: "hello".to_string(),
    };
    println!("p.x = {}, p.y = {}", p.x, p.y);

    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}
