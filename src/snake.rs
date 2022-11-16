use std::{io, thread, time::{Duration, Instant}, ops::Add};
use rand::{self, Rng};
use device_query::{DeviceQuery, DeviceState};

const SIZE: usize = 16;
const INDEX_SIZE: i32 = (SIZE as i32) - 1;

const INPUT_HEARING_TIME: u64 = 180;
const NAME_SCREEN_TIME: u64 = 500;

enum Event {HitWall, HitYou, Walked, WalkedOnApple}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: i32,
    y: i32
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Snake {
    position: Vector2D,
    old_positions: Vec<Vector2D>,
    direction: Vector2D,
    old_direction: Vector2D,
    apples: usize
}

pub fn play() -> io::Result<()> {
    let mut player = Snake {
        position: {Vector2D{x: 0, y: 0}},
        old_positions: vec![{Vector2D{x: 0, y: 0}}],
        direction: {Vector2D{x: 1, y: 0}}, 
        old_direction: {Vector2D{x: 0, y: 0}},
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
    thread::sleep(Duration::from_millis(NAME_SCREEN_TIME));

    loop {
        print_map(&map);
        println!("\nApples: {}", player.apples);
        
        let time_counter = Instant::now();
        loop {
            match player_input(&device) {
                Some(new_direction) => {
                    if new_direction != player.direction {
                        player.old_direction = player.direction
                    }
                    player.direction = new_direction
                },
                None => ()
            }
            if time_counter.elapsed() >= Duration::from_millis(INPUT_HEARING_TIME) {
                break;
            }
        }
        drop(time_counter);

        let mut next_position = player.position + player.direction;

        if player.old_positions.len() > 0 {
            let lastest_index = player.old_positions.len() -1 as usize;
            if player.old_positions[lastest_index] == next_position  {
                next_position = player.position + player.old_direction;
            }
        }

        match check_next_position(&map, next_position) {
            Event::HitWall => {
                println!("\nHit a wall!");   
                break;
            }
            Event::HitYou => {
                println!("\nHit yourself!");   
                break;
            }
            Event::Walked => {
                player.old_positions.push(player.position);
                player.position = next_position;
            },
            Event::WalkedOnApple => {
                player.apples += 1;
                player.old_positions.push(player.position);
                player.position = next_position;
            }
        }

        map[player.old_positions[0].y as usize][player.old_positions[0].x as usize] = '🟩';
        map[player.position.y as usize][player.position.x as usize] = '🐍';

        while player.old_positions.len() > player.apples {
            player.old_positions.remove(0);
        }
        
        match generate_apple(&map) {
            None => (),
            Some(location) => {
                map[location.y as usize][location.x as usize] = '🍎'
            }
        }    
    }
    Ok(())
}

fn check_next_position(map: &Vec<Vec<char>>, next_position: Vector2D) -> Event {

    if next_position.x < 0 || next_position.x > INDEX_SIZE|| next_position.y < 0 || next_position.y > INDEX_SIZE {
        return Event::HitWall;
    }
    else if map[next_position.y as usize][next_position.x as usize] == '🐍'{
        return Event::HitYou;
    }
    else {
        if map[next_position.y as usize][next_position.x as usize] == '🍎' {
            return Event::WalkedOnApple;
        }
        return Event::Walked;
    }
}

fn player_input(device: &DeviceState) -> Option<Vector2D> {
    let key_presses = device.get_keys().clone();
    if key_presses.len() > 0 {
        match key_presses[0].to_string().chars().next().unwrap() {
        'W' => {return Some(Vector2D {x: 0, y: -1});},
        'A' => {return Some(Vector2D {x: -1, y: 0});},
        'S' => {return Some(Vector2D {x: 0, y: 1});},
        'D' => {return Some(Vector2D {x: 1, y: 0});},
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
    loop {
        let randon_location = Vector2D {
            x: rand::thread_rng().gen_range(0..SIZE) as i32, 
            y: rand::thread_rng().gen_range(0..SIZE) as i32
        };
        if map[randon_location.y as usize][randon_location.x as usize] != '🐍' {
            return Some(randon_location);
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for line in map.iter() {
        for char in line {
            print!(" {} ", char);
        }
        println!();
    }
}