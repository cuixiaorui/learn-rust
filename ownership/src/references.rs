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
fn main() {
    // 在特定作用域下，只可以有一个可变的引用

    let mut s = String::from("hello");

    // 只可以有一个可变的引用
    // let r1 = &mut s;
    // println!("{}", r1);

    // 这里会报错
    // 不可以超过一个
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2)
}
