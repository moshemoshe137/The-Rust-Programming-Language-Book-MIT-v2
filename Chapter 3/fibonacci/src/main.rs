fn main() {
    for i in 0..=50 {
        print!("The {}th Fibonacci number is ", i);
        let f = fib_r(i);
        println!("{}", f);
    }
    for i in 0..=50 {
        print!("The {}th Fibonacci number is ", i);
        let f = fib_i(i);
        println!("{}", f);
    }
}

fn fib_r(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fib_r(n - 1) + fib_r(n - 2);
}

fn fib_i(n: usize) -> usize {
    let (mut low_fib, mut high_fib) = (0, 1);
    let mut counter = 0;
    while counter < n {
        let new_highest = low_fib + high_fib;
        let new_lowest = high_fib;

        low_fib = new_lowest;
        high_fib = new_highest;
        counter += 1;
    }
    return low_fib;
}
