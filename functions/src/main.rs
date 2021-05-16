fn main() {
    println!("Hello, world!");
    foo("nihao".to_string());
    println!("five value : {}",five());

}

// 每一个参数必须给一个类型
// 这个使用规范可以用在 ts 中
fn foo(a: String) {
    println!("a value is  {}", a);
}

fn five() -> i32 {
    4 + 1
}
