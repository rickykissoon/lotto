use std::io;

#[derive(Debug)]
struct Guesses {
    guesses: Vec<i32>,
}
trait Push {
    fn push(&mut self, number: i32);
    fn len(&self) -> usize;
}

impl Push for Guesses {

    fn push(&mut self, number: i32) {
        let length = self.len();

        if length >= 5 {
            println!("cannot add anymore values to vector");
        }

        println!("length of this vec is: {}", length);
        self.push(number);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

fn main() {
    println!("Welcome to the Terminal Lottery");

    let mut guesses: Guesses;

    println!("vector guesses contains: {:#?}", guesses);
    println!("vector guesses is size: {:#?}", guesses.len());

    // Add values to guesses
    guesses.push(4);
    guesses.push(50);
    guesses.push(45);
    guesses.push(3);
    println!("vector guesses contains: {:#?}", guesses);
    println!("vector guesses is size: {:#?}", guesses.len());
    println!("run add on guesses");
   // guesses.add(42);


    guesses.push(13);
    println!("vector guesses contains: {:#?}", guesses);
    println!("vector guesses is size: {:#?}", guesses.len());
    println!("run add on guesses");
    guesses.push(59);


    println!("vector guesses contains: {:#?}", guesses);
    println!("vector guesses is size: {:#?}", guesses.len());
    
    /*loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim() {
            "exit" => {
                println!("exiting");
                break;
            },
            &_ => println!("unrecognized input..."),
        }
    }*/
}


/*
    To play user first selects 5 random numbers from 1 to 69
    Code runs and randomly selects 5 numbers
    If all user numbers match all randomly generated numbers the user wins
*/