use rand::seq::SliceRandom;
use std::io;

mod constants;

fn main() {
    println!("{} {}",  constants::HANGMAN[6], constants::TITLE);
    let words = constants::generate_words();
    let mut chances = 2;
    let word = words.choose(&mut rand::thread_rng()).unwrap().to_string();
    let limit = if word.len() <= 6 {word.len()} else {6};
    println!("The word is \x1b[93m{}\x1b[0m",
             word[0..1].to_string() +
             &"_ ".repeat(word.len()-1).to_string());

    loop {
        if chances > limit {
            println!("\x1b[31mYou ran out of chances!\x1b[0m");
            println!("The word was \x1b[36m\x1b[1m{}\x1b[0m", word);
            break;
        }
        println!("{}\n", constants::HANGMAN[7-chances]);
       
        let mut guess = String::new();
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        guess = guess.trim().to_ascii_lowercase();
        chances += 1;
        if guess == word {
            println!("\x1b[32m\x1b[1mYou won!\x1b[0m");
            break;
        } else if chances <= limit {
            println!("\nThat's a wrong guess!\nYou have \x1b[35m\x1b[1m{}\x1b[0m chances left
                     \nhint: ", limit+1 - chances);
            let hint = word[..chances-1].to_string() +
                (&"_ "[..].repeat(word.len()+1-chances));


            println!("\x1b[33m{}\x1b[0m", hint);
        }

        
    }
}
