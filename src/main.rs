use std::{fs, io};

use rand::seq::IteratorRandom;

fn main() {
    let secret_word = match get_secret_word() {
        Ok(secret_word) => secret_word,
        Err(e) => panic!("Problem opening file: {}", e),
    };
    println!("The random word is {} ", secret_word);
    println!("The secret word's length is {}", secret_word.len());
}

fn get_secret_word() -> Result<String, io::Error> {
    let file = fs::read_to_string("words.txt")?;
    let mut rng = rand::thread_rng();
    let secret_word = file.lines().choose(&mut rng).unwrap().to_string();
    Ok(secret_word)
}
