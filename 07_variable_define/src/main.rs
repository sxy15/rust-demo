fn main() {
    // 定义变量的格式为：let 变量名:变量类型 = 值;
    // 命名规范：变量名必须以字母或者下划线开头，后面可以跟字母、数字、下划线
    // 变量名不能使用 Rust 关键字
    // 变量名区分大小写

    // 可变变量 mut

    // 定义一个变量，变量名为 a，类型为 i32，值为 10
    let a:i32 = 10;

    println!("The value of a is: {}", a);

    // 定义一个可变变量，变量名为 b，类型为 i32，值为 20
    let mut b:i32 = 20;

    println!("The value of b is: {}", b);

    // 修改变量 b 的值
    b = 30;

    println!("The value of b is: {}", b);

}
