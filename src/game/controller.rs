use rand::Rng;

use super::{
    cards::{CardKind, CardStack, ALL_CARDS, CARD_KIND_COUNT},
    player::Player,
};

pub type Deck = [CardStack; CARD_KIND_COUNT];

// A CardStack owned by the specified player.
pub struct PlayerCardStack {
    pub player_turn: usize,
    pub card: CardStack,
}

pub fn get_activatable_cards(
    roll_total: u8,
    player_turn: usize,
    players: &Vec<Player>,
) -> Vec<PlayerCardStack> {
    let mut activatable_cards: Vec<PlayerCardStack> = players
        .iter()
        .flat_map(|player| {
            player
                .cards
                .into_iter()
                .filter(|card| {
                    card.count > 0 && card.test_activation(roll_total, player.turn == player_turn)
                })
                .map(|cards| PlayerCardStack {
                    player_turn: player.turn,
                    card: cards,
                })
                .collect::<Vec<PlayerCardStack>>()
        })
        .collect::<Vec<PlayerCardStack>>();
    activatable_cards.sort_by_key(|x| x.card.get_order());
    activatable_cards
}

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

#[cfg(test)]
mod tests {
    use crate::game::{
        cards::{CardKind, CardStack, ALL_CARDS},
        player::{Player, PlayerKind},
    };

    use super::get_activatable_cards;

    #[test]
    fn card_activation_order_primary_industry() {
        let card_activations = get_activatable_cards(9, 0, &get_players());
        assert_eq!(card_activations[0].card.kind, CardKind::FamilyRestaurant);
        assert_eq!(card_activations[0].card.count, 1);
        assert_eq!(card_activations[0].player_turn, 1);
        assert_eq!(card_activations[1].card.kind, CardKind::Mine);
        assert_eq!(card_activations[1].card.count, 2);
        assert_eq!(card_activations[1].player_turn, 0);
    }

    #[test]
    fn card_activation_order_secondary_industry() {
        let card_activations = get_activatable_cards(3, 0, &get_players());
        assert_eq!(card_activations[0].card.kind, CardKind::Cafe);
        assert_eq!(card_activations[0].card.count, 1);
        assert_eq!(card_activations[0].player_turn, 1);
        assert_eq!(card_activations[1].card.kind, CardKind::Bakery);
        assert_eq!(card_activations[1].card.count, 1);
        assert_eq!(card_activations[1].player_turn, 0);
    }

    fn get_players() -> Vec<Player> {
        vec![
            Player {
                cards: ALL_CARDS.map(|kind| CardStack {
                    count: match kind {
                        CardKind::WheatField | CardKind::Bakery => 1,
                        CardKind::Mine => 2,
                        _ => 0,
                    },
                    kind,
                }),
                name: String::from(""),
                kind: PlayerKind::Human,
                turn: 0,
                coins: 1,
            },
            Player {
                cards: ALL_CARDS.map(|kind| CardStack {
                    count: match kind {
                        CardKind::WheatField
                        | CardKind::Bakery
                        | CardKind::FamilyRestaurant
                        | CardKind::Cafe => 1,
                        _ => 0,
                    },
                    kind,
                }),
                name: String::from(""),
                kind: PlayerKind::Human,
                turn: 1,
                coins: 0,
            },
        ]
    }
}
