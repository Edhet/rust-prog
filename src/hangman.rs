use std::{io, i8};
use rand::{self, thread_rng, Rng};

pub fn play() -> io::Result<()> {
    let allowed_chars = vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "a", "s", "d", "f", "g", "h", "j", "k", "l", "z", "x", "c", "v", "b", "n", "m",];

    let words = vec!["macarrao", "salsicha", "batata", "guaravita", "sal", "antofagasta", "peba"];
    let selector = thread_rng().gen_range(0..words.len());
    let word = String::from(words[selector]);

    let mut lives: i8 = 5;

    let mut inp_chars: Vec<char> = Vec::new();
    let mut characters: Vec<char> = Vec::new();

    for letter in word.chars() {
        if characters.contains(&letter) {
            continue;
        }
        else {
            characters.push(letter);
        }
    }

    println!(r"
     _
    | |
    | |__   __ _ _ __   __ _ _ __ ___   __ _ _ __
    | '_ \ / _` | '_ \ / _` | '_ ` _ \ / _` | '_ \
    | | | | (_| | | | | (_| | | | | | | (_| | | | |
    |_| |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                        __/ |
                       |___/  ");
    loop {
        let mut input = String::new();

        println!("\n\x1b[33m{}\x1b[0m", print_word(&word, &inp_chars));
        println!("\x1b[33mLives remaing: {}\x1b[0m", lives);

        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase(); 
        let input = input.as_str();

        if allowed_chars.contains(&input) {
            let input = input.trim().chars();

            for inp_char in input {
                if characters.contains(&inp_char) {
                    if inp_chars.contains(&inp_char){
                        println!("'{}' has already been inputed.", inp_char);
                    }
                    else {
                        inp_chars.push(inp_char);
                    }
                }
                else {
                    lives -= 1;
                }
            }

            if word == print_word(&word, &inp_chars) {
                println!("\n\x1b[32mYou Won!\nThe word was {}!\x1b[0m", word);
                break;
            }
            if lives == 0 {
                println!("\n\x1b[31mYou Lost!\nThe word was '{}'!\x1b[0m", word);
                break;
            }
        }
        else {
            println!("The character inputed is not allowed.");
        }
    }
    Ok(())
}

fn print_word (all: &String, allowed: &Vec<char>) -> String{
    let mut new_string = String::new();

    for letter in all.chars() {
        if allowed.contains(&letter) {
            new_string.push(letter);
        }
        else {
            new_string.push_str("_");
        }
    }

    return new_string;
}