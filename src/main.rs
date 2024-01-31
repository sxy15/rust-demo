fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

fn main() { // 只读引用实 现了 Copy trait，也就意味着引用的赋值、传参都会产生新的浅拷贝
    let data = vec![1, 2, 3, 4, 5];
    let data1 = &data;

    println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data1);

    println!("sum of data1: {}", sum(data1));

    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]);
}
