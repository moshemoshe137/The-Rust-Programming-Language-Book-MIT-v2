fn main() {
    println!("Hello, world!");

    another_function(5);
    print_label_measurments(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}."); // 4

    let x = five();
    println!("The value of x is {x}");

    let x = plus_one(5);
    println!("The value of x is {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is {x}");
}

fn print_label_measurments(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
