use std::io::{self, Write};
use std::fs::{File, self};

#[derive(Debug)]
struct Comment {
    username: String,
    rating: i32,
}

fn main(){
    let mut co_file = File::create("/home/edhet/Downloads/rust-comment/Comments.txt").expect("Error opening file.");

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
                co_file.write_all(user_input.username.as_bytes()).expect("Error writing to file.");},
                
            2 => {user_input.rating = buffer.trim().parse().expect("Error on Int -> Str");
                co_file.write_all(buffer.as_bytes()).expect("Error writing to file.");},
            
            _ => println!("Error"),
        }
        fs::write("Comments.txt", "Adicionou!?");
    }
}