use std::sync::RwLock;

use crate::cards;
static LANG: RwLock<Language> = RwLock::new(Language::Swedish);
#[derive(Default, Debug, Clone, Copy)]
enum Language {
    #[default]
    English,
    Swedish,
}
pub fn suit_name(suit: cards::Suit) -> String {
    match lang() {
        Language::English => match suit {
            cards::Suit::Clubs => "Clubs",
            cards::Suit::Diamonds => "Diamonds",
            cards::Suit::Hearts => "Hearts",
            cards::Suit::Spades => "Spades",
        },
        Language::Swedish => match suit {
            cards::Suit::Clubs => "Klöver",
            cards::Suit::Diamonds => "Ruter",
            cards::Suit::Hearts => "Hjärter",
            cards::Suit::Spades => "Spader",
        },
    }
    .to_string()
}
pub fn rank_name(rank: u8) -> String {
    if let (11..=14) = rank {
    } else {
        return rank.to_string();
    }
    match lang() {
        Language::English => match rank {
            11 => "Jack",
            12 => "Queen",
            13 => "King",
            14 => "Ace",
            _ => unreachable!(),
        },
        Language::Swedish => match rank {
            11 => "Knekt",
            12 => "Dam",
            13 => "Kung",
            14 => "Ess",
            _ => unreachable!(),
        },
    }
    .to_string()
}
pub fn card_name(suit: cards::Suit, rank: u8) -> String {
    let rank = rank_name(rank);
    match lang() {
        Language::English => format!("{} of {}", rank, suit),
        Language::Swedish => format!("{} {}", suit, rank),
    }
}
fn lang() -> Language {
    match LANG.try_read() {
        Ok(lang) => *lang,
        Err(_) => Language::default(),
    }
}
