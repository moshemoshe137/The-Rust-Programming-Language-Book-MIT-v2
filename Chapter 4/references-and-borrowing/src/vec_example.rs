pub fn main() {
    // let mut vec: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &vec[2];
    // vec.push(4);
    // println!("Third element is {}", *num);
    // //  --> src\vec_example.rs:4:5
    // //   |
    // // 3 |     let num: &i32 = &vec[2];
    // //   |                      --- immutable borrow occurs here
    // // 4 |     vec.push(4);
    // //   |     ^^^^^^^^^^^ mutable borrow occurs here
    // // 5 |     println!("Third element is {}", *num);
    // //   |                                     ---- immutable borrow later used here
    // //
    // // For more information about this error, try `rustc --explain E0502`.
    // // error: could not compile `references-and-borrowing` due to previous error

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    vec.push(4);
}
