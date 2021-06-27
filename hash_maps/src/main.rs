use std::collections::HashMap;

fn main() {
    // 新建一个 hash
    // let mut scores = HashMap::new();

    // 通过 insert 来添加数据
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 20);

    // 使用一个元组的 collect 方法来创建
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let init_scores = vec![10, 50];

    // 通过 _ 来占位
    // let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    // 存储引用类型的情况
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // 所有权已经移动到 map 里面了，所以在这里不可以打印了

    // println!("{}", field_name);
    // println!("{}", field_value);
    // 可以看看如果是用引用的话 是不是可以解决这个问题

    // map.insert(&field_name, &field_value);
    // 这样是ok 的
    // println!("{}", field_name);
    // println!("{}", field_value);

    // 访问 hashmap 中的值

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 20);

    // let team_name = String::from("Blue");
    // 这里的 team_name 新创建的也可以获取到值，还是和js 有区别的
    // 暂时可以理解为他们的引用都是一样的把？ 还是获取的值，作为 HashMap 的 key
    // let score = scores.get(&team_name);
    // println!("{:?}", score);

    // 循环 HashMap
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, val) in &scores {
    //     println!("key: {}; val :{}", key, val);
    // }

    // 更新 hashmap 的值
    // 覆盖一个值
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // // 这里就是覆盖了之前的值
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores);

    // 只在key 没有对应值得时候插入
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // 这里的 blue 就不会被覆盖掉
    // 值还是 10
    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V )
        // 这样我们通过引用就可以直接修改掉值了
        *count += 1;
    }

    println!("{:?}", map);
}
