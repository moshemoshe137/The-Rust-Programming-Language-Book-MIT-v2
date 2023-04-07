// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     // ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
//     //    Compiling fixing-ownership-errors v0.1.0 (C:\Users\mrubi\Projects-git\The Rust Programming Language Book MIT v2\Chapter 4\4.3-fixing-ownership-errors)
//     // error[E0596]: cannot borrow `*name` as mutable, as it is behind a `&` reference
//     //  --> src\fixing_an_unsafe_program_not_enough_permissions.rs:3:5
//     //   |
//     // 3 |     name.push(String::from("Esq."));
//     //   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `name` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//     //   |
//     // help: consider changing this to be a mutable reference
//     //   |
//     // 1 | fn stringify_name_with_title(name: &mut Vec<String>) -> String {
//     //   |                                    ~~~~~~~~~~~~~~~~
//     //
//     // For more information about this error, try `rustc --explain E0596`.
//     // error: could not compile `fixing-ownership-errors` due to previous error
// }

// fix solution
// BUT A BAD ONE!
// changes the name variable iplace
pub fn stringify_name_with_title_mut(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// also not ideal! Takes ownership of the arguments which is rare
// makes the input `name` unusable after it's called
pub fn stringify_name_with_title_take_ownership(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

pub fn stringify_name_with_title_clone(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

pub fn stringify_name_with_title_without_clone(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq."); // just add the suffix later
    full
}
