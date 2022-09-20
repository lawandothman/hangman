use std::process::Command;

pub struct Game {
    pub secret_word: String,
    pub discovered_letters: String,
    pub lives: u8,
    pub status: String,
}

impl Game {
    pub fn update_status(&mut self, status: String) {
        self.status = status
    }
    pub fn update_screen(&self, secret_word: &String) {
        clear();
        println!(
            r"
     _
    | |
    | |__   __ _ _ __   __ _ _ __ ___   __ _ _ __
    | '_ \ / _` | '_ \ / _` | '_ ` _ \ / _` | '_ \
    | | | | (_| | | | | (_| | | | | | | (_| | | | |
    |_| |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                        __/ |
                       |___/"
        );
        println!("\nCAN YOU GUESS THE WORD?");
        print_hangman(self);
        println!("Lives: {}", self.lives);
        println!("Discovered Letters: {}", self.discovered_letters);
        println!("{}", secret_word);
        println!("{}", self.status);
    }
    pub fn check_user_guess(&self, user_guess: char) -> UserGuessStatus {
        if self.discovered_letters.contains(user_guess) {
            return UserGuessStatus::AlreadyDiscovered;
        }

        if !self.secret_word.contains(user_guess) {
            return UserGuessStatus::WrongGuess;
        }

        UserGuessStatus::CorrectGuess
    }
    pub fn hide_secret_word(&self, discovered: &str) -> String {
        let mut result = String::new();
        for (_, c) in self.secret_word.chars().enumerate() {
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
}

fn clear() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn print_hangman(game: &Game) {
    match game.lives {
        0 => {
            println!(
                r"
            +---+
            |   |
            O   |
           /|\  |
           / \  |
                |
        =========
            "
            )
        }
        1 => {
            println!(
                r"
            +---+
            |   |
            O   |
           /|\  |
           /    |
                |
        =========
            "
            )
        }
        2 => {
            println!(
                r"
            +---+
            |   |
            O   |
           /|\  |
                |
                |
        =========
            "
            )
        }
        3 => {
            println!(
                "
            +---+
            |   |
            O   |
           /|   |
                |
                |
        =========
            "
            )
        }
        4 => {
            println!(
                r"
            +---+
            |   |
            O   |
            |   |
                |
                |
        =========
        "
            )
        }
        5 => {
            println!(
                "
            +---+
            |   |
            O   |
                |
                |
                |
        =========
            "
            )
        }
        6 => {
            println!(
                r"
            +---+
            |   |
                |
                |
                |
                |
        =========
            "
            )
        }
        _ => {
            panic!("This shouldn't be possible")
        }
    }
}
pub enum UserGuessStatus {
    AlreadyDiscovered,
    CorrectGuess,
    WrongGuess,
}
