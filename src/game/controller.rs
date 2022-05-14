use rand::Rng;

use super::cards::{CardKind, CardStack, ALL_CARDS, CARD_KIND_COUNT};

pub type Deck = [CardStack; CARD_KIND_COUNT];

pub fn create_deck() -> Deck {
    ALL_CARDS.map(|kind| CardStack {
        count: match kind {
            CardKind::BusinessCenter | CardKind::Stadium | CardKind::TvStation => 4,
            _ => 6,
        },
        kind,
    })
}

pub fn create_player_deck() -> Deck {
    ALL_CARDS.map(|kind| CardStack {
        count: match kind {
            CardKind::WheatField | CardKind::Bakery => 1,
            _ => 0,
        },
        kind,
    })
}

pub type DiceRoll = (u8, Option<u8>);

pub fn roll_dice(roll_two_dice: bool) -> DiceRoll {
    let mut rng = rand::thread_rng();
    (
        rng.gen_range(1..=6),
        if roll_two_dice {
            Some(rng.gen_range(1..=6))
        } else {
            None
        },
    )
}
