use std::io;

pub enum Actions {
    Idle,
    Eat,
    Sleep,
    Play
}

pub struct Cat {
    pub name: String,
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

    fn normalize (&mut self) {
        if self.boredom > 10 {
            self.boredom = 10;
        }
        if self.tiredness > 10 {
            self.tiredness = 10;
        }
        if self.hunger > 10 {
            self.hunger = 10;
        }

        if self.boredom < 0 {
            self.boredom = 0;
        }
        if self.tiredness < 0 {
            self.tiredness = 0;
        }
        if self.hunger < 0 {
            self.hunger = 0;
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

        self.normalize();
        self.draw();
    }
}

pub fn play() -> io::Result<()> {
    let actions = vec!["play", "eat", "sleep", "idle", "exit"];
    let mut cat = Cat::default();

    println!(r"
        Welcome to Cat Tamagochi!
        What's the name of your cat?
    ");
    loop {
        io::stdin().read_line(&mut cat.name)?;
        if cat.name.len() != 0 {
            break;
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
        let input = input.as_str().trim();

        if actions.contains(&input) {
            if input == "Exit" {
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

        if cat.boredom == 10 || cat.hunger == 10 || cat.tiredness == 10 {
            println!(r"Your cat fleed...");
            break;
        }
    }
    Ok(())
}