// fn main() {
//     // 默认是不可变的值
//     let x = 5;
//     println!("the value of x is : {}",x);

//     // 不可以被二次赋值
//     x = 15;
//     println!("the value of x is : {}",x);
// }

// 可变的变量 mut 关键字

// fn main(){
//     let mut x = 5;

//     println!("the value of x is : {}",x);

//     // 可以被二次赋值
//     x = 15;
//     println!("the value of x is : {}",x);

// }

// 隐藏
fn main() {
    // 这里是创建了新的 x 变量 
    // 替换了之前的 x 变量
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
