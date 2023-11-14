#[warn(dead_code)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("yuanjie"),
        email: String::from("55"),
        sign_in_count: 1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: String::from("yuanjie"),
    //     email: String::from("anthor"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        username: String::from("yuanjie"),
        email: String::from("55"),
        ..user1
    };

    println!("user1: {:?}", user1);
    println!("user2: {:?}", user2);
}