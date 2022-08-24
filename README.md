# Rust Programs

This repository is made to keep track (and backup) my progress in development of simple software using the Rust language.

### How to compile those projects?

To compile, first you need to have the rust compiler installed, then you can clone this repo and create a main.rs on the 'src' folder, import the project you want to compile and call it's public function, like the following.

    mod project_name;

    fn main() {
    project_name::public_function();
    }

Then you can just compile it by running cargo on the folder

    cargo run -r

---

## comment.rs

Reads input from user and stores it in a .txt on the executing folder.

## guessing_game.rs

A number between 1 and 10 is generated randomly, you have to guess it. Part of rust's official tutorial.

## hangman.rs

Simple hangman game with hardcoded words.

## tictactoe.rs

Tic Tac Toe with enemy "AI".

## calculator_cli.rs

CLI calculator with a primitive string processing.

## cat_tamagotchi.rs

Tamagotchi clone in 'turns' for practicing usage of enums and structs.
