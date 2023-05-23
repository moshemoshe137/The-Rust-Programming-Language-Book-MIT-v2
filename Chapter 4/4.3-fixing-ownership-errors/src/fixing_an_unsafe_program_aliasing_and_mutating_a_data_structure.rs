#![allow(dead_code)]
// pub fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
//     //    Compiling fixing-ownership-errors v0.1.0 (C:\Users\mrubi\Projects-git\The Rust Programming Language Book MIT v2\Chapter 4\4.3-fixing-ownership-errors)
//     // error[E0502]: cannot borrow `*dst` as mutable because it is also borrowed as immutable
//     //  --> src\fixing_an_unsafe_program_aliasing_and_mutating_a_data_structure.rs:6:13
//     //   |
//     // 2 |     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     //   |                            ---------- immutable borrow occurs here
//     // ...
//     // 5 |         if s.len() > largest.len() {
//     //   |                      ------------- immutable borrow later used here
//     // 6 |             dst.push(s.clone());
//     //   |             ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
//     //
//     // For more information about this error, try `rustc --explain E0502`.
//     // error: could not compile `fixing-ownership-errors` due to previous error
//
// }

// We could copy/clone stuff
// but it kind of sucks and has performance issues
pub fn add_big_strings_clone_largest(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// do length first, mutate `dst` later
// but creating `to_add` is also a performance hit.
pub fn add_big_strings_dst_afterwards(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();
    dst.extend(to_add);
}

fn add_big_strings_best(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
