use std::iter::Sum;

pub trait Summary{
    fn summarize(&self) -> String;
}


pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// Post 构造函数实现Summary 特征
impl Summary for Post{
    fn summarize(&self) -> String {
        format!("文章{},作者是{}",self.title,self.author)
    }
}

// 使用特征作为函数参数
pub fn notify(item: &impl Summary){
    println!("Breaking news! {}",item.summarize())
}

fn main() {
    let post = Post{title:"Go语言".to_string(),author:"Go语言".to_string(),content:"Go语言".to_string()};

    // 接口调用
    println!("{}",post.summarize())
}