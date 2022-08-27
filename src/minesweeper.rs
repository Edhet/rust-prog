use rand::Rng;
use std::io;

pub fn play() -> io::Result<()> {
    let mut allowed_inputs = vec!['1', '2', '3', '4', '5', '6'];
    
    println!(r"
     __  ___ _                                                            
    /  |/  /(_)____   ___   _____ _      __ ___   ___   ____   ___   _____
   / /|_/ // // __ \ / _ \ / ___/| | /| / // _ \ / _ \ / __ \ / _ \ / ___/
  / /  / // // / / //  __/(__  ) | |/ |/ //  __//  __// /_/ //  __// /    
 /_/  /_//_//_/ /_/ \___//____/  |__/|__/ \___/ \___// .___/ \___//_/     
                                                    /_/  

    How to play:

          1 2 3 4
        1 E E E E
        2 E E E E
        3 E E E E
        4 E E E E

    Insert the cell you want to play by line followed by row, example:
    12 for line 1 and row 2.
    No need to put flags, just avoid the bombs!

    Insert grid size (min is 6, max is 9): 

");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut size = input.trim().parse::<usize>().unwrap_or(6);
    
    size = size.clamp(6, 9);

    if size > 6 {
        for n in 7..=size {
            match n {
                7 => allowed_inputs.push('7'),
                8 => allowed_inputs.push('8'), 
                9 => allowed_inputs.push('9'), 
                _ => continue
            }
        }
    }

    let mut table = vec![vec![0; size]; size];

    for _i in 0..=size * 2{
        let line = rand::thread_rng().gen_range(1..size - 1);
        let row = rand::thread_rng().gen_range(1..size - 1);

        if table[line][row] == 0 {
            table[line][row] = -1;
        }
    }

    print_table(&table, false);
    loop {

        // READ USER INPUT
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let mut number_input = vec![];
        if input.len() == 2 {
            for char in input.chars() {
                if allowed_inputs.contains(&char) {
                    number_input.push(char.to_string().parse::<usize>().unwrap() - 1);
                }
            }
        }

        // OPEN PLAYER INPUTED TILE
        if number_input.len() == 2 {
            if table[number_input[0]][number_input[1]] == -1 {
                println!("\n\x1b[31mYou exploded!\x1b[0m");
                print_table(&table, true);
                break;
            }
            else if table[number_input[0]][number_input[1]] == 1 {
                println!("Already selected that cell");
            }
            else if table[number_input[0]][number_input[1]] == 0 {
                table[number_input[0]][number_input[1]] = 1;
            }
            
            // AUTO-OPEN TILES
            let mut line_i = 0;
            for line in table.clone() {
                let mut row_i = 0;
                for _row in line {
                    if auto_open(line_i, row_i, table.clone()) {
                        table[line_i][row_i] = 1;
                        
                        let mut sub_line_i = 0;
                        for lines in table.clone() {
                            let mut sub_row_i = 0;
                            if line_i == 0 {
                                if sub_line_i == line_i || sub_line_i == line_i + 1 {
                                    for _rows in lines {
                                        if row_i == 0 {
                                            if sub_row_i == row_i || sub_row_i == row_i + 1 {
                                                if table[sub_line_i][sub_row_i] == 0 {
                                                    table[sub_line_i][sub_row_i] = 1;
                                                }
                                            }
                                        }
                                        else if sub_row_i == row_i || sub_row_i == row_i - 1 || sub_row_i == row_i + 1 {
                                            if table[sub_line_i][sub_row_i] == 0 {
                                                table[sub_line_i][sub_row_i] = 1;
                                            }
                                        }
                                        sub_row_i += 1;
                                    }   
                                }
                            }
                            else if sub_line_i == line_i || sub_line_i == line_i - 1 || sub_line_i == line_i + 1 {
                                for _rows in lines {
                                    if row_i == 0 {
                                        if sub_row_i == row_i || sub_row_i == row_i + 1 {
                                            if table[sub_line_i][sub_row_i] == 0 {
                                                table[sub_line_i][sub_row_i] = 1;
                                            }
                                        }
                                    }
                                    else if sub_row_i == row_i || sub_row_i == row_i - 1 || sub_row_i == row_i + 1 {
                                        if table[sub_line_i][sub_row_i] == 0 {
                                            table[sub_line_i][sub_row_i] = 1;
                                        }
                                    }
                                    sub_row_i += 1;
                                }   
                            }
                            sub_line_i += 1;
                        }

                    }
                    row_i += 1;
                }
                line_i += 1;
            }

            // CHECK IF WON GAME
            let mut continue_game = false;
            for line in table.iter() {
                if line.contains(&0) {
                    continue_game = true;
                }
            }
            if !continue_game {
                println!("\n\x1b[32mYou won!\x1b[0m");
                print_table(&table, true);
                break;
            }
            print_table(&table, false);
        }
        else {
            println!("Wrong format...");
        }
    }
    Ok(())
}

fn print_table (table: &Vec<Vec<i32>>, show_bombs: bool) {
    print!("\n");

    let mut line_i = 0;
    for line in table {
        let mut row_i = 0;
        for entry in line {
            match entry {
                -1 => { if show_bombs {print!(" \x1b[31mQ\x1b[0m ");}
                        else {print!(" \x1b[32m*\x1b[0m ");}},
                0 => print!(" \x1b[32m*\x1b[0m "),
                1 => print!(" \x1b[36m{}\x1b[0m ", count_bombs(line_i, row_i, table.clone())),
                _ => continue
            }
            row_i += 1;
        }
        print!("\n");
        line_i += 1;
    }
}

fn count_bombs (line: usize, row: usize, table: Vec<Vec<i32>>) -> i32 {
    let mut close_bombs = 0;

    let mut line_i = 0;
    for lines in table.iter() {
        let mut row_i = 0;

        if line == 0 {
            if line_i == line || line_i == line + 1 {
                for _rows in lines {
                    if row == 0 {
                        if row_i == row || row_i == row + 1 {
                            if table[line_i][row_i] == -1 {
                                close_bombs += 1;
                            }
                        }
                    }
                    else if row_i == row || row_i == row - 1 || row_i == row + 1 {
                        if table[line_i][row_i] == -1 {
                            close_bombs += 1;
                        }
                    }
                    row_i += 1;
                }   
            }
        }
        else if line_i == line || line_i == line - 1 || line_i == line + 1 {
            for _rows in lines {
                if row == 0 {
                    if row_i == row || row_i == row + 1 {
                        if table[line_i][row_i] == -1 {
                            close_bombs += 1;
                        }
                    }
                }
                else if row_i == row || row_i == row - 1 || row_i == row + 1 {
                    if table[line_i][row_i] == -1 {
                        close_bombs += 1;
                    }
                }
                row_i += 1;
            }   
        }
        line_i += 1;
    }
    return close_bombs;
}

fn auto_open (line: usize, row: usize, table: Vec<Vec<i32>>) -> bool {

    if count_bombs(line, row, table.clone()) == 0 {
        let mut line_i = 0;
        for lines in table.iter() {
            let mut row_i = 0;

            if line == 0 {
                if line_i == line || line_i == line + 1 {
                    for _rows in lines {
                        if row == 0 {
                            if row_i == row || row_i == row + 1 {
                                if table[line_i][row_i] == 1 {
                                    return true;
                                }
                            }
                        }
                        else if row_i == row || row_i == row - 1 || row_i == row + 1 {
                            if table[line_i][row_i] == 1 {
                                return true;
                            }
                        }
                        row_i += 1;
                    }   
                }
            }
            else if line_i == line || line_i == line - 1 || line_i == line + 1 {
                for _rows in lines {
                    if row == 0 {
                        if row_i == row || row_i == row + 1 {
                            if table[line_i][row_i] == 1 {
                                return true;
                            }
                        }
                    }
                    else if row_i == row || row_i == row - 1 || row_i == row + 1 {
                        if table[line_i][row_i] == 1 {
                            return true;
                        }
                    }
                    row_i += 1;
                }   
            }
            line_i += 1;
        }
    }
    return false;
}