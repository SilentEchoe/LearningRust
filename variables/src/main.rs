#[warn(unused_imports)]


struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

// 定义方法
impl Circle   {
    fn new(x:f64,y:f64,radius:f64) -> Circle{
        Circle{
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法
    fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let circle1 = crate::Circle::new(0.0,0.0,2.0);
    println!("{}",circle1.area());
}