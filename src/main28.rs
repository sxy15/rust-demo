use std::fs::File;
// use std::io::ErrorKind;
use std::io;
use std::io::Read;

// // Result is an enum with two variants, Ok and Err
// fn main() {
//     let f = File::open("Cargo.toml");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Problem opening the file: {:?}", error)
//         },
//     };

//     println!("f is {:?}", f); // { fd: 3, path: "/workspaces/rust-demo/Cargo.toml", read: true, write: false }
// }


// fn main() {
//     let f = File::open("Cargo.toml");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             match error.kind() {
//                 ErrorKind::NotFound => match File::create("Cargo.toml") {
//                     Ok(fc) => fc,
//                     Err(e) => panic!("Problem creating the file: {:?}", e),
//                 },
//                 other_error => panic!("Problem opening the file: {:?}", other_error),
//             }
//         },
//     };

//     println!("f is {:?}", f); // { fd: 3, path: "/workspaces/rust-demo/Cargo.toml", read: true, write: false }
// }


// fn main() {
//     let f = File::open("vv.toml").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("vv.toml").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// fn main() {
//     // unwrap: 如果 Result 值是 Ok，unwrap 会返回 Ok 中的值, 如果 Result 是 Err，unwrap 会为我们调用 panic!
//     let f = File::open("vv.toml").unwrap();

//     // expect: 与 unwrap 一样，expect 允许我们选择 panic! 的错误信息
//     let f = File::open("vv.toml").expect("Failed to open vv.toml");
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("vv.toml");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) { // read_to_string 返回一个 Result，所以需要处理
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
// fn main() {
//     let result = read_username_from_file();
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("Cargo.toml")?;

//     let mut s = String::new();
//     f.read_to_string(&mut s)?;

//     Ok(s)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    // ? 运算符会自动将错误值传播给调用者
    File::open("Cargo.toml")?.read_to_string(&mut s)?;

    Ok(s)
}
fn main() {
    let result = read_username_from_file();

    println!("result is {:?}", result);
}