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
fn main() {
    let five = Some(5);
    plus_one(five);
    plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
