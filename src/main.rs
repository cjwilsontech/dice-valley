mod game;
mod ui;

use crate::game::{
    cards::{CardIcon, CardKind},
    controller::{
        award_coins, award_coins_combo, buy_card_from_deck, buy_landmark, create_deck,
        get_activatable_cards, roll_dice, steal_coins, steal_coins_from_all, trade_establishments,
    },
    landmarks::LandmarkKind,
    player::PlayerKind,
};

pub const MAX_PLAYER_COUNT: usize = 4;

fn main() {
    println!("Dice Valley");

    let (mut players, player_count) = ui::get_players();
    let mut card_deck = create_deck();

    let mut current_turn: usize = 0;
    loop {
        let player = players.get(current_turn).expect("Player to not be OOB.");
        ui::start_player_turn(&player);

        let number_of_dice = match player.kind {
            PlayerKind::Human => {
                if player.landmarks.contains(&LandmarkKind::TrainStation) {
                    ui::get_number_of_dice(true)
                } else {
                    1
                }
            }
            PlayerKind::Computer => 1,
        };

        let (mut first_die, mut second_die) = roll_dice(number_of_dice == 2);
        let mut roll_total = first_die + second_die.unwrap_or_default();
        ui::roll_result(first_die, second_die, roll_total);

        if player.landmarks.contains(&LandmarkKind::RadioTower) && ui::ask_reroll() {
            (first_die, second_die) = roll_dice(number_of_dice == 2);
            roll_total = first_die + second_die.unwrap_or_default();
            ui::roll_result(first_die, second_die, roll_total);
        }

        let activatable_cards = get_activatable_cards(roll_total, current_turn, &players);
        ui::show_activated_cards(&activatable_cards);

        let before_coins = player.coins;
        for card_stack in activatable_cards {
            let shopping_mall_bonus = if (card_stack.card.get_icon() == CardIcon::Bread
                || card_stack.card.get_icon() == CardIcon::Cup)
                && players
                    .get(card_stack.owner_turn)
                    .expect("Player to not be OOB.")
                    .landmarks
                    .contains(&LandmarkKind::ShoppingMall)
            {
                1
            } else {
                0
            };
            for _ in 0..card_stack.card.count {
                match card_stack.card.kind {
                    CardKind::AppleOrchard => award_coins(&mut players, card_stack.owner_turn, 3),
                    CardKind::Bakery => {
                        award_coins(&mut players, card_stack.owner_turn, 1 + shopping_mall_bonus)
                    }
                    CardKind::BusinessCenter => {
                        let (other_player, other_player_card_kind, player_card_kind) =
                            ui::get_player_to_trade_establishment_with(
                                &players,
                                card_stack.owner_turn,
                            );
                        trade_establishments(
                            &mut players,
                            card_stack.owner_turn,
                            other_player,
                            player_card_kind,
                            other_player_card_kind,
                        );
                        0
                    }
                    CardKind::Cafe => steal_coins(
                        &mut players,
                        current_turn,
                        card_stack.owner_turn,
                        1 + shopping_mall_bonus,
                    ),
                    CardKind::CheeseFactory => {
                        award_coins_combo(&mut players, card_stack.owner_turn, CardIcon::Cow, 3)
                    }
                    CardKind::ConvenienceStore => {
                        award_coins(&mut players, card_stack.owner_turn, 3 + shopping_mall_bonus)
                    }
                    CardKind::FamilyRestaurant => steal_coins(
                        &mut players,
                        current_turn,
                        card_stack.owner_turn,
                        2 + shopping_mall_bonus,
                    ),
                    CardKind::Forest => award_coins(&mut players, card_stack.owner_turn, 1),
                    CardKind::FruitAndVegetableMarket => {
                        award_coins_combo(&mut players, card_stack.owner_turn, CardIcon::Wheat, 2)
                    }
                    CardKind::FurnitureFactory => {
                        award_coins_combo(&mut players, card_stack.owner_turn, CardIcon::Gear, 3)
                    }
                    CardKind::Mine => award_coins(&mut players, card_stack.owner_turn, 5),
                    CardKind::Ranch => award_coins(&mut players, card_stack.owner_turn, 1),
                    CardKind::Stadium => {
                        steal_coins_from_all(&mut players, card_stack.owner_turn, player_count, 2)
                    }
                    CardKind::TvStation => {
                        let from_player =
                            ui::get_player_to_steal_coins_from(&players, card_stack.owner_turn, 5);
                        steal_coins(&mut players, from_player, card_stack.owner_turn, 5)
                    }
                    CardKind::WheatField => award_coins(&mut players, card_stack.owner_turn, 1),
                };
            }
        }

        let player = players.get(current_turn).expect("Player to not be OOB.");
        ui::share_post_distribution_results(player.coins, before_coins);

        let purchase_decision = ui::buy_a_card(&card_deck, &player);
        if purchase_decision.is_some() {
            let (card_purchase, landmark_purchase) = purchase_decision.unwrap();
            match card_purchase {
                Some(card_kind) => {
                    buy_card_from_deck(&mut players, current_turn, &mut card_deck, card_kind)
                }
                None => buy_landmark(
                    &mut players,
                    current_turn,
                    landmark_purchase.expect("Expected either a card or a landmark."),
                ),
            }
        }

        let player = players.get(current_turn).expect("Player to not be OOB.");
        if player.landmarks.contains(&LandmarkKind::AmusementPark) && Some(first_die) == second_die
        {
            ui::amusement_park_turn();
            continue;
        }

        current_turn += 1;
        if current_turn >= player_count {
            current_turn = 0;
        }
    }
}
