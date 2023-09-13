fn main() {
  // tuple is a collection of values of different types
  // tuple is fixed length: once declared, they cannot grow or shrink in size

  // tuple with two elements
  let tup:(i32, f64) = (500, 6.4);

  // destructuring
  let (x, y) = tup;

  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);

  println!("The value of tup.0 is: {}", tup.0);
}