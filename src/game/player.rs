use super::{cards::CardKind, controller::Deck, landmarks::LandmarkKind, players::human};

#[derive(Clone)]
pub enum PlayerKind {
    Computer,
    Human,
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub turn: usize,
    pub cards: Deck,
    pub coins: u8,
    pub landmarks: Vec<LandmarkKind>,
}

impl Player {
    pub fn get_kind_name(&self) -> &'static str {
        match self.kind {
            PlayerKind::Computer => "Computer",
            PlayerKind::Human => "Human",
        }
    }

    pub fn get_number_of_dice(&self) -> u8 {
        match self.kind {
            PlayerKind::Computer => 1,
            PlayerKind::Human => human::get_number_of_dice(),
        }
    }
    pub fn ask_reroll(&self) -> bool {
        match self.kind {
            PlayerKind::Computer => false,
            PlayerKind::Human => human::ask_reroll(),
        }
    }

    pub fn get_trade_establishments(
        &self,
        players: &Vec<Player>,
        owner_turn: usize,
    ) -> (usize, CardKind, CardKind) {
        match self.kind {
            PlayerKind::Computer => todo!(),
            PlayerKind::Human => human::get_trade_establishments(players, owner_turn),
        }
    }

    pub fn get_player_to_steal_coins_from(
        &self,
        players: &Vec<Player>,
        owner_turn: usize,
        amount: u8,
    ) -> usize {
        match self.kind {
            PlayerKind::Computer => todo!(),
            PlayerKind::Human => human::get_player_to_steal_coins_from(players, owner_turn, amount),
        }
    }

    pub fn buy_a_card(&self, card_deck: &Deck) -> Option<(Option<CardKind>, Option<LandmarkKind>)> {
        match self.kind {
            PlayerKind::Computer => todo!(),
            PlayerKind::Human => human::buy_a_card(self, card_deck),
        }
    }
}
