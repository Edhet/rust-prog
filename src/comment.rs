use std::io::{self, Write};
use std::fs::{OpenOptions, File, self};
use std::env;

#[derive(Debug)]
struct Comment {
    username: String,
    rating: i32,
}

pub fn call() -> io::Result<()> {

    let mut file_path = env::current_dir().unwrap();
    file_path.push("Database.txt");

    let mut database = OpenOptions::new()
    .create(true)
    .append(true)
    .open(file_path)?;
    
    println!("To insert a comment type 'Insert'\nTo see the average rating of comments type 'Average'\nTo exit the program type 'Exit'\n");
    
    loop {
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)?;
        let input = input.as_str().trim();

        match input {
            "Insert" => take_input(&mut database)?,
            "Average" => average()?,
            "Exit" => break,
            _ => { println!("This option is not available.\n"); continue;},
        };
    }
    Ok(())
}

fn take_input(file: &mut File) -> io::Result<()> {


    let mut user_comment = Comment {
        username: String::new(),
        rating: 0,
    };

    for entry in 1..=2 {
        let mut buffer = String::new();

        match entry {
            1 => {println!("\nInsert username:");
                io::stdin()
                .read_line(&mut buffer)?;        
                user_comment.username.push_str(&buffer.trim()); },  
            2 => {println!("Insert you rating:");      
                io::stdin()
                .read_line(&mut buffer)?;
                user_comment.rating = buffer.trim().parse::<i32>().unwrap_or(0)},
            _ => break
        }
    }

    file.write_all(format!("Name: '{}'\n", user_comment.username).as_bytes())?;
    file.write_all(format!("Rating: '{}'\n", user_comment.rating).as_bytes())?;

    Ok(())
}

fn average() -> io::Result<()> {

    let digit = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut file_path = env::current_dir().unwrap();
    file_path.push("Database.txt");
    let contents = fs::read_to_string(file_path)?;

    let mut str_list = vec![];

    let mut buffer_string = String::new();
    for char in contents.chars(){
        if digit.contains(&char) == true {
            buffer_string.push(char);
        }
        if char == '\n' && buffer_string.len() != 0 {
            str_list.push(buffer_string);
            buffer_string = String::new();
        }
    }

    let entries = str_list.len() as i32;

    let mut sum = 0;
    for entry in str_list {
        let number: i32 = entry.trim().parse().unwrap();
        sum += number;
    }
    
    println!("\nThe average of ratings is: ~{}\n", sum / entries);
    Ok(())
}