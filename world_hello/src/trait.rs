pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@作者{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
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
    println!("1 new weibo: {}", weibo.summarize());
}
