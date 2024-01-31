#[warn(unused_imports)]

fn main() {
    let storage = StorageLayer {
        storage_components: vec![
            Box::new(Mysql {
                host: String::from("Mysql Address"),
            }),
            Box::new(MongoDB {
                host: String::from("MongoDB Address"),
            }),
        ],
    };
    storage.run()
}

// 定义一个存储类型的特征
pub trait Storage {
    // 定义一个链接的行为
    fn connen(&self);
}

// 定义一个Mysql结构体
pub struct Mysql {
    pub host: String,
}

//Mysql 实现存储层的连接操作
impl Storage for Mysql {
    fn connen(&self) {
        println!("{}", "Mysql 连接成功")
    }
}

// 定义一个MongoDB的结构体
pub struct MongoDB {
    pub host: String,
}

impl Storage for MongoDB {
    fn connen(&self) {
        println!("{}", "MongoDB 连接成功")
    }
}

//创建一个统一的动态数组存储这些连接

pub struct StorageLayer {
    pub storage_components: Vec<Box<dyn Storage>>,
}

impl StorageLayer {
    pub fn run(&self) {
        for component in self.storage_components.iter() {
            component.connen()
        }
    }
}
