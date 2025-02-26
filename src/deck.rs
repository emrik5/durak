use rand::{Rng, SeedableRng, rng, rngs::SmallRng};

use crate::cards::{Card, Suit};
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(ranks_per_suit: u8) -> Self {
        let rank_range = match ranks_per_suit {
            0..=13 => (15 - ranks_per_suit)..=(14),
            _ => 1..=ranks_per_suit,
        };
        let mut cards = vec![];
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in rank_range.clone() {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }
    pub fn into_inner(self) -> Vec<Card> {
        self.cards
    }
    pub fn shuffle(&mut self) {
        let shuffle_iterations = self.cards.len() * 2;
        let rng: Vec<usize> = SmallRng::from_rng(&mut rng())
            .random_iter()
            .take(shuffle_iterations)
            .map(|n: i32| n.abs() as usize % self.cards.len())
            .collect();
        for i in rng {
            let swap = self.cards.swap_remove(i);
            self.cards.push(swap);
        }
    }
}
