// fn main() {
//     let data = vec![1, 42, 3, 4, 5];

//     let v = 42;

//     if let Some(pos) = find_pos(data, v) {
//         println!("Found {} at position {}", v, pos);
//     }
// }

// fn find_pos(data: Vec<i32>, value: i32) -> Option<usize> {
//     for (i, v) in data.iter().enumerate() {
//         if *v == value {
//             return Some(i);
//         }
//     }
//     None
// }


fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // all ixx uxx usize isize fxx types
    is_copy::<i8>();
    is_copy::<i64>();
    is_copy::<u64>();
    is_copy::<usize>();

    // function (actually fn pointer)
    is_copy::<fn()>();

    // raw pointer
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference
    is_copy::<&String>();
    is_copy::<&Vec<u8>>();

    // array tuple with values which is copy
    is_copy::<[i32; 3]>();
    is_copy::<(i32, i32)>();
}

fn types_not_impl_copy_trait() {
    // unsized or dynamically sized types
    // is_copy::<str>();
    // is_copy::<[i32]>();

    // mutable reference
    // is_copy::<&mut String>();

    // array tuple with values which is not copy
    // is_copy::<[String; 3]>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}