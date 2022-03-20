// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

///////////////////////////////////////////
//               解题思路
// 第一个打印处需要
// 共享借用 vec0（该打印语句不能修改），所以要求：
//   1. vec0 在此之前不能丢失对底层数据对所有权
//   2. vec0 在此之前不能给出可变借用
//
// 所以，只能通过 clone() 或者 to_vec() 等
// 方式让 vec1 单独拥有一份数据
///////////////////////////////////////////

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
