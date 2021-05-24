// 控制流
// if && 循环

// fn main() {
//     println!("Hello, world!");
// }

// branches
// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     // 表达式必须是 boolean 值，不然会报错
//     let number = 7;

//     // ide 直接给出 error 的提示了
//     // 好用，和 ts 类似
//     if number {
//         println!("throw error")
//     }
// }

// fn main() {
//     // 在 let 语句的右侧使用 if (因为 if 是个表达式)
//     // demo1

//     // let condition = true;

//     // let number = if condition { 5 } else { 10 };

//     // println!("number val is : {}", number)

//     // demo2
//     // let condition = true;
//     // // type error
//     // // rust 需要在编译阶段就知道变量的类型
//     // // 不可以动态的获取变量的类型，因为这样会使编译器变的复杂
//     // let number = if condition { 5 } else { "ten" };

//     // println!("number val is :{}", number)

// }

// 循环
// fn main() {
//     // loop
//     // loop 是无限的循环，可以利用 break 来终止
//     let mut count = 1;
//     loop {
//         count += 1;

//         println!("heiheihei");

//         if count == 10 {
//             break;
//         }
//     }
// }

// while 条件循环
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number = number - 1;
//     }

//     println!("LIFTOFF!!!");
// }

// 使用 for 循环遍历
// fn main() {
//     let a = [10, 20, 30, 40];

//     // 用 .iter
//     for element in a.iter() {
//         println!("the value is :{}", element)
//     }
// }

fn main() {
    // 使用 for 来执行特定的次数
    // 不包含 4
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
