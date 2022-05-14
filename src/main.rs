mod game;
mod ui;

use crate::game::controller::{create_deck, create_player_deck, roll_dice, Deck};
use crate::game::player::{get_player_type_name, PlayerType};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");
    let player_count = ui::get_player_count();
    let player_names = ui::get_player_names(player_count);
    let player_types = ui::get_player_types(player_count, &player_names);
    let card_deck = create_deck();
    let player_cards: [Deck; MAX_PLAYER_COUNT] = [
        create_player_deck(),
        create_player_deck(),
        create_player_deck(),
        create_player_deck(),
    ];

    let mut player_turn: usize = 0;
    loop {
        let current_player_name = player_names[player_turn]
            .as_ref()
            .expect("Player name to be defined.");
        let player_type = player_types[player_turn]
            .as_ref()
            .expect("Player type to be defined.");

        ui::start_player_turn(&current_player_name, &get_player_type_name(&player_type));
        let number_of_dice = match player_type {
            PlayerType::Human => ui::get_number_of_dice(true),
            PlayerType::Computer => 1,
        };

        let (first_die, second_die) = roll_dice(number_of_dice == 2);
        let roll_total = first_die + second_die.unwrap_or_default();
        ui::roll_result(first_die, second_die, roll_total);

        player_turn += 1;
        if player_turn >= player_count {
            player_turn = 0;
        }
    }
}
