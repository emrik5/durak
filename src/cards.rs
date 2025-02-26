use std::{
    fmt::{Display, write},
    ops::Index,
};

use crate::translation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", translation::suit_name(*self)))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: u8,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!("{}", translation::card_name(self.suit(), self.rank())),
        )
    }
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Self {
        Self { suit, rank }
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
    pub fn rank(&self) -> u8 {
        self.rank
    }
}
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn count(&self) -> usize {
        self.0.len()
    }
    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }
    pub fn play_index(&mut self, index: impl Into<usize>) -> Option<Card> {
        let index = index.into();
        if index >= self.count() {
            return None;
        }
        Some(self.0.swap_remove(index))
    }
    pub fn play_card(&mut self, card: Card) -> Option<Card> {
        let index = self.0.iter().position(|&item| item == card)?;
        self.play_index(index)
    }
}
