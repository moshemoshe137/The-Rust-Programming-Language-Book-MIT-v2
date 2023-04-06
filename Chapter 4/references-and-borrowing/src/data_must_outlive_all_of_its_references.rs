// pub fn main() {
//     // // invalid:
//     // let s = String::from("Hello world");
//     // let s_ref = &s;
//     //
//     // drop(s);
//     // println!("{}", s_ref);
//     let strings = vec![];
//     let default = String::from("default");
//     let s = first_or(&strings, &default);
//     drop(default);
//     println!("{}", s);
// }
//
// fn first(strings: &Vec<String>) -> &String {
//     let s_ref = &strings[0];
//     return s_ref;
// }
//
// fn first_or(strings: &Vec<String>, default: &String) -> &String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }
