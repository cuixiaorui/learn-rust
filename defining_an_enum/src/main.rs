// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(four);
//     route(six);
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);

//     fn route(ip_type: IpAddrKind) {}
// }

// 可以和结构体混合使用
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// 可以给枚举指定类型
// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::"));

//     println!("home ipaddr is {:?}", home);
// }

// 可以给枚举类型指定多个值

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::"));
// }

// 可以嵌套各种各样的类型
fn main() {
    enum Message {
        Quit,                       // 没有关联任何的数据
        Move { x: i32, y: i32 },    // 匿名的结构体
        Write(String),              // 单独的一个 String
        ChangeColor(i32, i32, i32), // 包含 3 个 i32 类型的值
    }

    // 还可以给枚举添加方法
    impl Message {
        // 第一个参数还是接受 self
        fn call(&self) {
            // 可以在这里定义实现

            println!("call ");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call()
}


// TODO
// rust 的里面的 options 枚举类型暂时没有看懂
