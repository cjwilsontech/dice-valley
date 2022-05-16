use crate::{
    game::{cards::CardKind, controller::Deck, landmarks::LandmarkKind, player::Player},
    ui,
};

pub fn get_number_of_dice() -> u8 {
    ui::get_number_of_dice(true)
}

pub fn ask_reroll() -> bool {
    ui::ask_reroll()
}

pub fn get_trade_establishments(
    players: &Vec<Player>,
    owner_turn: usize,
) -> (usize, CardKind, CardKind) {
    ui::get_player_to_trade_establishment_with(players, owner_turn)
}

pub fn get_player_to_steal_coins_from(
    players: &Vec<Player>,
    owner_turn: usize,
    amount: u8,
) -> usize {
    ui::get_player_to_steal_coins_from(players, owner_turn, amount)
}

pub fn buy_a_card(
    player: &Player,
    card_deck: &Deck,
) -> Option<(Option<CardKind>, Option<LandmarkKind>)> {
    ui::buy_a_card(card_deck, player)
}
