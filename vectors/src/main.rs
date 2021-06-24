// vector 就和 js 里面的数组一样
fn main() {
    // 新建 vector
    // 在新建的时候如何没有指定值得话， rust 是推导不出来类型的
    // 所以需要我们给指定一个类型，比如下面就指定了 i32 类型
    // let v: Vec<i32> = Vec::new();

    // 通过 vec! 宏来创建
    // 这里编译器自动会推导出是 i32 的类型
    // let v = vec![1, 2, 3];

    // 更新 vector
    // 注意这里必须是 mut ，是一个可变的
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);

    // 读取 vector 的元素
    // let v = vec![1, 2, 3, 4, 5];
    // // 这里获取到的是指针
    // let third = &v[2];
    // println!("The third element is {}", third);

    // // 还可以通过 get 方法获取到值
    // // 通过 get 获取到的也是指针类型
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // 有2个获取值得原因是，他们在获取超出索引时处理方式是不一样的
    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    // println!("The first element is: {}", first);

    // 遍历 vector 中的元素
    // let  v = vec![100, 32, 27];

    // 这里使用 &v 和 v 是一样的

    // 这里是不可以改变
    // for i in v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 27];
    // // 这里是可以改变
    // for i in &mut v {
    //     // 在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
    //     // 这里的 i 是指针
    //     *i += 50;
    //     println!("{}", i);
    // }

    // 使用枚举来存储多种类型
    // 因为 vector 只可以存储指定的类型的值
    // 如果想突破这个限制的话，就可以利用 枚举可以存储多类型的特性来解决

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 这里的类型是枚举类型 SpreadsheetCell
    // 而这个枚举类型就是
    // 下面赋值的时候，我就可以赋值 i32 类型，或者 String 类型，或者是 float 类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 这里获取到的 a 就是 SpreadsheetCell::Int(3)
    let a = row.get(0);
}
