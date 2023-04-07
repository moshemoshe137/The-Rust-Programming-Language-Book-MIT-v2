// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
//     //    Compiling fixing-ownership-errors v0.1.0 (C:\Users\mrubi\Projects-git\The Rust Programming Language Book MIT v2\Chapter 4\4.3-fixing-ownership-errors)
//     // error[E0106]: missing lifetime specifier
//     //  --> src\fixing_an_unsafe_program_returning_a_reference_to_the_stack.rs:1:25
//     //   |
//     // 1 | fn return_a_string() -> &String {
//     //   |                         ^ expected named lifetime parameter
//     //   |
//     //   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//     // help: consider using the `'static` lifetime
//     //   |
//     // 1 | fn return_a_string() -> &'static String {
//     //   |                          +++++++
//     //
//     // For more information about this error, try `rustc --explain E0106`.
//     // error: could not compile `fixing-ownership-errors` due to previous error
// }

// one fix
pub fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

// second fix
pub fn return_a_string_literal() -> &'static str {
    "Hello world"
}

// third fix
// more on this in chapter 15.4
use std::rc::Rc;
pub fn return_a_string_with_gc() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

// fourth fix
// the caller is responsible for creating space for the string.
pub fn return_a_string_mutable_reference(output: &mut String) {
    output.replace_range(.., "Hello world");
}
