pub fn main() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));

    let first = &name.0;

    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
