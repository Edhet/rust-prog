use std::io;
use rand::{self, Rng};

const SIZE: usize = 3;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const END: &str = "\x1b[0m";

#[derive(PartialEq)]
enum TurnResult {Won, Lost, Tie, Continue}

pub fn play() -> io::Result<()> {
    let width = SIZE;
    let height = SIZE;
    let mut table = vec![vec![0; width]; height];

    println!(r"
    _   _      _             _             
   | | (_)    | |           | |            
   | |_ _  ___| |_ __ _  ___| |_ ___   ___ 
   | __| |/ __| __/ _` |/ __| __/ _ \ / _ \
   | |_| | (__| || (_| | (__| || (_) |  __/
    \__|_|\___|\__\__,_|\___|\__\___/ \___|
                                           
    1A | 1B | 1C
    2A | 2B | 2C
    3A | 3B | 3C
    
    Insert the cell you want to put the 'X'
    ");
    
    loop {

        // PLAYER TURN
        loop {
            let mut string_input: String = String::new();
            io::stdin().read_line(&mut string_input)?;
            let string_input = string_input.to_uppercase();
            let string_input = string_input.trim();
            
            if string_input.len() == 2 {
                let number_input = numerical_player_input(string_input);
                
                if table[number_input[0]][number_input[1]] == 0 {
                    table[number_input[0]][number_input[1]] = 1;
                    break;
                }
                else {
                    println!("This cell has already been marked.");
                }
            }
            else {
                println!("Wrong format...");
            }
        }

        if continue_game(&table) == false {print_table(&table); break;}

        // AI TURN
        loop {
            let line = rand::thread_rng().gen_range(0..=2);
            let row = rand::thread_rng().gen_range(0..=2);
            
            if table[line][row] == 0 {
                table[line][row] = -1;
                break;
            }
        }

        if continue_game(&table) == false {print_table(&table); break;}
        print_table(&table);
    }
    Ok(())
}

fn numerical_player_input(input: &str) -> Vec<usize> {
    let numbers = vec!['1', '2', '3'];
    let letters = vec!['A', 'B', 'C'];
    let mut number_input = vec![];

    let mut iter = 0;
    for char in input.chars() {
        if iter == 0 {
            if numbers.contains(&char) {
                number_input.push(char.to_string().parse::<usize>().unwrap() - 1)
            }
            else {
                println!("Wrong format...");
            }
        }
        else if iter == 1 {
            if letters.contains(&char) {
                match char {
                    'A' => number_input.push(0),
                    'B' => number_input.push(1),
                    'C' => number_input.push(2),
                    _ => continue
                }
            }
            else {
                println!("Wrong format...");
            } 
        }
        else {
            break;
        }
        iter += 1;
    }
    return number_input;
}

fn print_table(table: &Vec<Vec<i32>>) {
	print!("\n");
    for line in table {
        let mut tabs = 0; 
        for entry in line {
            match entry {
                0 => print!("   "),
                1 => print!(" {GREEN}X{END} "),
                -1 => print!(" {RED}O{END} "),
                _ => continue
            }
            if tabs < 2 {
                print!("{YELLOW}|{END}");
                tabs += 1;
            }
        }
        print!("\n");
    }
}

fn continue_game(table: &Vec<Vec<i32>>) -> bool {
    let turn_result = check_turn_result(&table);

    match turn_result {
        TurnResult::Won => {println!("\n{GREEN}You won!{END}");
                return false;},
        TurnResult::Lost => {println!("\n{RED}You lost!{END}");
                return false},
        TurnResult::Tie => {println!("\n{YELLOW}Tie!{END}");
                return false},
        TurnResult::Continue => return true,
    }
}

fn check_turn_result(table: &Vec<Vec<i32>>) -> TurnResult {
    let mut result: TurnResult;
    
    result = check_tie(&table);
    if result != TurnResult::Continue {
        return result;
    }
    result = check_lines(&table);
    if result != TurnResult::Continue {
        return result;
    }
    result = check_rows(&table);
    if result != TurnResult::Continue {
        return result;
    }
    result = check_diagonals(&table);
    return result;
}

fn check_tie (table: &Vec<Vec<i32>>) -> TurnResult {
    for line in table {
		if line.contains(&0) {
            return TurnResult::Continue;
		}
	}
    return TurnResult::Tie;
}

fn check_lines (table: &Vec<Vec<i32>>) -> TurnResult {
    for line in 0..=2 {
        let mut sum = 0;
        for row in 0..=2 {
            sum += table[line][row]; 
            match sum {
                3 => return TurnResult::Won,
                -3 => return TurnResult::Lost,
                _ => ()
            }
        }
    }
    return TurnResult::Continue;
}

fn check_rows (table: &Vec<Vec<i32>>) -> TurnResult {
    for row in 0..=2 {
        let mut sum = 0;
        for line in 0..=2 {
            sum += table[line][row]; 
            match sum {
                3 => return TurnResult::Won,
                -3 => return TurnResult::Lost,
                _ => ()
            }
        }
    }
    return TurnResult::Continue;
}

fn check_diagonals (table: &Vec<Vec<i32>>) -> TurnResult {
    if table[1][1] != 0 {
        let mut sum = 0;
        sum += table[1][1] + table[0][2] + table[2][0];
        match sum {
            3 => return TurnResult::Won,
            -3 => return TurnResult::Lost,
            _ => ()
        }
        let mut sum = 0;
        sum += table[1][1] + table[0][0] + table[2][2];
        match sum {
            3 => return TurnResult::Won,
            -3 => return TurnResult::Lost,
            _ => ()
        }
    }
    return TurnResult::Continue;
}