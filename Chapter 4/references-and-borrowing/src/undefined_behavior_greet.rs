#![allow(unused_variables)]
pub fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}

/*
    The Rust Programming Language Book MIT v2:Chapter 4\references-and-borrowing [chapter_4-Understanding_Ownership]> cargo check
        Checking references-and-borrowing v0.1.0 (C:\Users\mrubi\Projects-git\The Rust Programming Language Book MIT v2\Chapter 4\references-and-borrowing)
    error[E0382]: borrow of moved value: `m1`
     --> src\undefined_behavior_greet:6:30
      |
    3 |     let m1 = String::from("Hello");
      |         -- move occurs because `m1` has type `String`, which does not implement the `Copy` trait
    4 |     let m2 = String::from("world");
    5 |     greet(m1, m2);
      |           -- value moved here
    6 |     let s = format!("{} {}", m1, m2);
      |                              ^^ value borrowed here after move
      |
    note: consider changing this parameter type in function `greet` to borrow instead if owning the value isn't necessary
     --> src\undefined_behavior_greet:9:14
      |
    9 | fn greet(g1: String, g2: String) {
      |    -----     ^^^^^^ this parameter takes ownership of the value
      |    |
      |    in this function
      = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
      |
    5 |     greet(m1.clone(), m2);
      |             ++++++++

    error[E0382]: borrow of moved value: `m2`
     --> src\undefined_behavior_greet:6:34
      |
    4 |     let m2 = String::from("world");
      |         -- move occurs because `m2` has type `String`, which does not implement the `Copy` trait
    5 |     greet(m1, m2);
      |               -- value moved here
    6 |     let s = format!("{} {}", m1, m2);
      |                                  ^^ value borrowed here after move
      |
    note: consider changing this parameter type in function `greet` to borrow instead if owning the value isn't necessary
     --> src\undefined_behavior_greet:9:26
      |
    9 | fn greet(g1: String, g2: String) {
      |    -----                 ^^^^^^ this parameter takes ownership of the value
      |    |
      |    in this function
      = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
      |
    5 |     greet(m1, m2.clone());
      |                 ++++++++

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `references-and-borrowing` due to 2 previous errors

*/
