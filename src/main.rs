use std::{fs, io};

use hangman::{Game, UserGuessStatus};
use rand::seq::IteratorRandom;

fn main() {
    let secret_word = match get_secret_word() {
        Ok(secret_word) => secret_word,
        Err(e) => panic!("Problem opening file: {}", e),
    };

    let mut game = Game {
        secret_word,
        discovered_letters: String::new(),
        lives: 6,
        status: String::new(),
    };

    let mut hidden_secret_word = game.hide_secret_word(&game.discovered_letters);

    loop {
        game.update_screen(&hidden_secret_word);
        println!("Type your guess: ");

        let user_guess = get_user_guess();

        if validate_user_guess(user_guess) {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();
            match game.check_user_guess(guess_lower) {
                UserGuessStatus::CorrectGuess => {
                    game.discovered_letters.push(guess_lower);
                    game.update_status(String::from("Correct"));
                    hidden_secret_word = game.hide_secret_word(&game.discovered_letters);

                    if !hidden_secret_word.contains('_') {
                        game.update_status(String::from("You WON!"));
                        game.update_screen(&hidden_secret_word);
                        break;
                    }
                }
                UserGuessStatus::WrongGuess => {
                    game.discovered_letters.push(guess_lower);
                    game.lives -= 1;

                    if game.lives == 0 {
                        game.update_status(String::from("You lost!!"));
                        let secret_word = game.hide_secret_word(&game.secret_word);
                        game.update_screen(&secret_word);
                        break;
                    } else {
                        game.update_status(String::from("Wrong guess!"));
                    }
                }
                UserGuessStatus::AlreadyDiscovered => {
                    game.update_status(String::from("You already guessed this"));
                }
            }
        } else {
            game.update_status(String::from("This is not a letter!"));
        }
    }
}

fn get_secret_word() -> Result<String, io::Error> {
    let file = fs::read_to_string("words.txt")?;
    let mut rng = rand::thread_rng();
    let secret_word = file.lines().choose(&mut rng).unwrap().to_string();
    Ok(secret_word)
}

fn get_user_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn validate_user_guess(user_guess: Option<char>) -> bool {
    match user_guess {
        Some(guess) => {
            if !guess.is_alphanumeric() {
                false
            } else {
                true
            }
        }
        None => false,
    }
}
