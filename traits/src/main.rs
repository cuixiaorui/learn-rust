// 定义 traits
// 看起来和接口也差不多
// 但是不能为外部类型实现外部 trait

pub trait Summary {
    // 不实现，只给出函数签名
    // fn summarize(&self) -> String;
    // 给一个默认实现
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

fn main() {
    // 为类型实现 trait

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // 实现 summary 给 newsArticle
    // impl Summary for NewsArticle {
    //     // 这里是自己实现了一遍 trait
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }
    // 这里是使用默认实现
    impl Summary for NewsArticle {}

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
    }

    // 调用
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // trait 作为参数
    // 指定参数必须是实现了 Summary trait 的
    // 这里被称之为 impl Trait
    // pub fn notify(item: impl Summary) {
    //     println!("Breaking news! {}", item.summarize())
    // }

    // 这里称之为 impl bound
    // T:Summary 这里是指， 他们只要实现了 Summary trait 即可
    // pub fn notify<T: Summary>(item: impl T) {
    //     println!("Breaking news! {}", item.summarize())
    // }

    // 通过 + 指定多个 trait bound

    // pub fn notify(item: impl Summary + Display) {};
    // pub fn notify<T: Summary + Display>(item: T){}

    // 通过 where 来简化 trait bound
    // fn some_function<T: Display  + Clone , U:Clone + Debug>(t:T,u:U)-> i32{
    // }
    // 变化成下面这样
    // fn some_function<T, U>(t: T, u: U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }

    // 返回实现了 Trait 的类型
    // 关键字就是 impl
    // fn returns_summarizable() -> impl Summary {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }

    // 使用 trait bound 有条件地实现方法
    // 只给实现了指定的 trait 的 struct 写 Fn
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    // 这里就是普通的实现  new 
    // 所有的 Pair 都会有 new 这个 fn
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 而这里就是只给实现了 Display 和 PartialOrd 的 Pair 实现的 fn
    // 换句话说，只有实现了 Display 和 PartialOrd 的 Pair 才有 cmp_display 方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
