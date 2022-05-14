use super::controller::Deck;

pub enum PlayerKind {
    Computer,
    Human,
}

pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub cards: Deck,
    pub coins: u8,
}

impl Player {
    pub fn get_kind_name(&self) -> &'static str {
        match self.kind {
            PlayerKind::Computer => "Computer",
            PlayerKind::Human => "Human",
        }
    }
}
