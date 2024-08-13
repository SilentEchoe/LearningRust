
// 填空
fn drink(beverage: &str) {
   if beverage == "lemonade" {
      println!("Success!");
      // 实现下面的代码
      panic!("error")
   }

   println!("Exercise Failed if printing out this line!");
}

fn main() {
   drink("lemonade");

   println!("Exercise Failed if printing out this line!");
}