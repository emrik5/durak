use deck::Deck;

mod cards;
mod deck;
mod net;
mod player;
mod translation;
fn main() {
    let mut deck = Deck::new(13);
    deck.shuffle();
    for card in deck.into_inner() {
        println!("{}", card);
    }
}
