/* 移除所有可以消除的生命周期标注 */

fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: i32) -> i32 { x }

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) { self.0 += 1; }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}