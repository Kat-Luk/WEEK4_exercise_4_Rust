use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};

fn main() {
    let mut money: i64 = 0;
    println!("Do you have a million dollars? | y = yes, n = no");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input.");
    match choice.trim() {
        "y" => {
            money += 1000000;
            println!("All right then, millionaire.");
        }
        "n" => {
            money += 100000;
            println!("Let's just assume you have $100,000 then.");
        }
        _ => println!("errorik"),
    }
    create_threads(money);
}

fn create_threads(money: i64) {
    let money = Arc::new(Mutex::new(money));
    let starting_amount = *money.lock().unwrap();
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    drop(tx);

    let money1 = Arc::clone(&money);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));
            let mut m = money1.lock().unwrap();
            if *m <= 0 {
                println!("You lost all your money!");
                std::process::exit(0);
            } 
            if starting_amount > 600_000 && *m <= 600_000 { break; }
            *m -= 35_000;
            println!("ALERT!!! Someone stole $35,000 from you!");
            tx1.send(*m).unwrap();
        }
    });

    let money2 = Arc::clone(&money);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut m = money2.lock().unwrap();
            if *m <= 0 {
                println!("You lost all your money!");
                std::process::exit(0);
            }
            *m -= 10_000;  // ← subtract after check
            println!("ALERT!!! Someone stole $10,000 from you!");
            tx2.send(*m).unwrap();
        }
    });

    thread::spawn(move || {
        loop{
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            match user_input.trim() {
                "catch" => {
                    println!("caught the thieves.");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });


    for dollar in rx {
        println!("Funds left: {}", dollar);
    }
    println!("The thieves have left.");
}
