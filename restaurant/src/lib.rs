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
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 直接就可以访问
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
