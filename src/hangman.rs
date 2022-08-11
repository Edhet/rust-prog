use std::{io, i8};
use rand::{self, thread_rng, Rng};

pub fn play() {
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
    let mut playing = true;
    while playing == true {
        let mut input = String::new();

        println!("\n{}", print_word(&word, &inp_chars));
        println!("Lives remaing: {}", lives);

        io::stdin()
        .read_line(&mut input)
        .expect("Error on Input");
        let input = input.as_str().trim();

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
                println!("\nYou Won!\nThe word was {}!", word);
                playing = false;
            }
            if lives == 0 {
                println!("\nYou Lost!\nThe word was {}!", word);
                playing = false;
            }
        }
        else {
            println!("The character inputed is not allowed.");
        }
    }
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