fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
pub fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
