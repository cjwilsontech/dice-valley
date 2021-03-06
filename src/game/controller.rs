use rand::Rng;
use std::cmp::Ordering;

use super::{
    cards::{get_card_icon, CardIcon, CardKind, CardStack, ALL_CARDS, CARD_KIND_COUNT},
    landmarks::{get_landmark_cost, LandmarkKind, ALL_LANDMARKS},
    player::Player,
};

pub type Deck = [CardStack; CARD_KIND_COUNT];

// A CardStack owned by the specified player.
#[derive(Clone)]
pub struct PlayerCardStack {
    pub owner_turn: usize,
    pub card: CardStack,
}

pub fn award_coins(players: &mut Vec<Player>, player_turn: usize, amount: u8) -> u8 {
    let player = players
        .get_mut(player_turn)
        .expect("Player to be in bounds.");
    player.coins = u8::saturating_add(player.coins, amount);
    amount
}

pub fn take_coins(players: &mut Vec<Player>, player_turn: usize, amount: u8) -> u8 {
    let player = players
        .get_mut(player_turn)
        .expect("Player to be in bounds.");

    let available_amount = if u8::checked_sub(player.coins, amount).is_none() {
        player.coins
    } else {
        amount
    };

    player.coins = u8::saturating_sub(player.coins, available_amount);
    available_amount
}

pub fn steal_coins(
    players: &mut Vec<Player>,
    from_player_turn: usize,
    to_player_turn: usize,
    amount: u8,
) -> u8 {
    let available_amount = take_coins(players, from_player_turn, amount);
    award_coins(players, to_player_turn, available_amount);
    available_amount
}

pub fn award_coins_combo(
    players: &mut Vec<Player>,
    player_turn: usize,
    card_icon: CardIcon,
    amount: u8,
) -> u8 {
    let player = players.get(player_turn).expect("Player to be in boundsyy.");
    let card_count: u8 = player
        .cards
        .iter()
        .filter(|card| card.get_icon() == card_icon)
        .map(|card| card.count)
        .sum();
    let total_amount = u8::saturating_mul(
        card_count
            .try_into()
            .expect("Number of cards to not overflow u8."),
        amount,
    );
    award_coins(players, player_turn, total_amount);
    total_amount
}

pub fn steal_coins_from_all(
    players: &mut Vec<Player>,
    player_turn: usize,
    player_count: usize,
    amount: u8,
) -> u8 {
    let mut total_coins_stolen: u8 = 0;
    for i in (0..player_count).filter(|i| i.clone() != player_turn) {
        total_coins_stolen += steal_coins(players, i, player_turn, amount);
    }
    total_coins_stolen
}

pub fn trade_establishments(
    players: &mut Vec<Player>,
    player_turn: usize,
    other_player_turn: usize,
    player_card_kind: CardKind,
    other_player_card_kind: CardKind,
) {
    if get_card_icon(player_card_kind) == CardIcon::Major
        || get_card_icon(other_player_card_kind) == CardIcon::Major
    {
        panic!("Should not be able to trade Major establishments.");
    }

    remove_player_card(players, player_turn, player_card_kind, 1);
    remove_player_card(players, other_player_turn, other_player_card_kind, 1);
    add_player_card(players, player_turn, other_player_card_kind, 1);
    add_player_card(players, other_player_turn, player_card_kind, 1);
}

pub fn buy_card_from_deck(
    players: &mut Vec<Player>,
    player_turn: usize,
    card_deck: &mut Deck,
    card_kind: CardKind,
) {
    let card = card_deck
        .iter_mut()
        .find(|card| card.kind == card_kind)
        .expect("Expect to find card entry in deck.");
    remove_card_from_stack(card, 1);
    take_coins(players, player_turn, card.get_cost());
    add_player_card(players, player_turn, card_kind, 1);
}

pub fn buy_landmark(players: &mut Vec<Player>, player_turn: usize, landmark_kind: LandmarkKind) {
    take_coins(players, player_turn, get_landmark_cost(&landmark_kind));
    let player_mut = players
        .get_mut(player_turn)
        .expect("Player to be in bounds.");
    if player_mut.landmarks.contains(&landmark_kind) {
        panic!("Expected to not duplicate a landmark.");
    }
    player_mut.landmarks.push(landmark_kind);
}

pub fn add_player_card(
    players: &mut Vec<Player>,
    player_turn: usize,
    card_kind: CardKind,
    amount: u8,
) {
    let player_mut = players
        .get_mut(player_turn)
        .expect("Player to be in bounds.");
    let mut player_card = player_mut
        .cards
        .iter_mut()
        .find(|card| card.kind == card_kind)
        .expect("To find the card kind.");
    player_card.count = u8::saturating_add(player_card.count, amount);
}

pub fn remove_player_card(
    players: &mut Vec<Player>,
    player_turn: usize,
    card_kind: CardKind,
    amount: u8,
) {
    remove_card_from_stack(
        players
            .get_mut(player_turn)
            .expect("Player to be in bounds.")
            .cards
            .iter_mut()
            .find(|card| card.kind == card_kind)
            .expect("To find the card kind."),
        amount,
    );
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
                    owner_turn: player.turn,
                    card: cards,
                })
                .collect::<Vec<PlayerCardStack>>()
        })
        .collect::<Vec<PlayerCardStack>>();

    // Primary sort by card class order.
    activatable_cards.sort_by(|a, b| match a.card.get_order().cmp(&b.card.get_order()) {
        // Secondary sort by reverse turn order.
        Ordering::Equal => b.owner_turn.cmp(&a.owner_turn),
        other => other,
    });
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

pub fn has_player_won(player: &Player) -> bool {
    ALL_LANDMARKS
        .iter()
        .all(|landmark| player.landmarks.contains(landmark))
}

fn remove_card_from_stack(card: &mut CardStack, amount: u8) {
    card.count = u8::checked_sub(card.count, amount).expect("To not remove more cards than exist.");
}

#[cfg(test)]
mod tests {
    use crate::game::{
        cards::{CardIcon, CardKind, CardStack, ALL_CARDS},
        player::{Player, PlayerKind},
    };

    use super::{
        award_coins_combo, buy_card_from_deck, create_deck, get_activatable_cards, steal_coins,
        steal_coins_from_all, trade_establishments, Deck,
    };

    #[test]
    fn test_card_activation_order_primary_industry() {
        let card_activations = get_activatable_cards(9, 0, &get_players());
        assert_eq!(card_activations[0].card.kind, CardKind::FamilyRestaurant);
        assert_eq!(card_activations[0].card.count, 1);
        assert_eq!(card_activations[0].owner_turn, 1);
        assert_eq!(card_activations[1].card.kind, CardKind::Mine);
        assert_eq!(card_activations[1].card.count, 2);
        assert_eq!(card_activations[1].owner_turn, 0);
    }

    #[test]
    fn test_card_activation_order_secondary_industry() {
        let card_activations = get_activatable_cards(3, 0, &get_players());
        assert_eq!(card_activations[0].card.kind, CardKind::Cafe);
        assert_eq!(card_activations[0].card.count, 1);
        assert_eq!(card_activations[0].owner_turn, 3);
        assert_eq!(card_activations[1].card.kind, CardKind::Cafe);
        assert_eq!(card_activations[1].card.count, 1);
        assert_eq!(card_activations[1].owner_turn, 1);
        assert_eq!(card_activations[2].card.kind, CardKind::Bakery);
        assert_eq!(card_activations[2].card.count, 1);
        assert_eq!(card_activations[2].owner_turn, 0);
    }

    #[test]
    fn test_steal_exact_coins() {
        let mut players = get_players();
        let total_stolen = steal_coins(&mut players, 2, 0, 3);
        assert_eq!(total_stolen, 3);
        assert_eq!(players[2].coins, 0);
        assert_eq!(players[0].coins, 4);
    }

    #[test]
    fn test_steal_more_coins() {
        let mut players = get_players();
        let total_stolen = steal_coins(&mut players, 2, 0, 1);
        assert_eq!(total_stolen, 1);
        assert_eq!(players[2].coins, 2);
        assert_eq!(players[0].coins, 2);
    }

    #[test]
    fn test_steal_less_coins() {
        let mut players = get_players();
        let total_stolen = steal_coins(&mut players, 0, 1, 2);
        assert_eq!(total_stolen, 1);
        assert_eq!(players[0].coins, 0);
        assert_eq!(players[1].coins, 1);
    }

    #[test]
    fn test_steal_no_coins() {
        let mut players = get_players();
        let total_stolen = steal_coins(&mut players, 1, 0, 2);
        assert_eq!(total_stolen, 0);
        assert_eq!(players[1].coins, 0);
        assert_eq!(players[0].coins, 1);
    }

    #[test]
    fn test_steal_coins_from_all() {
        let mut players = get_players();
        let player_count = players.iter().count();
        let total_stolen = steal_coins_from_all(&mut players, 0, player_count, 1);
        assert_eq!(total_stolen, 1);
        assert_eq!(players[0].coins, 2);
        assert_eq!(players[1].coins, 0);
        assert_eq!(players[2].coins, 2);
    }

    #[test]
    fn test_award_coins_combo() {
        let mut players = get_players();
        let total_earned = award_coins_combo(&mut players, 2, CardIcon::Wheat, 2);
        assert_eq!(total_earned, 6);
        assert_eq!(players[2].coins, 9);
    }

    #[test]
    fn test_trade_establishments() {
        let mut players = get_players();
        trade_establishments(
            &mut players,
            2,
            1,
            CardKind::Bakery,
            CardKind::FamilyRestaurant,
        );
        assert_eq!(
            find_card_in_deck(&players[2].cards, CardKind::Bakery).count,
            0
        );
        assert_eq!(
            find_card_in_deck(&players[2].cards, CardKind::FamilyRestaurant).count,
            1
        );
        assert_eq!(
            find_card_in_deck(&players[1].cards, CardKind::FamilyRestaurant).count,
            0
        );
        assert_eq!(
            find_card_in_deck(&players[1].cards, CardKind::Bakery).count,
            2
        );
    }

    #[test]
    #[should_panic(expected = "Should not be able to trade Major establishments.")]
    fn test_trade_major_establishments() {
        let mut players = get_players();
        trade_establishments(
            &mut players,
            3,
            0,
            CardKind::BusinessCenter,
            CardKind::WheatField,
        );
    }

    #[test]
    fn test_buy_card_from_deck() {
        let mut players = get_players();
        let mut card_deck = create_deck();
        buy_card_from_deck(&mut players, 2, &mut card_deck, CardKind::AppleOrchard);
        let card = find_card_in_deck(&players[2].cards, CardKind::AppleOrchard);
        let card_deck = find_card_in_deck(&card_deck, CardKind::AppleOrchard);
        assert_eq!(card.count, 1);
        assert_eq!(players[2].coins, 0);
        assert_eq!(card_deck.count, 5);
    }

    #[test]
    fn test_buy_another_card_from_deck() {
        let mut players = get_players();
        let mut card_deck = create_deck();
        buy_card_from_deck(&mut players, 2, &mut card_deck, CardKind::WheatField);
        let card = find_card_in_deck(&players[2].cards, CardKind::WheatField);
        let card_deck = find_card_in_deck(&card_deck, CardKind::WheatField);
        assert_eq!(card.count, 4);
        assert_eq!(players[2].coins, 2);
        assert_eq!(card_deck.count, 5);
    }

    fn find_card_in_deck(deck: &Deck, card_kind: CardKind) -> &CardStack {
        deck.iter()
            .find(|card| card.kind == card_kind)
            .expect("To find the card.")
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
                landmarks: vec![],
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
                landmarks: vec![],
                name: String::from(""),
                kind: PlayerKind::Human,
                turn: 1,
                coins: 0,
            },
            Player {
                cards: ALL_CARDS.map(|kind| CardStack {
                    count: match kind {
                        CardKind::Bakery => 1,
                        CardKind::WheatField => 3,
                        _ => 0,
                    },
                    kind,
                }),
                landmarks: vec![],
                name: String::from(""),
                kind: PlayerKind::Human,
                turn: 2,
                coins: 3,
            },
            Player {
                cards: ALL_CARDS.map(|kind| CardStack {
                    count: match kind {
                        CardKind::Bakery
                        | CardKind::WheatField
                        | CardKind::Cafe
                        | CardKind::BusinessCenter => 1,
                        _ => 0,
                    },
                    kind,
                }),
                landmarks: vec![],
                name: String::from(""),
                kind: PlayerKind::Human,
                turn: 3,
                coins: 0,
            },
        ]
    }
}
