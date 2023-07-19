pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{} by 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("微博用户{} 发表内容{}", self.username, self.content)
    }
}

fn main() {
    let post = Post {
        title: "Rust标题".to_string(),
        author: "Rust作者".to_string(),
        content: "Rust内容".to_string(),
    };

    let weibo = Weibo {
        username: "微博用户".to_string(),
        content: "微博内容".to_string(),
    };

    println!("post: {}", post.summarize());
    println!("weibo: {}", weibo.summarize());
}
