#![allow(unused_variables, unused_assignments)]
mod safe;
// mod not_safe;
mod string;

fn main() {
    safe::main();

    //not_safe::main();

    // This is fine:
    let a = 5;
    let mut b = a;
    b += 1;

    // but this is slow/impossible
    // it uses 2 million elements!
    let a = [0; 1_000_000];
    let b = a;
    // thread 'main' has overflowed its stack
    // error: process didn't exit successfully: `target\debug\what-is-ownership.exe`
    // (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)

    // Uses pointers, and the pointed-to data is not copied
    let a = Box::new([0; 1_000_000]);
    let b = a;

    // `a` "owns" this box
    let a = Box::new([0; 1_000_000]);
    // The following statements "moves ownership" to `b`:
    let b = a;

    string::main()
}
