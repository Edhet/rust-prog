use std::io;
use rand::{self, Rng, thread_rng};

pub fn play() {
    let mut table = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut game_on = true;

    println!(r"
    _   _      _             _             
   | | (_)    | |           | |            
   | |_ _  ___| |_ __ _  ___| |_ ___   ___ 
   | __| |/ __| __/ _` |/ __| __/ _ \ / _ \
   | |_| | (__| || (_| | (__| || (_) |  __/
    \__|_|\___|\__\__,_|\___|\__\___/ \___|
                                           ");

    println!("\n1_|_2_|_3\n4_|_5_|_6\n7 | 8 | 9\n Insert the cell you want to put the 'X'\n");
    print_table(&table);
        
    while game_on == true {
        let mut playing: bool = true;
        while playing == true {
            let mut input_buffer: String = String::new();
            
            io::stdin()
            .read_line(&mut input_buffer)
            .expect("Error on input");
            let input_buffer: usize = input_buffer.trim().parse().expect("Error on parsing");

            if table[input_buffer - 1] != 0 {
                println!("This cell have already been taken");
            }
            else {
                table[input_buffer - 1] += 1;
                playing = false;
            }
        }
        game_on = game_check(&table);                                     
        if game_on == false {print_table(&table); break;}

        let mut ia_playing: bool = true;
        while ia_playing == true  {
            let ia_play: usize = thread_rng().gen_range(0..8);

            if table[ia_play] == 0 {
                table[ia_play] += 2;
                ia_playing = false;
            }
        }
        game_on = game_check(&table);
        print_table(&table);      
    }
}

fn game_check (in_table: &Vec<i32>) -> bool {
    
    
    if in_table.contains(&0) {
    }
    else {
        println!("\nTie!");
        return false;
    }

    if in_table[0] & in_table[1] & in_table[2] == 1 ||
    in_table[3] & in_table[4] & in_table[5] == 1 ||
    in_table[6] & in_table[7] & in_table[8] == 1 || 
    in_table[0] & in_table[3] & in_table[6] == 1 ||
    in_table[1] & in_table[4] & in_table[7] == 1 ||
    in_table[2] & in_table[5] & in_table[8] == 1 ||
    in_table[0] & in_table[4] & in_table[8] == 1 ||
    in_table[2] & in_table[4] & in_table[6] == 1 {
        println!("\nPlayer Won!");
        return false;
    }
    else if in_table[0] & in_table[1] & in_table[2] == 2 ||
    in_table[3] & in_table[4] & in_table[5] == 2 ||
    in_table[6] & in_table[7] & in_table[8] == 2 || 
    in_table[0] & in_table[3] & in_table[6] == 2 ||
    in_table[1] & in_table[4] & in_table[7] == 2 ||
    in_table[2] & in_table[5] & in_table[8] == 2 ||
    in_table[0] & in_table[4] & in_table[8] == 2 ||
    in_table[2] & in_table[4] & in_table[6] == 2 {
        println!("\nPlayer Lost");
        return false;
    }
    else {
        return true;
    }
}

fn print_table (in_table: &Vec<i32>) {
    let mut pretty_table: Vec<String> = vec![];

    for entry in in_table {
        let buffer: i32 = *entry;
        if buffer == 0 {
            pretty_table.push(" ".to_string());
        }
        if buffer == 1 {
            pretty_table.push("X".to_string());
        }
        if buffer == 2{
            pretty_table.push("O".to_string());
        }
    }

    println!("\n{}_|_{}_|_{}\n{}_|_{}_|_{}\n{} | {} | {}", 
    pretty_table[0], pretty_table[1], pretty_table[2],
    pretty_table[3], pretty_table[4], pretty_table[5],
    pretty_table[6], pretty_table[7], pretty_table[8]);
}