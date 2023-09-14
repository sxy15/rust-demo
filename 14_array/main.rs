fn main() {
 let arr1: [&str; 3] = ["a", "b", "c"]; 

  println!("The value of arr1 is: {:?}", arr1);

  let arr2: [&str; 3] = [""; 3];

  println!("The value of arr2 is: {:?}", arr2);

  println!("{}", arr2.len());


  for item in arr1 {
    println!("{}", item);
  }

  // arr1.iter 返回一个迭代器
  for item in arr1.iter() {
    println!("{}", item);
  }
}