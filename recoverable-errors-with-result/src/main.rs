use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // let f = File::open("hello.txt");

    // // 处理成功和失败2种情况
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // 匹配不同的错误
    // let f = File::open("hello.txt");

    // let f = match f {
    //     // 这边是可以无限的使用 match 来嵌套处理错误的
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // 失败时 panic 的简写：unwrap 和 expect
    // 使用 match 看起来有点太麻烦了
    // 下面是简化的写法
    // unwrap 如果是 ok 的话，那么直接给我们返回成功的值
    // 如果是 error 的话，自动给我们调用 panic!
    // let f = File::open("hello.txt").unwrap();
    // expect 类似于 unwrap ，但是可以让我们指定错误信息
    // let f = File::open("hello.txt").expect("hello.txt 文件不存在");

    // 传播错误
    // 把错误传播出来，让其他的函数也可以处理错误
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         // 重点在这里，可以把 error 给 return 出去
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     // 这里是调用了句柄，来读取里面的值
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         // 这里可以省略 return 的原因是， 这里是这个函数最好一个表达式了，默认就给我们 return 了
//         Err(e) => Err(e),
//     };

// }

// 上面太麻烦了，rust 提供了 ？ 操作符来简化上面的步骤
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// 可以使用 ？ 来做链式调用
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// 最终版本
// 上面的所有例子都是为了演示 error
// 实际上，平时写代码的时候更多的是用这种方式来实现
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
