// 普通的实现
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组的实现
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// // 元组并没有给出元素的名称，所以计算变得更费解了，因为不得不使用索引来获取元组的每一部分：
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 使用结构体来实现
// 和 js 里面的对象是一个意思

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main(){
//     let rect = Rectangle {
//         width: 30,
//         height: 50
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect)
//     );

//     fn area(rectangle: &Rectangle) -> u32{
//         rectangle.width * rectangle.height
//     }

// }

// 打印出 结构体
fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle{
        width: 50,
        height: 50
    };


    println!("${:#?}", rect);
}
