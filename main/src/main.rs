// 修复所有的 panic，让代码工作
fn main() {
   assert_eq!("abc".as_bytes(), [97, 98, 99]);

   let v = vec![1, 2, 3,4];
   let ele = v[3];
   let ele = v.get(3).unwrap();

   // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
   let _v = production_rate_per_hour(2);

   divide(15, 0);

   println!("Success!")
}

fn divide(x:u8, y:u8) {
   if y !=0  {
      println!("{}", x / y)
   }
}

fn production_rate_per_hour(speed: u8) -> f64 {
   let cph: u8 = 221;
   match speed {
      1..=4 => ((speed as f64) * (cph as f64) ) as f64,
      5..=8 => (speed * cph) as f64 * 0.9,
      9..=10 => (speed * cph) as f64 * 0.77,
      _ => 0 as f64,
   }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
   (production_rate_per_hour(speed) / 60 as f64) as u32
}