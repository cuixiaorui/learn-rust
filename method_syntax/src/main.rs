struct Rectangle {
    width: u32,
    height: u32,
}

// 这里可以允许有多个 impl
// 比如可以拆分开
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }

    // 神奇的是 静态方法的声明和普通的没啥特殊的，只有在调用上有区别
    // 调用 ： Rectangle::square()
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // 这是拆分开的 impl
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// fn main() {
//     let rect1 = Rectangle{
//         width: 50,
//         height: 80
//     };
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 相当于是静态方法
    let sq = Rectangle::square(3);
    println!("sq.width -> {}", sq.width);
    println!("sq.height -> {}", sq.height);
}
