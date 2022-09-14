# Rust Programs

This repository is made to keep track (and backup) my progress in development of simple software using the Rust language.

> Most projects don't work properly on windows CMD or PowerShell out of the box and require manual change in the source code. (Like Ansi Escape codes and Emojis)

### How to compile those projects?

To compile, first you need to have the rust compiler installed, then you can clone this repo and create a main.rs on the 'src' folder, import the project you want to compile and call it's public function, like the following.

    mod project_name;

    fn main() {
    project_name::public_function();
    }

Then you can just compile it by running cargo on the folder

    cargo run -r

## snake.rs

Snake game that captures user input through device queries.

## tictactoe.rs

Tic Tac Toe with enemy "AI".

## minesweeper.rs

A minesweeper game on a 6x6 to 9x9 grid.

## hangman.rs

Simple hangman game with hardcoded words.

## cat_tamagotchi.rs

Tamagotchi clone in 'turns' for practicing usage of enums and structs.

## comment.rs

Reads input from user and stores it in a .txt on the executing folder.

## calculator_cli.rs

CLI calculator with a primitive string processing.
