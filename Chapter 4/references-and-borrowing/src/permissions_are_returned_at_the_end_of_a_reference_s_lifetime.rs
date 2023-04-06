#![allow(dead_code)]
pub fn main() {
    let mut x = 1;
    let y = &x; // start of `y`'s life
    let z = *y; // end of `y`'s life
    x += z;
    println!("x is {x}");
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();

        v[0] = up; // can't update `v` until `c` is "dead"
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
