// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;

fn prompt(guesses_left: i32, guess: &str) {
    println!("The word so far is {}", guess);
    println!("You have {} guesses left", guesses_left);
    print!("Please guess a letter: ");
}

fn main() {
    println!("Welcome to cis1905 Hangman!");
    let target_word = "HANGMAN".to_string();
    let mut guessed_word = "-------".to_string();
    let mut guesses_left = 5;

    while guesses_left > 0 && guessed_word != target_word {
        prompt(guesses_left, &guessed_word);
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 1 {
            println!("Please enter a single letter.");
            println!();
            continue;
        }

        if target_word.contains(&guess) && !guessed_word.contains(&guess) {
            for (i, c) in target_word.chars().enumerate() {
                if c.to_string() == guess {
                    guessed_word.replace_range(i..i + 1, &guess)
                }
            }
        } else {
            println!("Sorry, that letter is not in the word");
            guesses_left -= 1;
        }
        println!();
    }

    if guessed_word == target_word {
        println!(
            "Congratulations you guessed the secret word: {}!",
            target_word
        );
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
