mod game;
mod ui;

use crate::game::controller::{create_deck, create_player_deck, Deck};
use crate::game::player::get_player_type_name;
use crate::ui::{get_player_count, get_player_names, get_player_types};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");
    let player_count = get_player_count();
    let player_names = get_player_names(player_count);
    let player_types = get_player_types(player_count, &player_names);
    let card_deck = create_deck();
    let player_cards: [Deck; MAX_PLAYER_COUNT] = [
        create_player_deck(),
        create_player_deck(),
        create_player_deck(),
        create_player_deck(),
    ];

    for i in 0..player_count as usize {
        let card = &player_cards[i][14];
        let card_from_deck = &card_deck[14];
        println!(
            "Name: {}, Type: {}, Kind: {}, Icon: {}, Order: {}, Cost: {}, Count: {}, Deck: {}, Activated on 1: {}",
            player_names[i].as_ref().expect(""),
            get_player_type_name(&player_types[i]),
            card.get_title(),
            card.get_icon_title(),
            card.get_order() as u8,
            card.get_cost(),
            card.count,
            card_from_deck.count,
            card.test_activation(1)
        );
    }
}
