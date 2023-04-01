pub fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    // // Does NOT compile!!!
    // let first = String::from("Ferris");
    // let full = add_suffix(first);
    // println!("{full}, originally {first}"); // first is now used here

    // DOES compile
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
