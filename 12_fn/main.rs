fn main() {

  // function 

  // fn name([param: type, ...]) -> return_type {}

  let res = add(1, 2);
  println!("The value of res is: {}", res);
}

// fn hello() {
//   println!("Hello, world!");
// }

fn add(x: i32, y: i32) -> i32 {
  x + y
}