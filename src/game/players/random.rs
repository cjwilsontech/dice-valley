use crate::game::{
    cards::{get_card_icon, CardIcon, CardKind, CardStack},
    controller::Deck,
    landmarks::{get_landmark_cost, LandmarkKind, ALL_LANDMARKS},
    player::Player,
};
use rand::Rng;

pub fn get_number_of_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub fn ask_reroll() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.25)
}

pub fn get_trade_establishments(
    players: &Vec<Player>,
    owner_turn: usize,
    player: &Player,
) -> (usize, CardKind, CardKind) {
    let mut rng = rand::thread_rng();

    let available_players: Vec<&Player> = players
        .iter()
        .filter(|player| player.turn != owner_turn)
        .collect();
    let player_count = available_players.iter().count();
    let other_player = available_players
        .get(rng.gen_range(0..player_count))
        .expect("Player to not be OOB.");

    let available_other_cards: Vec<CardKind> = other_player
        .cards
        .iter()
        .filter_map(|card| {
            if get_card_icon(card.kind) != CardIcon::Major && card.count > 0 {
                Some(card.kind)
            } else {
                None
            }
        })
        .collect();
    let other_player_card = available_other_cards
        .get(rng.gen_range(0..available_other_cards.iter().count()))
        .expect("Card to not be OOB.");

    let available_cards: Vec<CardKind> = player
        .cards
        .iter()
        .filter_map(|card| {
            if get_card_icon(card.kind) != CardIcon::Major && card.count > 0 {
                Some(card.kind)
            } else {
                None
            }
        })
        .collect();
    let player_card = available_cards
        .get(rng.gen_range(0..available_cards.iter().count()))
        .expect("Card to not be OOB.");

    (
        other_player.turn,
        other_player_card.clone(),
        player_card.clone(),
    )
}

pub fn get_player_to_steal_coins_from(players: &Vec<Player>, owner_turn: usize) -> usize {
    // Take from the player with the most coins.
    players
        .iter()
        .filter(|player| player.turn != owner_turn)
        .max_by_key(|player| player.coins)
        .expect("Player to not be None.")
        .turn
}

pub fn buy_a_card(
    player: &Player,
    card_deck: &Deck,
) -> Option<(Option<CardKind>, Option<LandmarkKind>)> {
    let mut rng = rand::thread_rng();

    // Buy a landmark if possible.
    let available_landmarks: Vec<LandmarkKind> = ALL_LANDMARKS
        .into_iter()
        .filter(|landmark| {
            get_landmark_cost(landmark) <= player.coins && !player.landmarks.contains(landmark)
        })
        .collect();

    let landmark_count = available_landmarks.iter().count();
    if landmark_count > 0 {
        let landmark = available_landmarks
            .get(rng.gen_range(0..landmark_count))
            .expect("Landmark to not be OOB.");
        return Some((None, Some(landmark.clone())));
    }

    // Sometimes don't buy.
    if rng.gen_bool(0.15) {
        return None;
    }

    // Try to buy a card.
    let available_cards: Vec<&CardStack> = card_deck
        .into_iter()
        .filter(|card| card.count > 0 && card.get_cost() <= player.coins)
        .collect();
    let card_count = available_cards.iter().count();
    if card_count > 0 {
        let card = available_cards
            .get(rng.gen_range(0..card_count))
            .expect("Card to not be OOB.");
        return Some((Some(card.kind), None));
    }

    // No cards we can afford.
    None
}
