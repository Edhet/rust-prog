use std::io::{self, Write};
use std::fs::OpenOptions;
use std::env;

#[derive(Debug)]
struct Comment {
    username: String,
    rating: i32,
}

pub fn call() {

    let mut file_path = env::current_dir().unwrap();
    file_path.push("Database.txt");

    let mut database = OpenOptions::new()
    .create(true)
    .append(true)
    .open(file_path)
    .expect("Error!");
    

    let mut user_input = Comment {
        username: String::new(),
        rating: 0,
    };

    for entry in 1..=2 {
        let mut buffer = String::new();

        io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input.");

        match entry {
            1 => {user_input.username = buffer;
                database.write_all(user_input.username.as_bytes()).expect("Error writing to file.");},
                
            2 => {user_input.rating = buffer.trim().parse().expect("Error on Int -> Str");
                database.write_all(buffer.as_bytes()).expect("Error writing to file.");},
            
            _ => println!("Error on Loop"),
        }
    }
}