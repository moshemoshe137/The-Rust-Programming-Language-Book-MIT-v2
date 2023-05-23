#![allow(unused_variables)]
pub fn main() {
    // this works fine
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    // let v: Vec<String> = vec![String::from("Hello world")];
    // let s_ref: &String = &v[0];
    // let s: String = *s_ref;
    // //    Compiling fixing-ownership-errors v0.1.0 (C:\Users\mrubi\Projects-git\The Rust Programming Language Book MIT v2\Chapter 4\4.3-fixing-ownership-errors)
    // // error[E0507]: cannot move out of `*s_ref` which is behind a shared reference
    // //   --> src\fixing_an_unsafe_program_copying_vs_moving_out_of_a_collection.rs:10:21
    // //    |
    // // 10 |     let s: String = *s_ref;
    // //    |                     ^^^^^^ move occurs because `*s_ref` has type `String`, which does not implement the `Copy` trait
    // //    |
    // // help: consider removing the dereference here
    // //    |
    // // 10 -     let s: String = *s_ref;
    // // 10 +     let s: String = s_ref;
    // //    |
    // //
    // // For more information about this error, try `rustc --explain E0507`.
    // // error: could not compile `fixing-ownership-errors` due to previous error

    // just don't try to take ownership
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    // or clone to get separate ownership
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push_str("!!!");
    println!("{s}");

    // or use something like `Vec::remove`
    let mut s: String = v.remove(0);
    s.push_str("/\\/\\/\\/\\/");
    println!("{s}");
    assert!(v.len() == 0);
}
