// fn main() {
//     println!("Hello, world!");
// }

// 变量作用域
// fn main() {
//     // s 在这里无效, 它尚未声明
//     let s = "hello"; // 从此处起，s 是有效的
//     // 使用 s
//     // s = s + "world!" // 这里是不可以的 s 是不可变的

// } // 此作用域已结束，s 不再有效

// 使用存在 堆上的 String 类型来演示
// fn main() {
//     // s 是可变的
//     let mut s = String::from("hello");

//     s.push_str(",world!");

//     println!("s value is : {}", s)
// }

// fn main() {
//     // 变量的生命周期
//     {
//         let s = String::from("hello"); // 从此处起，s 是有效的

//         // 使用 s
//     } // 不好用了

//     // throw error ，在当前的作用域下找不到 s
//     // 因为 s 已经被释放了
//     println!("s value is {}", s)
// }

// 移动

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     // 这里就是移动的概念
//     // rust 不会让 s1 和 s2 指向同一个堆里面的数据
//     // 这里 s1 就已经失效了
//     // 因为 rust 要避免二次释放
//     // throw error

//     // println!("s1 value: {}", s1)
// }

// 克隆
fn main() {
    // 使用 clone 的话，那么这 s1 和 s2 都会有值了
    // 因为这里的数据是 clone 了一份
    let s1 = String::from("hi, ni hao ya ");
    let s2 = s1.clone();

    println!("s1: {}; s2: {}", s1, s2);
}

// TODO 所有权与函数
