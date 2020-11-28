use rand::Rng;
use std::fmt;

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

pub struct Card {
    rank: String,
    suit: String,
    pub val: u32,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn new() -> Self {
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
    pub fn draw(&mut self) -> Result<Card, &str> {
        match self.cards.len() {
            0 => Err("Not enough cards in the deck!"),
            _ => {
                let card_index = rand::thread_rng().gen_range(0, self.cards.len());
                Ok(self.cards.remove(card_index))
            }
        }
    }
}
