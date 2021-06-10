// // 没有所有权的类型 slices

// fn main() {
//     //     let s = String::from("hello world");

//     //     //     let slice = &s[0..2];
//     //     let slice = &s[..2];
//     //     println!("first -> {}", slice);
//     //     let slice1 = &s[..];
//     //     println!("{}", slice1);
//     //     let word = first_word(&s);
//     //     println!("{}",word );

//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // 错误!

//     println!("the first word is: {}", word);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn main() {
    // 也可以用于数组
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

}
