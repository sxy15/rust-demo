fn main() {
  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);

  println!("score is {}", score);


  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let filed_name = String::from("Favorite color");
  let filed_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(filed_name, filed_value);

  // filed_name, filed_value 值被移动，哈希map成为所有者

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores);

  // 使用 entry 方法只在键没有对应一个值时插入
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores); // {"Yellow": 50, "Blue": 25}


  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      println!("{:?}", count);
      *count += 1;
  }

  println!("{:?}", map); // {"world": 2, "wonderful": 1, "hello": 1}
}
