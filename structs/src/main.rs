// fn main() {
//     // 和 js 里面的对象一样
//     // 定义结构体
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active: bool,
//     };

//     // 创建结构体
//     // 不可以改变的
//     let user1 = User {
//         email: String::from("cui_xiaorui@126.com"),
//         username: String::from("cuixiaorui"),
//         active: true,
//         sign_in_count: 1,
//     };

//     // 使用
//     println!("{}", user1.email);

//     let mut user2 = User {
//         email: String::from("cui_xiaorui@126.com"),
//         username: String::from("cuixiaorui"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user2.username = String::from("heihei");

//     println!("可变的 user2 ： {}", user2.username);

//     fn build_user(email: String, username: String) -> User {
//         // User {
//         //     email: email,
//         //     username: username,
//         //     active: true,
//         //     sign_in_count: 1,
//         // }

//         // 简写
//         // 和 js 一样
//         User {
//             email,
//             username,
//             active: true,
//             sign_in_count: 1,
//         }
//     }

//     let heiheihei = build_user(String::from("heiheihei@126.com"), String::from("heiheihei"));
//     println!("heiheihei: {}", heiheihei.email);
// }

// fn main() {
//     // 结构体更新语法
//     struct User {
//         username: String,
//         age: u32,
//     };

//     let user1 = User {
//         username: String::from("xiaohong"),
//         age: 18,
//     };

//     // let user2 = User {
//     //     username: String::from("cui_xiaorui"),
//     //     age: user1.age,
//     // };
//     // 还可以使用 .. 把剩余的都赋值
//     let user2 = User{
//         username: String::from("xiaoli"),
//         ..user1
//     };

//     println!("user2 -> age {}", user2.age);
// }

fn main() {
    // 使用没有命名字段的元组结构体来创建不同的类型
    // 也可以定义与元组（在第三章讨论过）类似的结构体，称为 元组结构体（tuple structs）
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 1, 2);
    let origin = Point(0, 0, 0);

    println!("black {}", black.0);
    println!("black {}", black.1);
}
