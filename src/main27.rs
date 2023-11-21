// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);// plus_one 所有权不会转移么？

//     let none = plus_one(None);

//     println!("five: {:?}", five);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }


// fn main() {
//     let v = Some(0u8);

//     if let Some(3) = v {
//         println!("three");
//     } else {
//         println!("others");
//     }
// }

// fn main() {
//     let mut v: Vec<i32> = Vec::new();
//     v.push(1);

//     let mut v = vec![1, 2, 3]; // vec! 宏
//     let third: &i32 = &v[2];

//     println!("third: {}", third);

//     match v.get(2) {
//         Some(third) => println!("third: {}", third),
//         None => println!("None"),
//     }

//     for i in &mut v {
//         *i += 50;
//     }

//     for i in &v {
//         println!("i: {}", i);
//     }
// }

// fn main() {
//     let data = "initial contents";
//     let s = data.to_string();

//     let s = "initial contents".to_string();
    
//     let s = String::from("initial contents");

//     // push_str 切片
//     // push 单个字符

//     let mut s1 = String::from("hello, ");
//     let s2 = "world";
//     s1.push_str(s2);
    
//     println!("s1 is {}", s1);

//     let mut s = String::from("lo");
//     s.push('l');
//     println!("s is {}", s);

//     let s1 = String::from("hello, ");
//     let s2 = String::from("world");
//     let s3 = s1 + &s2; // s1 被移动了，不能继续使用

//     println!("s3 is {}", s3);

//     // format! 宏
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{}-{}-{}", s1, s2, s3);

//     println!("s is {}", s);
// }

use std::collections::HashMap;
fn main() {
    // hashMap K V 类型必须一致
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collect 方法
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores1: HashMap<_, _> = 
        teams.iter().zip(initial_scores.iter()).collect();

    println!("scores1 is {:?}", scores1);

    // get -> Option
    let team_name = String::from("Blue");
    let score = scores1.get(&team_name);
    println!("score is {:?}", score);

    // 遍历
    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

    // 更新
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 覆盖

    println!("scores is {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 不会覆盖

    println!("scores is {:?}", scores);
}