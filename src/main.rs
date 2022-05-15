mod game;
mod ui;

use crate::game::{
    controller::{create_deck, get_activatable_cards, roll_dice},
    player::PlayerKind,
};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");

    let (players, player_count) = ui::get_players();
    let card_deck = create_deck();

    let mut player_turn: usize = 0;
    loop {
        let current_player = players
            .get(player_turn)
            .expect("Current player to be in bounds.");
        ui::start_player_turn(&current_player);

        let number_of_dice = match current_player.kind {
            PlayerKind::Human => ui::get_number_of_dice(true),
            PlayerKind::Computer => 1,
        };

        let (first_die, second_die) = roll_dice(number_of_dice == 2);
        let roll_total = first_die + second_die.unwrap_or_default();
        ui::roll_result(first_die, second_die, roll_total);

        let activatable_cards = get_activatable_cards(roll_total, player_turn, &players);

        player_turn += 1;
        if player_turn >= player_count {
            player_turn = 0;
        }
    }
}
