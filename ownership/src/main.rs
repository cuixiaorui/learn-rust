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
// fn main() {
//     // 使用 clone 的话，那么这 s1 和 s2 都会有值了
//     // 因为这里的数据是 clone 了一份
//     let s1 = String::from("hi, ni hao ya ");
//     let s2 = s1.clone();

//     println!("s1: {}; s2: {}", s1, s2);
// }

// 所有权与函数

// fn main() {
//     let s = String::from("hello");

//     // s 进入到函数内就会所有权转移（这里其实就是引用类型的话，会有所有权转移的操作）
//     // 所有权转移就是为了安全
//     // 然后在这个函数后面在访问 s 的时候， s 就实效了
//     takes_ownership(s);
//     // throw error
//     // println!("{}", s);

//     let x = 5;
//     // x 是 copy 的进入 makes_copy 以后 依然有效

//     makes_copy(x);
//     // 这里访问一点问题都没有
//     println!("{}", x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// 返回值与作用域

// fn main() {
//     let s1 = gives_ownership();
//     // 这里是可以访问 s1 的，因为 gives_ownership 这个函数把 some_string return 出来了
//     println!("{}", s1);

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);
//     // 这里 s3 和 s2 一样
//     println!("{}", s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string // 返回出去
// }

// 场景
// 想把引用类型的变量给函数，然后后续还想继续使用
// 而且还想返回多个值
// 一下的形式过于麻烦和形式主义
// 这个也是后续我们引入 引用的原因
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    // 直接使用的元组形式来返回多个值
    // 但是太过于麻烦

    (s, length)
}
