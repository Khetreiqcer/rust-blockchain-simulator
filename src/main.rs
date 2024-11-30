mod block;
mod blockchain;

use crate::block::Transaction;
use crate::blockchain::Blockchain;
use std::io;

fn main() {
    let filename = "blockchain.json";
    let mut blockchain = Blockchain::load_from_file(filename).unwrap_or_else(|| Blockchain::new(3));

    loop {
        println!("\n=== Blockchain CLI ===");
        println!("1. Add a new block");
        println!("2. Display the blockchain");
        println!("3. Validate blockchain");
        println!("4. Save and exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let transactions = input_transactions();
                blockchain.add_block(transactions);
                println!("Block added successfully!");
            }
            "2" => {
                println!("\nCurrent Blockchain:");
                for block in &blockchain.chain {
                    println!("{:#?}", block);
                }
            }
            "3" => {
                if blockchain.is_valid() {
                    println!("Blockchain is valid!");
                } else {
                    println!("Blockchain is invalid!");
                }
            }
            "4" => {
                blockchain.save_to_file(filename);
                println!("Blockchain saved to file. Exiting...");
                break;
            }
            _ => println!("Invalid option! Please choose a valid action."),
        }
    }
}

fn input_transactions() -> Vec<Transaction> {
    let mut transactions = Vec::new();

    loop {
        println!("\nEnter a new transaction:");
        println!("Sender:");
        let mut sender = String::new();
        io::stdin()
            .read_line(&mut sender)
            .expect("Failed to read sender");

        println!("Receiver:");
        let mut receiver = String::new();
        io::stdin()
            .read_line(&mut receiver)
            .expect("Failed to read receiver");

        println!("Amount:");
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read amount");
        let amount: f64 = amount.trim().parse().expect("Invalid amount!");

        transactions.push(Transaction {
            sender: sender.trim().to_string(),
            receiver: receiver.trim().to_string(),
            amount,
        });

        println!("Transaction added. Add another? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        if choice.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }

    transactions
}
