fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // // this errors:
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using `if` in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");

    // // this errors
    // /*
    //     error[E0308]: `if` and `else` have incompatible types
    //       --> src\main.rs:38:43
    //        |
    //     38 |     let number = if condition { 5 } else {"six"};
    //        |                                 -         ^^^^^ expected integer, found `&str`
    //        |                                 |
    //        |                                 expected because of this
    //
    //     For more information about this error, try `rustc --explain E0308`.
    //     error: could not compile `control-flow` due to previous error
    //  */
    // let number = if condition { 5 } else {"six"};
    // println!("The value of number is {number}");

    // rust has 3 loops: `loop`, `while`, and `for`

    // // an infinite loop:
    // loop {
    //     println!("again!")
    // }

    // Using `loop` to check if something is done
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with `while`
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping Through a Collection with `for`
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // more efficiently:
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }

    // countdown redone with `for`
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");

    let a = [5; 10]; // a = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5]
    println!("{a:?}");
}
