use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::iter::FromIterator;

static SUITS: [&str; 4] = ["spades", "clubs", "hearts", "diamonds"];
static RANKS: [(&str, u32); 13] = [
    ("ace", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
    ("jack", 10),
    ("queen", 10),
    ("king", 10),
];

struct Card {
    rank: String,
    suit: String,
    val: u32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for suit in &SUITS {
            for rank in &RANKS {
                cards.push(Card {
                    rank: String::from(rank.0),
                    suit: String::from(*suit),
                    val: rank.1,
                });
            }
        }
        Deck { cards: cards }
    }

    fn draw(&mut self) -> Result<Card, &str> {
        match self.cards.len() {
            0 => Err("Not enough cards in the deck!"),
            _ => {
                let card_index = rand::thread_rng().gen_range(0, self.cards.len());
                Ok(self.cards.remove(card_index))
            }
        }
    }
}

enum Action {
    Hit,
    Stay,
}

struct Player {
    name: String,
    hand: Vec<Card>,
    stayed: bool,
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            hand: Vec::new(),
            stayed: false,
        }
    }

    fn get_value(&self) -> u32 {
        let mut value = self.hand.iter().map(|x| x.val).fold(0, |x, y| x + y);
        let ace_count = self.hand.iter().filter(|x| x.val == 1).count() as u32;
        for _ in 0..ace_count {
            let new_value = value + 10;
            if new_value > 21 {
                break;
            }
            value = new_value;
        }
        value
    }

    fn draw(&mut self, deck: &mut Deck) {
        self.hand.push(deck.draw().expect("The deck is empty"))
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand = self
            .hand
            .iter()
            .fold(String::new(), |s, c| s + "\n" + c.to_string().as_str());
        write!(
            f,
            "{} has a total of {}, {}",
            self.name,
            self.get_value(),
            hand
        )
    }
}

fn main() {
    let mut playing = true;
    while playing {
        // Initialize the deck
        let mut deck = Deck::new();

        // Initialize the players
        let mut dealer = Player::new("dealer");
        let mut player = Player::new("player");

        // This is nice because it only holds the mutable references for the length of the loop.
        for player in [&mut player, &mut dealer].iter_mut() {
            for _ in 0..2 {
                player.draw(&mut deck);
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
                for player in [&mut dealer, &mut player].iter() {
                    println!();
                    println!("{}", player);
                }
                println!();
                let input = prompt_input("Hit or stay? (h/s)", "Please type h or s", |s| match s {
                    "h" => Some(Action::Hit),
                    "s" => Some(Action::Stay),
                    _ => None,
                });
                match input {
                    Action::Hit => player.draw(&mut deck),
                    Action::Stay => player.stayed = true,
                }
                println!();
                println!("------------------------------");
                println!();
            }

            if !dealer.stayed {
                match dealer.get_value().cmp(&17) {
                    Ordering::Less => dealer.draw(&mut deck),
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
            println!("{}", player);
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
