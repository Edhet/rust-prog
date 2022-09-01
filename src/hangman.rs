use std::{io, i8};
use rand::{self, thread_rng, Rng};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const END: &str = "\x1b[0m";

pub fn play() -> io::Result<()> {
    let allowed_chars = vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "A", "S", "D", "F", "G", "H", "J", "K", "L", "Z", "X", "C", "V", "B", "N", "M"];

    let words = vec!["PASTA", "SAUSAGE", "POTATO", "SODA", "SODIUM", "ANTOFAGASTA", "BRAZIL", "FUNNY", "COOL"];
    let selector = thread_rng().gen_range(0..words.len());
    let word = String::from(words[selector]);

    let mut lives: i8 = 5;

    let mut inp_chars: Vec<char> = Vec::new();
    let characters: Vec<char> = word_characters(&word);

    println!(r"
     _
    | |
    | |__   __ _ _ __   __ _ _ __ ___   __ _ _ __
    | '_ \ / _` | '_ \ / _` | '_ ` _ \ / _` | '_ \
    | | | | (_| | | | | (_| | | | | | | (_| | | | |
    |_| |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                        __/ |
                       |___/  

         Type in letters to guess the word...");
         
    loop {

        println!("\n{YELLOW}{}{END}", print_word(&word, &inp_chars));
        println!("{YELLOW}Lives remaing: {}{END}", lives);

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.to_uppercase(); 
        let input = input.trim();

        println!("{}", input);
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
                println!("\n{GREEN}You Won!\nThe word was {}!\n{END}", word);
                break;
            }
            if lives == 0 {
                println!("\n{RED}You Lost!\nThe word was '{}'!\n{END}", word);
                break;
            }
        }
        else {
            println!("The character inputed is not allowed.");
        }
    }
    Ok(())
}

fn word_characters(word: &String) -> Vec<char> {
    let mut characters = vec![];
    for letter in word.chars() {
        if characters.contains(&letter) {
            continue;
        }
        else {
            characters.push(letter);
        }
    }
    return characters;
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