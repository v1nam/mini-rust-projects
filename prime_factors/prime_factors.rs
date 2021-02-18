use std::io;

fn main() {
    let mut factors: Vec<usize> = Vec::new();

    let mut n = 2;
    let mut number = String::new();

    println!("Enter your number: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Problem taking input.");

    let mut number: usize = number.trim().parse().expect("Please enter an integer");

    while number > 1 {
        if number % n == 0 {
            factors.push(n);
            number /= n;
        } else {
            n += 1;
        }
    }

    match factors.len() {
        0 => println!("The number has 0 prime factors"),
        _ => {
            println!("The number has {} prime factor(s)", factors.len());
            println!("{}", factors.iter()
                .map(|x| {x.to_string()})
                .collect::<Vec<String>>()
                .join(", ")
            )
        }
    }
}
