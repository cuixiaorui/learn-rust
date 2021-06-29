fn main() {
    // 使用 panic! 来主动抛出错误
    // panic!("crash and burn");

    // 使用 panic！的 backtrace 来阅读出错的代码
    // 使用 RUST_BACKTRACE=1 cargo run 运行就可以了
    let v = vec![1, 2, 3];
    v[99];
}
