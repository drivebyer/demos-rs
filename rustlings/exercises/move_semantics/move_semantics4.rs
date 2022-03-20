// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

///////////////////////////////////////////
//                解题思路
// 根据函数栈调用规约，执行 fill_vec(); 后简要分为 3 步：
// 1. 将 fill_vec 的返回值填充到 main 函数的栈帧
//    预留空间中（返回值空间），此时 fill_vec 中创
//    建的容器所有权已经转移到 main 函数中
// 2. 执行 return 弹栈
// 3. 执行 let mut vec1 = fill_vec() 将返回值
//    赋值给 vec1 变量，此时容器所有权又发生一次转移
//
///////////////////////////////////////////
fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
