// 引用的规则
// 让我们概括一下之前对引用的讨论：

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。

// fn main() {
//     let s1 = String::from("hello");

//     // & 表示引用
//     // 可以使用，但是不会获取所有权
//     // &s1 让我们创建一个指向 s1 的引用，但是不拥有他
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}", s1, len)
// }

// fn calculate_length(s: &String) -> usize {
//     // 将获取引用作为函数参数叫做 借用
//     // 你只能用，不可以修改
//     s.len()
// }

// 引用不可以外部修改
// 只可以用 ， 不可以修改(也是保障安全)

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(s: &String) {
//     // 会编译失败，因为 s 这个引用只可以借用
//     // 不可以被修改
//     s.push_str(",world");
// }

// 可变引用
// fn main() {
//     // 加上关键字 mut
//     // 和可变的变量一样
//     // 在所有需要调用的地方都需要加上 mut
//     // 个人理解就是在使用上让你感觉麻烦，让你去思考如何使用不变量
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}", s);
// }

// // 也只能接受一个可变引用
// fn change(s: &mut String) {
//     // 会编译失败，因为 s 这个引用只可以借用
//     // 不可以被修改
//     s.push_str(",world");
// }

// 只可以有一个可变的引用
// fn main() {
//     // 在特定作用域下，只可以有一个可变的引用

//     let mut s = String::from("hello");

//     // 只可以有一个可变的引用
//     // let r1 = &mut s;
//     // println!("{}", r1);

//     // 这里会报错
//     // 不可以超过一个
//     // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

//     // 两个或更多指针同时访问同一数据。
//     // 至少有一个指针被用来写入数据。
//     // 没有同步数据访问的机制。
//     // let r1 = &mut s;
//     // let r2 = &mut s;

//     // println!("{}, {}", r1, r2)
// }

// 悬垂引用
// 指向的内存可能已经被分配给其它持有者
// 在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

fn main() {
    let reference_to_nothing = dangle();
}

// 悬垂引用的出现
// fn dangle() -> &String {
//     //  dangle 返回一个字符串的引用

//     // s 是一个新字符串
//     let s = String::from("hello");

//     // 返回了 s 的引用
//     &s
//     // 这里 s 离开作用域并被丢弃。其内存被释放。
// }

// 解决方案
// 直接把 s 给 return 出去
// 所有权被移动出去，所以没有值被释放。
fn dangle() -> String {
    let s = String::from("hello");

    s
}
