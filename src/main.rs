use std::io;
use rand::Rng;
use std::{thread, time};

struct Player {
    username: String,
    money: f32,
    games_played: i32,
}

struct Cents(u64);

struct Money {
    amount: i64,
    cents: Cents,
}

/*
    TODO:
    [] handle inproper input (expect number gets str)
    [] show player stats after game end
    [] allow player to bet on matching individual numbers
    [] decrement on money breaks from 0.97 -> 0.96000004 (use decimal types) (impelemt Money for money in Player)
*/

fn main() {
    println!("Welcome to the Terminal Lottery");

    let money = Money {
        amount: 1,
        cents: Cents(100),
    };

    println!("Enter your username: ");
    let mut player = Player {
        username: get_user_input(),
        money: 1.00,
        games_played: 0,
    };

    let mut jackpot: f32 = 0.5;

    display_player_stats(&player);
    
    loop {
        println!("Type 'start' to begin a new game");
        println!("Type 'exit' to quit:");

        let cmd = &*get_user_input();

        match cmd {
            "start" => {
                player.money -= 0.01;
                player.games_played += 1;
            
                println!("");
                println!("=====================");
                println!("New Game");
                println!("Jackpot is now ${jackpot}");
                let mut guesses: [i32; 5] = [0; 5];
            
                display_player_stats(&player);
                display_nonzero_elements(&guesses);
                get_guesses(&mut guesses);
            
                println!("Generating Lotto numbers...");
                let mut lotto: [i32; 5] = [0; 5];
                
                for x in 0..lotto.len() {
                    thread::sleep(time::Duration::from_secs(1));
                    let mut rng = rand::thread_rng();
                    lotto[x] = rng.gen_range(1..59);
            
                    display_nonzero_elements(&lotto);
                }
            
                evaluate_game(&mut guesses, &mut lotto);
                jackpot = jackpot * 0.3 as f32;
            },
            "exit" => {
                println!("exiting...");
                break;
            },
            &_ => println!("unrecognized..."),
        }
    }
}

fn display_player_stats(player: &Player) {
    println!("player: {} | ${} | games played: {}", player.username, player.money, player.games_played);
}

fn get_guesses(guesses: &mut[i32; 5]) {
    for x in 0..guesses.len() {
        while guesses[x] == 0 {
            println!("Enter guess {}: ", x+1);

            let number = get_user_input().parse::<i32>().unwrap();

            if number > 59 || number <= 0 {
                println!("Please select a number from 1 to 59");
            } else {
                guesses[x] = number;

                println!("");
                display_nonzero_elements(guesses);
            }
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    String::from(user_input.trim())
}

fn display_nonzero_elements(guesses: &[i32]) {
    for x in guesses {
        if x > &0 {
            print!("{x} ");
        }
    }
    println!(" ");
}

fn evaluate_game(guesses: &mut[i32; 5], lotto: &mut[i32; 5]) {
    guesses.sort();
    lotto.sort();

    println!("sorted guesses: {:?}", guesses);
    println!("sorted lotto numbers: {:?}", lotto);

    if guesses == lotto {
        println!("YOU WIN!!");
    } else {
        println!("lose");
    }
}