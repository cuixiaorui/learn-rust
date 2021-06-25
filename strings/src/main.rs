fn main() {
    // 新建字符串
    // let mut s = String::new();

    // 通过 to_string 来创建
    // 得到的都是 String 类型
    // let data = "initial contents";
    // 方式一
    // let s = data.to_string();
    // 方式二
    // let s = "initial contents".to_string();
    // 方式三
    // let s = String::from("initial contents");

    // 更新字符串
    // 使用 push_str
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // 这里的 s2 是不会获取到所有权的
    // 所以在下面还是可以使用的，通过 println! 来打印
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // push 附加字符串
    // let mut s = String::from("lo");
    // 必须是 char 类型，而在 rust 中 char 类型必须是用单引号来声明
    // s.push('l');

    // 使用 + 运算符拼接字符串
    // + 其实是内部调用了 add 函数
    // self 这个其实是用得 String 自身，所以会触发所有权，后续就不可以使用了
    // fn add(self, s: &str) -> String
    // let s1 = String::from("Hello");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // // s1 被移动了，现在不可以使用了
    // println!("s1:{}", s1);
    // // s2 用得是引用，所以现在还是可以使用的
    // println!("s2:{}", s2);

    // 使用 format!
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // 使用上和 println! 一样，但是它会生成一个 String
    // 拼接起来会更方便
    // let s = format!("{}-{}-{}", s1, s2, s3);

    // 在 rust 中不可以使用索引来获取字符串
    // 因为内部实现的原因导致的，具体可以看 https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html#%E4%BB%80%E4%B9%88%E6%98%AF%E5%AD%97%E7%AC%A6%E4%B8%B2

    // 遍历字符串的方法
    // 使用 chars 
    // for c in "hello world".chars() {
    //     println!("{}", c);
    // }

    // 遍历字节
    // for c in "hello world".bytes() {
    //     println!("{}", c);
    // }
}
