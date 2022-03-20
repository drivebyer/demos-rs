// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

///////////////////////////////////////////
//                解题思路
// 这道题倒是纠正了我之前的一个错误观点。
// 在同一个 scope 中，Rust 并没有禁止一个以上的 mutable reference
// Rust 禁止的是：在一个 mutable reference 的声明与使用之间
// 出现另外一个 mutable reference。
///////////////////////////////////////////
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
