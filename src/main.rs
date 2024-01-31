// fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1 > s2 {
//         s1
//     } else {
//         s2
//     }
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");

//     let result = max(&s1, &s2);

//     println!("The longest string is {}", result);
// }

pub fn strtok<'a, 'b>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');

    println!("hello is: {}, s1 is: {}, s is: {}", hello, s1, s);
}