
// 修复错误
fn main() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            (10 * n) as f64
        } else {
            println!("数字太大，我们得让它减半");

            (n / 2) as f64
        };

    println!("{} -> {}", n, big_n);
}