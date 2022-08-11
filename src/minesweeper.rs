use rand::Rng;
use std::io;

pub fn play() {
    let allowed_inputs = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"];

    let mut map = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for _i in 0..=5 {
        let placement = rand::thread_rng().gen_range(0..15);

        if map[placement] != -1 {
            map[placement] = -1;
        }
    }

    println!(r"
     __  ___ _                                                            
    /  |/  /(_)____   ___   _____ _      __ ___   ___   ____   ___   _____
   / /|_/ // // __ \ / _ \ / ___/| | /| / // _ \ / _ \ / __ \ / _ \ / ___/
  / /  / // // / / //  __/(__  ) | |/ |/ //  __//  __// /_/ //  __// /    
 /_/  /_//_//_/ /_/ \___//____/  |__/|__/ \___/ \___// .___/ \___//_/     
                                                    /_/  ");
    println!("Insert the cell number you want to open\nNo need to put flags\n");

    let mut playing = true;
    while playing == true {
        let mut input = String::new();
        let mut input_history = vec![];

        print_map(&map);
        
        io::stdin()
        .read_line(&mut input)
        .expect("Error on input");
        let input = input.as_str().trim();
        input_history.push(input);
        
        if allowed_inputs.contains(&input) == true || input_history.contains(&input) == false {
            let mut input: usize = input.parse().expect("Error parsing");
            input -= 1;

            if map[input] == -1 {
                playing = false;
                println!("\nYou exploded\n");
            }
            else {
                map[input] = 1;
                if check_win(&map) == true {
                    println!("\nYou found all bombs.\n You won!!\n");
                    print_map(&map);
                    playing = false;
                }
            }
        } 
        else {
            println!("Invalid input.");
        }
    }

    fn print_map (map: &Vec<i32>) {
        let mut index = 0;
        let mut newline = 0;

        for entry in map {
            let buffer: i32 = *entry;
            index += 1;
            newline += 1;

            match buffer {
                -1 => print!("💣"),
                0 => print!("▩ "),
                1 => print!("{} ", detect_mines(index, map)),
                _ => continue
            }
            if newline == 4 {
                newline = 0;
                print!("\n");
            }
        }
        print!("\n");
    }

    fn detect_mines (place: usize, map: &Vec<i32>) -> i32 {
        let mut close_mines = 0;
        
        return close_mines;
    }

    fn check_win (map: &Vec<i32>) -> bool {
        let mut end_game = true;

        if map.contains(&0) {
            end_game = false;
        }

        return end_game;
    }
}