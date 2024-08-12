/* 增加合适的生命周期标准，让代码工作 */

// `i32` 的引用必须比 `Borrowed` 活得更久
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
#[derive(Debug)]
struct NamedBorrowed<'b> {
   x: &'b i32,
   y: &'b i32,
}

#[derive(Debug)]
enum Either<'c> {
   Num(i32),
   Ref(&'c i32),
}

fn main() {
   let x = 18;
   let y = 15;

   let single = Borrowed(&x);
   let double = NamedBorrowed { x: &x, y: &y };
   let reference = Either::Ref(&x);
   let number    = Either::Num(y);

   println!("x is borrowed in {:?}", single);
   println!("x and y are borrowed in {:?}", double);
   println!("x is borrowed in {:?}", reference);
   println!("y is *not* borrowed in {:?}", number);
}