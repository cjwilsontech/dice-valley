use super::controller::Deck;

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
}

impl Player {
    pub fn get_kind_name(&self) -> &'static str {
        match self.kind {
            PlayerKind::Computer => "Computer",
            PlayerKind::Human => "Human",
        }
    }
}
