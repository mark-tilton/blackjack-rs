mod deck;
mod player;

use deck::*;
use player::*;
use std::cmp::Ordering;

fn main() {
    let mut playing = true;
    while playing {
        // Initialize the deck
        let mut deck = Deck::new();

        // Initialize the players
        let mut dealer = Player::new("dealer");
        let mut player = Player::new("player");

        // This is nice because it only holds the mutable references for the length of the loop.
        for player in [&mut dealer, &mut player].iter_mut() {
            for _ in 0..2 {
                player.draw(&mut deck, false);
            }
        }

        // Game loop
        let winners = loop {
            if player.get_value() == 21 && dealer.get_value() == 21 {
                break vec![&player, &dealer];
            }

            // I kinda wish I could consolidate these.
            match player.get_value().cmp(&21) {
                Ordering::Equal => break vec![&player],
                Ordering::Greater => break vec![&dealer],
                _ => {}
            }
            match dealer.get_value().cmp(&21) {
                Ordering::Equal => break vec![&dealer],
                Ordering::Greater => break vec![&player],
                _ => {}
            }

            if !player.stayed {
                // Display the state of the game.
                println!();
                dealer.display(true, false);
                println!();
                player.display(false, true);

                println!();
                let input = prompt_input("Hit or stay? (h/s)", "Please type h or s", |s| match s {
                    "h" => Some(Action::Hit),
                    "s" => Some(Action::Stay),
                    _ => None,
                });
                println!();
                println!("------------------------------");
                println!();
                match input {
                    Action::Hit => player.draw(&mut deck, true),
                    Action::Stay => player.stayed = true,
                }
            }

            if !dealer.stayed {
                match dealer.get_value().cmp(&17) {
                    Ordering::Less => dealer.draw(&mut deck, true),
                    _ => dealer.stayed = true,
                };
            }

            if [&player, &dealer].iter().all(|x| x.stayed) {
                match player.get_value().cmp(&dealer.get_value()) {
                    Ordering::Equal => break vec![&player, &dealer],
                    Ordering::Less => break vec![&dealer],
                    Ordering::Greater => break vec![&player],
                }
            }
        };

        println!();
        println!("==============================");
        for player in [&dealer, &player].iter() {
            println!();
            player.display(false, true);
        }
        println!();

        // Report the winner
        match winners.len() {
            1 => println!("{} wins!", winners[0].name),
            2 => println!("Tie!"),
            _ => {} // Values below 1 or above 2 should not happen
        }

        println!();

        // See if the player wants to play again.
        playing = prompt_input(
            "Would you like to play again? (y/n)",
            "Please enter y or n",
            |s| match s {
                "y" => Some(true),
                "n" => Some(false),
                _ => None,
            },
        );

        println!();
        println!("------------------------------");
    }
}

fn prompt_input<T>(prompt: &str, error_text: &str, selector: fn(&str) -> Option<T>) -> T {
    println!("{}", prompt);
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match selector(input.trim()) {
                Some(result) => break result,
                None => {
                    println!("{}", error_text);
                    continue;
                }
            },
            Err(_) => {
                println!("Something has gone wrong, please try again.");
                continue;
            }
        }
    }
}
