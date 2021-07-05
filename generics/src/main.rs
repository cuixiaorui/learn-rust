// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);

//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         // 这里需要实现一个 std::cmp::PartialOrd，这是一个 trait
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

// 结构体定义中的泛型
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // 这里都是 i32 类型
//     let interger = Point { x: 5, y: 10 };

//     // 这里都是 f64 类型
//     let flost = Point { x: 5.0, y: 10.0 };

//     // 如果 2 个类型不相同的话，那么就会报错
//     let notSame = Point { x: 5, y: 10.0 };
// }

// 可以定义多个泛型
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // 这里就是没问题的
//     let notSame = Point { x: 5, y: 10.0 };
// }

// 枚举定义中的泛型
// enum Option<T> {
//     Some(T),
//     None,
// }

// // 枚举定义多个泛型
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法定义中的泛型
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x);
// }

// 在方法定义多泛型

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.1 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
}
