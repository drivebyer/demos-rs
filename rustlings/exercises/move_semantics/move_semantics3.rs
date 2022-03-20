// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

///////////////////////////////////////////
//                解题思路
// 在题目要求前提下，
// 因为 fill_vec() 函数中需要对容器执行 push 操作，
// 该操作需要容器的可变借用，所以 fill_vec() 需要
// 传入容器的可变借用。
///////////////////////////////////////////
fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
