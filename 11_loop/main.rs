fn main() {

  for num in 1..5 {
    println!("The value of num is: {}", num);
  }

  for num in 1..=5 {
    println!("The value of num is: {}", num);
  }

  let study = vec!["rust", "python", "c++"];

  // iter() 不会消耗掉集合
  for name in study.iter() {
    match name {
      &"rust" => println!("Rust is awesome!"),
      _ => println!("Other language"),
    }
  }

  // into_iter() 会消耗掉集合
  for name in study.into_iter() {
    match name {
      "rust" => println!("Rust is awesome!"),
      _ => println!("Other language"),
    }
  }

  let mut study2 = vec!["rust", "python", "c++"];
  for name in study2.iter_mut() {

    *name = match name {
      &mut "rust" => { "rust mut" },
      _ => *name,
    }
  }

  println!("study2: {:?}", study2);

  let mut num = 1;
  while num < 4 {
    println!("num is: {}", num);
    num += 1;
  }

  let mut num2 = 1;
  loop {
    println!("num2 is: {}", num2);
    num2 += 1;
    if num2 == 4 {
      break;
    }
  }
}