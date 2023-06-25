fn main() {}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new 是Circle的关联函数,因为它的第一个参数不是self,且 new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例

    fn new(x:f64,y:f64,radius:f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
        
}

}

