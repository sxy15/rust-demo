fn main() {
  // 常量的定义
  // 1. const
  // 2. static
  const MAX_POINTS: u32 = 100_000;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  static MAX_POINTS2: u32 = 100_000;
  println!("The value of MAX_POINTS2 is: {}", MAX_POINTS2);
}