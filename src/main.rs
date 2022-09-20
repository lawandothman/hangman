use std::{fs, io, process::Command};

use rand::seq::IteratorRandom;

struct Game {
    secret_word: String,
    discovered_letters: String,
    lives: u8,
    status: String,
}

enum UserGuessStatus {
    AlreadyDiscovered,
    CorrectGuess,
    FalseGuess,
}

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

    let mut hidden_secret_word = hide_secret_word(&game.secret_word, &game.discovered_letters);
    println!("The random word is {} ", &game.secret_word);

    loop {
        update_screen(&game, &hidden_secret_word);
        println!("Type your guess: ");

        let user_guess = get_user_guess();

        if validate_user_guess(user_guess) {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();
            match check_user_guess(&game, guess_lower) {
                UserGuessStatus::CorrectGuess => {
                    game.discovered_letters.push(guess_lower);
                    let status = String::from("Correct!");
                    game.status = status;
                    hidden_secret_word =
                        hide_secret_word(&game.secret_word, &game.discovered_letters);

                    if !hidden_secret_word.contains('_') {
                        game.status = String::from("You WON!");
                        update_screen(&game, &hidden_secret_word);
                        break;
                    }
                }
                UserGuessStatus::FalseGuess => {
                    game.discovered_letters.push(guess_lower);
                    game.lives -= 1;

                    if game.lives == 0 {
                        game.status = String::from("You lost!!");
                    } else {
                        game.status = String::from("Wrong guess!");
                    }
                }
                UserGuessStatus::AlreadyDiscovered => {
                    game.status = String::from("You already guessed this")
                }
            }
        } else {
            let status = String::from("This is not a letter!");
            game.status = status;
        }
    }
}

fn get_secret_word() -> Result<String, io::Error> {
    let file = fs::read_to_string("words.txt")?;
    let mut rng = rand::thread_rng();
    let secret_word = file.lines().choose(&mut rng).unwrap().to_string();
    Ok(secret_word)
}

fn hide_secret_word(input: &str, discovered: &str) -> String {
    let mut result = String::new();
    for (_, c) in input.chars().enumerate() {
        result.push(if c == ' ' {
            c
        } else if discovered.contains(c) {
            c
        } else {
            '_'
        });
        result.push(' ');
    }
    result.push('\n');
    result
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

fn check_user_guess(game: &Game, user_guess: char) -> UserGuessStatus {
    if game.discovered_letters.contains(user_guess) {
        return UserGuessStatus::AlreadyDiscovered;
    }

    if !game.secret_word.contains(user_guess) {
        return UserGuessStatus::FalseGuess;
    }

    UserGuessStatus::CorrectGuess
}

fn update_screen(game: &Game, secret_word: &String) {
    clear();
    println!("HANGMAN: CAN YOU GUESS THE WORD?");
    println!(
        "Lives: {}. Discovered Letters: {}",
        game.lives, game.discovered_letters
    );
    println!("{}", secret_word);
    println!("{}", game.status);
}

fn clear() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
