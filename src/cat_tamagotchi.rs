use std::io;

enum Actions {Idle, Eat, Sleep, Play}

struct Cat {
    name: String,
    hunger: i8,
    tiredness: i8,
    boredom: i8,
    state: Actions,
}

impl Default for Cat {
    fn default() -> Self {
        Cat { name: String::new(), hunger: 0, tiredness: 0, boredom: 0, state: Actions::Idle }
    }
}

impl Cat {
    fn rename(&mut self, name: String) -> Result<(), ()> {
        if name.len() != 0 {
            self.name = name;
            return Ok(());
        }
        return Err(());
    }

    fn draw(&self) {
        match self.state {
            Actions::Sleep => { println!(r"
             |\'/-..--.
            / _ _   ,  ;
            `~=`Y'~_<._/
            <`-....__.'
            "); },  
            Actions::Play => { println!(r"
             _._     _,-'- -`-._
            (,-.`._,'(       |\`-/|
                `-.-' \ )-`( , o o)
                      `-    \`_`'''-
                "); },
            Actions::Eat =>{ println!(r"           
                          |\__/,|   (`\    
                          |o o  |__ _) )    
                        _.( T   )  `  /     
                n n._   ((_ `^--' /_<  \    
              <'' _ ]=- `` `-'(((/  (((/   
                "); },
            Actions::Idle =>{ println!(r"
             /\ ___ /\
            (  o   o  ) 
             \  >#<  /
             /       \  
            /         \       ^
           |           |     //
            \         /    //
             ///  ///   --
                "); },

        }
        println!(r"
        Name: {}    
        Hunger: {}  Tiredness: {}   Boredom: {}
        ", self.name.trim(), self.hunger, self.tiredness, self.boredom);
    }

    fn cat_do(&mut self, arg: &str) {
        match arg {
        "sleep" =>  self.state = Actions::Sleep,
        "play" =>  self.state = Actions::Play,
        "eat" =>  self.state = Actions::Eat,
        _ =>  self.state = Actions::Idle,   
        }
    }

    fn update (&mut self) {
        self.tiredness += 2;
        self.boredom += 2;
        self.hunger += 1;
        
        match self.state {
            Actions::Sleep => self.tiredness -= 4,  
            Actions::Play => self.boredom -= 4,
            Actions::Eat => self.hunger -= 4,
            Actions::Idle => {
                self.tiredness -= 1;
                self.boredom-= 1;
                self.hunger -= 1;
            }
        }

        self.tiredness = self.tiredness.clamp(0, 10);
        self.boredom = self.boredom.clamp(0, 10);
        self.hunger = self.hunger.clamp(0, 10);

        self.draw();
    }

    fn flee (&self) -> bool {
        if self.boredom == 10 || self.hunger == 10 || self.tiredness == 10 {
            return true;
        }
        return false;
    }
}

pub fn play() -> io::Result<()> {
    let actions = vec!["play", "eat", "sleep", "idle", "exit"];
    let mut cat = Cat::default();

    println!(r"
        Welcome to Cat Tamagotchi!
        What's the name of your cat?
    ");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        match cat.rename(input) {
            Ok(()) => break,
            Err(()) => println!("Invalid name.")
        }
    }
    
    cat.draw();
    println!(r"
        You can take the following actions:
            play, eat, sleep, idle and exit
    ");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.to_lowercase();
        let input = input.trim();

        if actions.contains(&input) {
            if input == "exit" {
                break;
            }
            else {
                cat.cat_do(input);
                cat.update();
            }
        }
        else {
            println!("The input does not correspond to an action.");
        }

        if cat.flee() {
            println!("Your cat fleed...");
            break;
        }
    }
    Ok(())
}