use std::io;

fn main() {
   let mut number = String::new();
   io::stdin()
       .read_line(&mut number)
       .expect("Failed to get input");
    
   let mut steps: Vec<usize> = Vec::new();
   let mut other_nums: Vec<usize> = Vec::new();

   for (i, e) in number.trim().chars().enumerate() {
       let num = e.to_string().parse::<usize>().unwrap();
       if i % 2 != 0 {
           steps.push(num);
       } else {
           other_nums.push(num);
       }
    }

    steps = steps.iter().map(|x| x * 2).collect();
    steps = steps.iter().map(|x| 
                             if x.to_string().len() > 1 {
                                 x % 10 + 1
                             } else {*x}
                        ).collect();
    steps.extend(other_nums);
    match steps.iter().sum::<usize>() % 10 {
        0 => println!("Valid"),
        _ => println!("Invalid"),
    }
}
