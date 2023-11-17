use std::path::Component;

// 为UI组件定义一个特性
pub trait Draw {
    fn draw(&self);
}

//只要组件实现了Draw 特征，就可以调用 draw 方法来进行渲染。

// 按钮
pub struct Button {
    pub width: u32,
    pub hight: u32,
    pub label: String,
}

// Button实现 Draw 特征
impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮
        println!("{}", "绘制按钮")
    }
}

struct SelectBox {
    pub width: u32,
    pub hight: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("{}", "绘制SelectBox")
    }
}

// 动态数组存储UI对象

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
