fn main() {
    const THREE_HOURS_IN_SECONDS: u64 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let c = 5;

    let c = c + 1;

    {
        let c = c * 2;
        println!("The value of c in the inner scope is: {c}");
    }
    println!("The value of c is: {c}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}")
}
