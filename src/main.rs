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

enum Action {
    Hit,
    Stay,
}

// Represents a player for the scope of a single game
struct Player {
    name: String,
    hand: Vec<Card>,
    stayed: bool,
    action: fn(&Player, &[Player]) -> Action,
}

impl Player {
    fn new(name: &str, action: fn(&Player, &[Player]) -> Action) -> Player {
        Player {
            name: String::from(name),
            hand: Vec::new(),
            stayed: false,
            action: action,
        }
    }

    fn take_action(&self, game_state: &[Player]) -> Action {
        (self.action)(self, game_state)
    }

    // TODO: Add ace handling
    fn get_value(&self) -> u32 {
        self.hand.iter().map(|x| x.val).fold(0, |x, y| x + y)
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
        let mut deck: Vec<Card> = Vec::new();
        populate_deck(&mut deck); // Maybe this should be deck::new()?

        // Initialize the players
        let dealer = Player::new("dealer", |p, _| match p.get_value().cmp(&17) {
            Ordering::Less => Action::Hit,
            _ => Action::Stay,
        });
        // ------------------------------

        // player drew ace of clubs
        // dealer drew five of hearts

        // player has a total of 10,
        // six of hearts
        // three of diamonds
        // ace of clubs

        // dealer has a total of 16,
        // nine of spades
        // two of clubs
        // five of hearts
        // Hit or stay? (h/s)
        // h
        // ------------------------------
        // player drew four of diamonds
        // dealer drew jack of diamonds

        // player has a total of 14,
        // six of hearts
        // three of diamonds
        // ace of clubs
        // four of diamonds

        // dealer has a total of 26,
        // nine of spades
        // two of clubs
        // five of hearts
        // jack of diamonds

        // I would benefit here from having an immutible game state.

        let player = Player::new("player", |_, game_state| {
            // Display the state of the game.
            for player in game_state.iter() {
                println!();
                println!("{}", player);
            }
            println!();
            let input = prompt_input("Hit or stay? (h/s)", "Please type h or s", |s| match s {
                "h" => Some(Action::Hit),
                "s" => Some(Action::Stay),
                _ => None,
            });
            println!();
            println!("------------------------------");
            println!();
            input
        });
        let mut players = [player, dealer]; // Move players into the players array

        // Initialize the player's hands
        for player in players.iter_mut() {
            match draw_card(&mut deck) {
                Ok(card) => player.hand.push(card),
                Err(msg) => println!("{}", msg), // TODO: Handle this case better.
            };
        }

        // Game loop
        let winners = 'game_loop: loop {
            // What are the players going to do?
            // I spent a long time trying to come up with a way to do this without a range here.
            // The problem is that if the player loses, it needs to return the opponent.
            for i in 0..players.len() {
                let player = &players[i];
                match player.get_value().cmp(&21) {
                    Ordering::Equal => break 'game_loop vec![&*player],
                    Ordering::Greater => break 'game_loop vec![&players[1 - i]],
                    _ => {}
                }
                if player.stayed {
                    continue;
                }
                match player.take_action(&players) {
                    Action::Hit => {
                        match draw_card(&mut deck) {
                            Ok(card) => {
                                println!("{} drew {}", player.name, card);
                                players[i].hand.push(card);
                            }
                            Err(msg) => println!("{}", msg), // TODO: Handle this case better.
                        };
                    }
                    Action::Stay => players[i].stayed = true,
                }
            }
            if players.iter().all(|x| x.stayed) {
                match players[0].get_value().cmp(&players[1].get_value()) {
                    Ordering::Equal => break vec![&players[0], &players[1]],
                    Ordering::Less => break vec![&players[1]],
                    Ordering::Greater => break vec![&players[0]],
                }
            }
        };

        println!();
        println!("============================");
        for player in players.iter() {
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

fn populate_deck(deck: &mut Vec<Card>) {
    deck.clear();
    for suit in &SUITS {
        for rank in &RANKS {
            deck.push(Card {
                rank: String::from(rank.0),
                suit: String::from(*suit),
                val: rank.1,
            });
        }
    }
}

fn draw_card(deck: &mut Vec<Card>) -> Result<Card, &str> {
    match deck.len() {
        0 => Err("Not enough cards in the deck!"),
        _ => {
            let card_index = rand::thread_rng().gen_range(0, deck.len());
            Ok(deck.remove(card_index))
        }
    }
}
