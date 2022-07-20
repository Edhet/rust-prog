use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn game() {

    let random_number: i32 = rand::thread_rng().gen_range(1..10);
    
    println!("Random number is: {}\n", random_number);

    loop {

        let mut user_input = String::new();
        println!("Please enter your Guess.");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading the input.");

        let user_input: i32 =  user_input.trim().parse().expect("Error");

        match user_input.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("Right answer!");
                break;
            }
        }
    }

    println!("Well done!!");
}

