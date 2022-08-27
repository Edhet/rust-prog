use std::io;
use rand::{self, Rng};

pub fn play() -> io::Result<()> {
    let width = 3;
    let height = 3;
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
                let number_input = player_input(string_input);
                
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

fn continue_game (table: &Vec<Vec<i32>>) -> bool {
	let mut won = false;
    let mut lost = false;
	let mut tie = true;

	for line in table {
		if line.contains(&0) {
			tie = false;
		}
	}

    for line in 0..=2 {
        let mut sum = 0;
        for row in 0..=2 {
            sum += table[line][row]; 
            if sum == 3 {
                won = true;
            }
            if sum == -3 {
                lost = true;
            }
        }
    }
    for row in 0..=2 {
        let mut sum = 0;
        for line in 0..=2 {
            sum += table[line][row]; 
            if sum == 3 {
                won = true;
            }
            if sum == -3 {
                lost = true;
            }
        }
    }
    if table[1][1] != 0 {
        let mut sum = 0;
        sum += table[1][1] + table[0][2] + table[2][0];
        if sum == 3 {
            won = true;
        }
        if sum == -3 {
            lost = true;
        }

        let mut sum = 0;
        sum += table[1][1] + table[0][0] + table[2][2];
        if sum == 3 {
            won = true;
        }
        if sum == -3 {
            lost = true;
        }
    }
	
    if won {
        println!("\nYou won!");
        return false;
    }
    if lost {
        println!("\nYou lost!");
        return false;
    }
	if tie {
		println!("\nTie!");
		return false;
	}
    return true;
}

fn player_input (input: &str) -> Vec<usize> {
    let numbers = vec!['1', '2', '3'];
    let letters = vec!['A', 'B', 'C'];
    let mut number_input = vec![];

    let mut iter = 0;
    for char in input.chars() {
        if iter == 0 {
            if numbers.contains(&char) {
                match char {
                    '1' => number_input.push(0),
                    '2' => number_input.push(1),
                    '3' => number_input.push(2),
                    _ => continue
                }
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

fn print_table (table: &Vec<Vec<i32>>) {
	print!("\n");
    for line in table {
        let mut tabs = 0; 
        for entry in line {
            match entry {
                0 => print!("   "),
                1 => print!(" X "),
                -1 => print!(" O "),
                _ => continue
            }
            if tabs < 2 {
                print!("|");
                tabs += 1;
            }
        }
        print!("\n");
    }
}