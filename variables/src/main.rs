//定义一个特征
pub trait Summary {
    // 定义一个行为
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

pub struct Post{
    // 定义一个字段
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let post = Post {
        title: String::from("My Post"),
        author: String::from("My Author"),
        content: String::from("My Content"),
    };

    let NewsArticle = NewsArticle {
        headline: String::from("My Headline"),
        location: String::from("My Location"),
        author: String::from("My Author"),
        content: String::from("My Content"),
    };

    println!("post: {}", post.summarize());
    println!("NewsArticle: {}", NewsArticle.summarize());

    notify(&post);

}