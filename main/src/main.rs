// 实现一个Bird 的特征
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // 填空
    let duck = Duck;
    duck.swim();

    // 动态

     let bird = hatch_a_bird( 2);
    // // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // // bird.swim();
    // // 但它依然可以叫唤
     assert_eq!(bird.quack(), "duck duck");
    //
     let bird = hatch_a_bird(1);
    // // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // // bird.fly();
    // // 但它也可以叫唤
     assert_eq!(bird.quack(), "swan swan");

    println!("Success!")
}

// 实现以下函数
fn hatch_a_bird(x: i32) -> Box<dyn Bird>{
    let v = Some(x);
    if v == Some(1) {
        Box::new(Swan{})
    }else {
        Box::new(Duck{})
    }
}
