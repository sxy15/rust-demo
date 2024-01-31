// 斐波那契数列


fn swap(a: &mut i32, b: &mut i32) {
    let t = *a + *b;
    *a = *b;
    *b = t;
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        swap(&mut a, &mut b);
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    while i < n {
        swap(&mut a, &mut b);
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let mut a = 1;
    let mut b = 1;

    for _i in 2..n {
        swap(&mut a, &mut b);

        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);

    // cargo run -- https://www.rust-lang.org rust.md
    for arg in std::env::args() {
        println!("{}", arg); 
        // https://www.rust-lang.org
        // rust.md
    }
}