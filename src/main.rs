use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Write;

fn main() {
    let mut money = 0;
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

fn create_threads(mut money: i64) {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        if money > 600000 {
            loop{
                thread::sleep(Duration::from_secs(3));
                println!("ALERT!!! Someone stole $35,000 from you!");
                tx1.send(money-35000).unwrap();
            }
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        if money > 10000-1{
            loop{
                thread::sleep(Duration::from_secs(5));
                println!("ALERT!!! Someone stole $10,000 from you!");
                tx2.send(money-10000).unwrap();
            }
        }
    });

    let tx3 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        loop{
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            match user_input.trim() {
                "catch" => {
                    println!("The thieves have left.");
                    std::process::exit(0);
                }
                _ => {
                    println!("Oh nooo");
                }
            }
        }
    });


    for dollar in rx {
        println!("Funds left: {}", dollar);
    }
}
