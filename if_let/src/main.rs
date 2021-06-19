fn main() {
    let v = Some(0u8);
    // match v {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    // 可以用 if let
    // 这里的效果和上面的一样
    // if let Some(3) = v {
    //     println!("three");
    // }

    // 也支持 else 的分支
    if let Some(3) = v {
        println!("three");
    } else {
        println!("not three");
    }
}
