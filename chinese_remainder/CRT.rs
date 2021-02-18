use std::io;
use std::collections::HashMap;

fn main() {
    let mut values: HashMap<usize, usize> = HashMap::new();
    println!("Please enter the values in the format `b mod n`");
    println!("Enter `q` to break");

    loop {
        let mut statement = String::new();
        io::stdin()
            .read_line(&mut statement)
            .expect("Failed to read input");
        
        if &statement.trim() == &"q" {break;}

        let mut mods = statement.trim().split("mod");
        let mod_result: usize = mods.next().unwrap().trim().parse().unwrap();
        let mod_value: usize = mods.next().unwrap().trim().parse().unwrap();

        values.entry(mod_value).or_insert(mod_result);
 
    }

    println!("The number is: {}", chinese_remainder(values));
}

fn chinese_remainder(values: HashMap<usize, usize>) -> usize {
    let N: usize = values.keys().product();
    let mut total: usize = 0;

    for (n, Bi) in values {
        let Ni = N / n;
        let Xi = inverse_mod(&Ni, &n);
        total += Bi * Ni * Xi;
    }

    return total % N;
}

fn inverse_mod(Ni: &usize, n: &usize) -> usize {
    let num = Ni % n;
    let mut trial = 0;

    while &trial < Ni {
        if (num * trial) % n == 1 {
            return trial;
        } else {
            trial += 1;
        }
    }
    return 1;
}

