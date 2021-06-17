// 最普通的使用方式
// fn main() {
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter,
//     }

//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => {
//                 println!("Penny");
//                 1
//             }
//             // 尽量使用 => 直接返回其值
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter => 25,
//         }
//     }

//     let penny_value = value_in_cents(Coin::Penny);
//     println!("penny -> {}", penny_value);
// }

// 绑定值得模式
// fn main() {
//     #[derive(Debug)]
//     enum UsState {
//         Alabama,
//         Alaska,
//     }

//     #[derive(Debug)]
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }

//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter(state) => {
//                 println!("State quarter from {:?}!", state);
//                 25
//             }
//         }
//     }

//     let usState = value_in_cents(Coin::Quarter(UsState::Alaska));
//     // output -> Alaska
//     println!("usState {:?}!", UsState::Alaska);
// }

// 结合 option 枚举来使用
// fn main() {
//     let five = Some(5);
//     plus_one(five);
//     plus_one(None);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     // match 必须处理所有情况
//     // 如果这里的 Some 或者 None 的分支有一个没有处理
//     // 那么编译器就会报错
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

// 可以使用通配符来处理不想匹配所有的情况
fn main() {
    let v = 0u8;
    // 这里的 v 的情况太多了
    // 而且我们只需要处理 几个分支就可以了
    // 所有我们可以用 _ => ()
    match v {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }
}
