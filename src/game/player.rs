use super::{controller::Deck, landmarks::LandmarkKind};

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
}
