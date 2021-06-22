// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn server_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {

//     // absolute path -> 绝对路径
//     // hosting 是私有的，不可以直接使用
//     // crate::front_of_house::hosting::add_to_waitlist()

//     // relative path -> 相对路径
//     // hosting 是私有的，不可以直接使用
//     // front_of_house::hosting::add_to_waitlist()

//     // 解决方案就是在函数上或者是 mod 上给个 pub ,设置成 public 公开的就可以了
// }

// 使用 super 起始的相对路径
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // 我们还可以使用 super 开头来构建从父模块开始的相对路径
//         // 可以使用 super 来获取到 parent
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// 创建公有的结构体和枚举
// 结构体默然都是私有的 ，
// 如果你想要外部可访问的话，需要 pub
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         // 必须提供一个 函数，才可以给内部的 struct 赋值
//         // 这就是有了封装的能力了
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // 这边不可以修改， 这个 seasonal_fruit 是私有的
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// enum 是个默认就 public 的
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     // 直接就可以访问
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 可以使用 use 来简化导入的操作
// 这里是绝对路径的方式导入的
// use crate::front_of_house::hosting;
// 相对路径导入方式
// 注意：导入一个函数的时候要导入其模块而不是直接把这个函数给导入进来
// 像下面这样
// 这是为了可以清晰的区分出这个函数是出自哪个模块的
// use front_of_house::hosting::add_to_waitlist
// 如果说导出的是 引入结构体、枚举和其他项时，习惯是指定它们的完整路径
// HashMap 就是完整的路径
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// use front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// 如果遇到导入的模块里面同名的话，那么需要写到模块那一层
// 为了好区分
// use std::fmt;
// use std::io;
// 还可以使用 as 给修改名称
// use std::fmt;
// use std::io as IoResult;

// // 这里就是很好的把 Result 做了区分
// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// 使用 pub use 重新导出名称
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // 就是在这里加个 pub
// // 和 js 里面的 export xx from xxx 是一样的效果
// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// 使用外部包
// 和使用本地的包没啥区别
// 也是给它导入，然后使用即可
// use rand::Rng;

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1, 101);
// }

// 嵌套路径来消除大量的 use 行
// 一种简写的导入的方式
// use std::cmp::Ordering;
// use std::io;
// // ---snip---
// use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;

// 简写的方式
// use std::io::{self, Write};

// 通过 glob 运算符将所有的公有定义引入作用域
// 就是全部都给其导入进来
// 但是需要注意的到，这么导入的话，有可能分不清楚有什么名称和在哪里定义的
// use std::collections::*;
