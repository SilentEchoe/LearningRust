use std::string;

fn main() {
    let storage = StorageLayer {
        StorageComponents: vec![Box::new(Mysql {
            Host: String::from("Mysql"),
        })],
    };
    storage.run()
}

// 定义一个存储类型的特征
pub trait Storage {
    // 定义一个链接的行为
    fn Connen(&self);
}

// 定义一个Mysql结构体
pub struct Mysql {
    pub Host: String,
}

//Mysql 实现存储层的连接操作
impl Storage for Mysql {
    fn Connen(&self) {
        println!("{}", "Mysql 连接成功")
    }
}

// 定义一个MongoDB的结构体
pub struct MongoDB {
    pub Host: String,
}

impl Storage for MongoDB {
    fn Connen(&self) {
        println!("{}", "MongoDB 连接成功")
    }
}

//创建一个统一的动态数组存储这些连接

pub struct StorageLayer {
    pub StorageComponents: Vec<Box<dyn Storage>>,
}

impl StorageLayer {
    pub fn run(&self) {
        for component in self.StorageComponents.iter() {
            component.Connen()
        }
    }
}
