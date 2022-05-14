pub enum PlayerType {
    Computer,
    Human,
}

pub fn get_player_type_name(player_type: &PlayerType) -> &'static str {
    match player_type {
        PlayerType::Computer => "Computer",
        PlayerType::Human => "Human",
    }
}
