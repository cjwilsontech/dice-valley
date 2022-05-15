mod game;
mod ui;

use crate::game::{
    cards::{CardIcon, CardKind},
    controller::{
        award_coins, award_coins_combo, create_deck, get_activatable_cards, roll_dice, steal_coins,
        steal_coins_from_all, trade_establishments,
    },
    player::PlayerKind,
};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");

    let (mut players, player_count) = ui::get_players();
    let card_deck = create_deck();

    let mut player_turn: usize = 0;
    loop {
        let current_player = players.get(player_turn).expect("Player to not be OOB.");
        ui::start_player_turn(&current_player);

        let number_of_dice = match current_player.kind {
            PlayerKind::Human => ui::get_number_of_dice(true),
            PlayerKind::Computer => 1,
        };

        let (first_die, second_die) = roll_dice(number_of_dice == 2);
        let roll_total = first_die + second_die.unwrap_or_default();
        ui::roll_result(first_die, second_die, roll_total);

        let activatable_cards = get_activatable_cards(roll_total, player_turn, &players);

        let before_coins = current_player.coins;
        for player_card in activatable_cards {
            for _ in 0..player_card.card.count {
                match player_card.card.kind {
                    CardKind::AppleOrchard => award_coins(&mut players, player_turn, 3),
                    CardKind::Bakery => award_coins(&mut players, player_turn, 1),
                    CardKind::BusinessCenter => {
                        let (other_player, other_player_card_kind, player_card_kind) =
                            ui::get_player_to_trade_establishment_with(&players, player_turn);
                        trade_establishments(
                            &mut players,
                            player_turn,
                            other_player,
                            player_card_kind,
                            other_player_card_kind,
                        );
                        0
                    }
                    CardKind::Cafe => {
                        steal_coins(&mut players, player_turn, player_card.player_turn, 1)
                    }
                    CardKind::CheeseFactory => {
                        award_coins_combo(&mut players, player_turn, CardIcon::Cow, 3)
                    }
                    CardKind::ConvenienceStore => award_coins(&mut players, player_turn, 3),
                    CardKind::FamilyRestaurant => {
                        steal_coins(&mut players, player_turn, player_card.player_turn, 2)
                    }
                    CardKind::Forest => award_coins(&mut players, player_turn, 1),
                    CardKind::FruitAndVegetableMarket => {
                        award_coins_combo(&mut players, player_turn, CardIcon::Wheat, 2)
                    }
                    CardKind::FurnitureFactory => {
                        award_coins_combo(&mut players, player_turn, CardIcon::Gear, 3)
                    }
                    CardKind::Mine => award_coins(&mut players, player_turn, 5),
                    CardKind::Ranch => award_coins(&mut players, player_turn, 1),
                    CardKind::Stadium => {
                        steal_coins_from_all(&mut players, player_turn, player_count, 2)
                    }
                    CardKind::TvStation => {
                        let from_player =
                            ui::get_player_to_steal_coins_from(&players, player_turn, 5);
                        steal_coins(&mut players, from_player, player_turn, 5)
                    }
                    CardKind::WheatField => award_coins(&mut players, player_turn, 1),
                };
            }
        }

        let current_player = players.get(player_turn).expect("Player to not be OOB.");
        ui::share_post_distribution_results(current_player.coins, before_coins);

        player_turn += 1;
        if player_turn >= player_count {
            player_turn = 0;
        }
    }
}
