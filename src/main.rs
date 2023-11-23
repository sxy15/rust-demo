fn main() {
    let x = vec![1, 2, 3, 4, 5];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3, 4, 5];

    assert!(equal_to_x(y));
}