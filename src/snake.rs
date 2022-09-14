use std::{io, thread, time::Duration, fmt::Debug};
use rand::{self, Rng};
use device_query::{DeviceQuery, DeviceState};

const SIZE: usize = 16;
const INDEX_SIZE: usize = SIZE - 1;
const GAME_SPEED: u64 = 100;
const START_SPEED: u64 = 500;

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: i32,
    y: i32
}

struct Snake {
    position: Vector2D,
    direction: Vector2D,
    end_positions: Vec<Vector2D>,
    apples: usize
}

pub fn play() -> io::Result<()> {
    let mut player = Snake {
        position: {Vector2D{x: 0, y: 0}},
        direction: {Vector2D{x: 1, y: 0}}, 
        end_positions: vec![{Vector2D{x: 0, y: 0}}],
        apples: 0
    };

    let mut map = vec![vec!['🟩'; SIZE]; SIZE];
    map[player.position.x as usize][player.position.y as usize] = '🐍';

    let device = DeviceState::new();

    println!(r"                                                 
    //   ) )                                     
   ((          __      ___     / ___      ___    
     \\     //   ) ) //   ) ) //\ \     //___) ) 
       ) ) //   / / //   / / //  \ \   //        
((___ / / //   / / ((___( ( //    \ \ ((____   
     
                    Starting... 
    ");
    thread::sleep(Duration::from_millis(START_SPEED));

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print_map(&map);
        println!("\nScore: {}", player.apples);

        thread::sleep(Duration::from_millis(GAME_SPEED));
        match player_input(&device) {
            Some(new_direction) => player.direction = new_direction,
            None => ()
        }

        let next_position = Vector2D {
            x: player.position.x + player.direction.x,
            y: player.position.y + player.direction.y
        };

        if next_position.x < 0 || next_position.x > INDEX_SIZE as i32 || next_position.y < 0 || next_position.y > INDEX_SIZE as i32 {
            println!("Hit a wall!");
            break;
        }
        else if map[next_position.y as usize][next_position.x as usize] == '🐍'{
            println!("Hit yourself!");
            break;
        }
        else {
            if map[next_position.y as usize][next_position.x as usize] == '🍎' {
                player.apples += 1;
            }
            player.end_positions.push(player.position);
            player.position.x = next_position.x;
            player.position.y = next_position.y;
        }

        map[player.end_positions[0].y as usize][player.end_positions[0].x as usize] = '🟩';
        map[player.position.y as usize][player.position.x as usize] = '🐍';

        while player.end_positions.len() > player.apples {
            player.end_positions.remove(0);
        }
        
        match generate_apple(&map) {
            None => (),
            Some(location) => {
                map[location.y as usize][location.x as usize] = '🍎'
            }
        }

        thread::sleep(Duration::from_millis(GAME_SPEED));
        match player_input(&device) {
            Some(new_direction) => player.direction = new_direction,
            None => ()
        }        
    }
    Ok(())
}

fn player_input(device: &DeviceState) -> Option<Vector2D> {
    let mut new_direction = Vector2D {x: 0, y: 0};

    if device.get_keys().len() > 0 {
        match device.get_keys()[0].to_string().chars().next().unwrap() {
        'W' => {new_direction.x = 0; new_direction.y = -1; return Some(new_direction);},
        'A' => {new_direction.x = -1; new_direction.y = 0; return Some(new_direction);},
        'S' => {new_direction.x = 0; new_direction.y = 1; return Some(new_direction);},
        'D' => {new_direction.x = 1; new_direction.y = 0; return Some(new_direction);},
        _ => ()
        }
    }
    return None;
}

fn generate_apple(map: &Vec<Vec<char>>) -> Option<Vector2D> {
    for line in map.iter() {
        if line.contains(&'🍎') {
            return None;
        }
    }
    let randon_location = Vector2D {
        x: rand::thread_rng().gen_range(0..SIZE) as i32, 
        y: rand::thread_rng().gen_range(0..SIZE) as i32
    };
    return Some(randon_location);
}

fn print_map(map: &Vec<Vec<char>>) {
        for line in map.iter() {
            for char in line {
                print!(" {} ", char);
            }
            println!();
        }
}