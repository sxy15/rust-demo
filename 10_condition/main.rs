fn main() {
  let total = 100.0;

  if total > 100.0 {
    println!("Total is greater than 100");
  } else if total < 100.0 {
    println!("Total is less than 100");
  } else {
    println!("Total is equal to 100");
  }

  if total == 100.0 {
    println!("Total is equal to 100");
  }

  if total != 100.0 {
    println!("Total is not equal to 100");
  }

  if total >= 80.0 && total <= 100.0 {
    println!("Total is between 80 and 100");
  }

  let code = "10010";
  let choose = match code {
    "10010" => "Jakarta",
    "10020" => "Bandung",
    "10030" => "Surabaya",
    _ => "Unknown"
  };

  println!("Choose: {}", choose);
}