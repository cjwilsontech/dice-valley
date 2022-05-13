mod game;
mod ui;

use game::player::get_player_type_name;
use ui::{get_player_count, get_player_names, get_player_types};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");
    let player_count = get_player_count();
    let player_names = get_player_names(player_count);
    let player_types = get_player_types(player_count, &player_names);

    for i in 0..player_count as usize {
        println!(
            "Name: {}, Type: {}",
            player_names[i].as_ref().expect(""),
            get_player_type_name(&player_types[i])
        );
    }
}
