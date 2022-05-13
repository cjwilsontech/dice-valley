pub enum PlayerType {
    Computer,
    Human,
}

pub fn get_player_type_name(player_type: &Option<PlayerType>) -> &'static str {
    match player_type {
        Some(PlayerType::Computer) => "Computer",
        Some(PlayerType::Human) => "Human",
        None => "None",
    }
}
