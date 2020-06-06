use crate::deck::*;

pub enum Action {
    Hit,
    Stay,
}

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
    pub stayed: bool,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            hand: Vec::new(),
            stayed: false,
        }
    }

    pub fn get_value(&self) -> u32 {
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

    pub fn draw(&mut self, deck: &mut Deck, print: bool) {
        let card = deck.draw().expect("The deck is empty");
        if print {
            println!("{} drew {}", self.name, card);
        }
        self.hand.push(card);
    }

    pub fn display(&self, hide_first: bool, show_total: bool) {
        if show_total {
            println!("{}: {}", self.name, self.get_value())
        } else {
            println!("{}:", self.name);
        }
        for i in 0..self.hand.len() {
            if hide_first && i == 0 {
                println!("    hidden");
            } else {
                println!("    {}", self.hand[i]);
            }
        }
    }
}
