fn main() {
  // &str is a string slice
  let s:&str = "Hello, world!";
  println!("The value of s is: {}", s);
  // String is a growable, heap-allocated data structure
  
  let s1 = String::new();
  println!("The value of s1 is: {}, s1-len: {}", s1, s1.len());

  let mut s2 = String::from("Hello, ");
  println!("The value of s2 is: {}", s2);

  // push push_str 
  s2.push_str("world!");
  println!("The value of s2 is: {}", s2);

  s2.push('!');
  println!("The value of s2 is: {}", s2);

  // replace 
  let res = s2.replace("world", "rust");
  println!("The value of res is: {}", res);

  // to_string as_str
  let s3 = res.as_str();
  println!("The value of s3 is: {}", s3);

  let s4 = s3.to_string();
  println!("The value of s4 is: {}", s4);

  // trim
  let s5 = "  hello rust  ".trim();
  println!("The value of s5 is: {}", s5);

  // split
  let s6 = "hello world".split(" ");
  for i in s6 {
    println!("The value of i is: {}", i);
  }

  // & ?? 
  let s7 = s4 + &s5;
  println!("The value of s7 is: {}", s7);
}