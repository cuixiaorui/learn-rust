fn main() {
    // 标量类型 scalar
    // 整型
    // u 开头的是无符号整数
    // i 开头的是有符号的整数
    // 默认的是有符号的 i32
    const n: u32 = 190000000;
    println!("this is a n value : {}", n);

    // 浮点型
    const f: f32 = 2.32;
    println!("float value : {}", f);

    // 布尔值
    const b: bool = true;
    println!("bool value : {}", b);

    // 复合类型
    // 元组(一个或者多个类型的值混合在一起)
    // 必须用 ()
    let tup: (i32, f64, u8) = (500, 4.2, 1);
    // 解构
    let (x, y, z) = tup;

    println!("tuples The value of y is: {}", y);
    // 索引也是从 0 开始的
    println!("tuples first value: {}", tup.0);
    println!("tuples second value: {}", tup.1);



    // 数组
    // 变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
    let arr=[3;5];
    println!("arr [0] : {}", arr[0]);


}
