#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        // fn summarize(&self) -> String;
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            // String::from("(Read more from ...)")
            format!("(Read more from {}...)", self.summarize_author())
        }
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
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    pub fn notify(item: &impl Summary) {
        println!("Breaking news: {}", item.summarize());
    }
    let tweet = Tweet {
        username: String::from("drones_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Saving Adou"),
        location: String::from("Chang Ban"),
        author: String::from("Zhao Yun"),
        content: String::from("saving Adou in the Chang Ban battle"),
    };
    println!("1 new tweet:{}", tweet.summarize());
    println!("1 new article:{}", article.summarize());
}
