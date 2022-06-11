use tabled::{
    builder::Builder,
    object::{Columns, Segment},
    Alignment, Header, MaxWidth, Modify, Style,
};

use crate::{
    game::{
        cards::{get_card_title, CardIcon, CardKind, CardStack, CARD_KIND_COUNT},
        controller::{create_player_deck, Deck, PlayerCardStack},
        landmarks::{
            get_landmark_cost, get_landmark_description, get_landmark_title, LandmarkKind,
            ALL_LANDMARKS, LANDMARK_KIND_COUNT,
        },
        player::{Player, PlayerKind},
    },
    MAX_PLAYER_COUNT,
};
use std::io;

pub fn get_players() -> (Vec<Player>, usize) {
    let player_count = get_player_count();
    let mut players = Vec::new();

    for turn in 0..player_count {
        let name = get_player_name(turn + 1);
        players.push(Player {
            turn,
            name,
            kind: get_player_kind(),
            cards: create_player_deck(),
            coins: 0,
            landmarks: Vec::new(),
        });
    }
    (players, player_count)
}

pub fn get_roll_two_dice() -> bool {
    println!("How many dice will you roll: (1-2):");
    loop {
        return match get_input().trim().parse() {
            Ok(num) => match num {
                1 => false,
                2 => true,
                _ => {
                    println!("Please specify between 1 and 2 dice:");
                    continue;
                }
            },
            Err(_) => {
                println!("Please specify between 1 and 2 dice:");
                continue;
            }
        };
    }
}

pub fn start_player_turn(player: &Player) {
    println!();
    println!("It's {}'s ({}) turn.", player.name, player.get_kind_name());
}

pub fn roll_result(first: u8, second: Option<u8>, total: u8) {
    println!();
    match second {
        Some(second) => println!("Rolled {} and {} for {}", first, second, total),
        None => println!("Rolled {}", first),
    }
}

pub fn get_player_to_steal_coins_from(
    players: &Vec<Player>,
    player_turn: usize,
    amount: u8,
) -> usize {
    println!(
        "Which player would you like to steal {} coins from:",
        amount
    );
    get_player_except(players, player_turn)
}

pub fn get_player_to_trade_establishment_with(
    players: &Vec<Player>,
    player_turn: usize,
) -> (usize, CardKind, CardKind) {
    loop {
        println!("Which player would you like to trade a establishment with:");
        let other_player_number = get_player_except(players, player_turn);

        let other_player = players
            .get(other_player_number)
            .expect("Selected player to not be OOB");
        let other_player_card = get_non_major_card_kind(&other_player);

        let current_player = players
            .get(player_turn)
            .expect("Selected player to not be OOB");
        let player_card = get_non_major_card_kind(&current_player);

        return (other_player.turn, other_player_card, player_card);
    }
}

pub fn share_post_distribution_results(current_coins: u8, before_coins: u8) {
    let current_coins_i16: i16 = current_coins.into();
    let before_coins_i16: i16 = before_coins.into();
    let delta = current_coins_i16 - &before_coins_i16;
    println!(
        "Coins: {} ({}{})",
        current_coins,
        if delta.is_negative() { "" } else { "+" },
        delta
    );
}

pub fn buy_a_card(
    card_deck: &Deck,
    player: &Player,
) -> Option<(Option<CardKind>, Option<LandmarkKind>)> {
    if player.coins == 0 {
        return None;
    }

    println!();
    println!("Available cards");

    // Print regular cards.
    let mut index: usize = 0;
    let data = card_deck
        .map(|card_stack| {
            let player_card_count = match player.cards.iter().find(|c| c.kind == card_stack.kind) {
                Some(c) => c.count,
                None => 0,
            };
            index += 1;
            vec![
                (index - 1).to_string(),
                card_stack.get_title().to_string(),
                card_stack.get_cost().to_string(),
                player_card_count.to_string(),
                card_stack.count.to_string(),
                card_stack.get_activation_description().to_string(),
                card_stack.get_order_title().to_string(),
                card_stack.get_icon_title().to_string(),
                card_stack.get_description().to_string(),
            ]
        })
        .to_vec();

    print_table(
        "Establishments",
        vec![
            "#",
            "Title",
            "Cost",
            "Owned",
            "Available",
            "Activation",
            "Class",
            "Icon",
            "Description",
        ],
        data,
    );

    // Print unbuilt landmarks.
    let available_landmarks: Vec<LandmarkKind> = ALL_LANDMARKS
        .into_iter()
        .filter(|kind| !player.landmarks.contains(&kind))
        .collect();
    let data = available_landmarks
        .iter()
        .map(|landmark| {
            index += 1;
            vec![
                (index - 1).to_string(),
                get_landmark_title(landmark).to_string(),
                get_landmark_cost(landmark).to_string(),
                get_landmark_description(landmark).to_string(),
            ]
        })
        .collect();

    print_table("Landmarks", vec!["#", "Title", "Cost", "Description"], data);

    println!("Would you like to buy a card? (#, n):");

    loop {
        break match get_input().trim().to_lowercase().as_str() {
            "n" => None,
            input => match input.parse::<usize>() {
                Ok(selected_index) => {
                    if selected_index < CARD_KIND_COUNT {
                        let card = card_deck
                            .get(selected_index)
                            .expect("Card index to be in bounds.");
                        if card.count == 0 {
                            println!(
                                "Sorry, there are no {} left. Please select another option:",
                                card.get_title()
                            );
                            continue;
                        }

                        let cost = card.get_cost();
                        if player.coins < cost {
                            println!(
                            "Sorry, you only have {} coins but need {}. Please select another option:",
                            player.coins,
                            cost
                        );
                            continue;
                        }

                        Some((Some(card.kind), None))
                    } else if selected_index - CARD_KIND_COUNT < available_landmarks.len() {
                        let landmark_index = selected_index - CARD_KIND_COUNT;
                        let landmark = available_landmarks
                            .get(landmark_index)
                            .expect("Landmark index to be in bounds.");

                        let cost = get_landmark_cost(&landmark);
                        if player.coins < cost {
                            println!(
                            "Sorry, you only have {} coins but need {}. Please select another option:",
                            player.coins,
                            cost
                        );
                            continue;
                        }

                        Some((None, Some(landmark.clone())))
                    } else {
                        println!("Invalid option, please select a number from the list:");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Please specifiy either a number or \"n\" to skip.");
                    continue;
                }
            },
        };
    }
}

pub fn show_purchase_decision(
    purchase_decision: &Option<(Option<CardKind>, Option<LandmarkKind>)>,
) {
    match purchase_decision {
        Some((card_kind, landmark_kind)) => match card_kind {
            Some(card) => println!("Bought the {}.", get_card_title(card.clone())),
            None => println!(
                "Bought the {} landmark.",
                get_landmark_title(
                    &landmark_kind
                        .clone()
                        .expect("Expected either a card or a landmark.")
                )
            ),
        },
        None => println!("Didn't buy anything."),
    }
}

pub fn show_activated_cards(cards: &Vec<PlayerCardStack>) {
    if cards.is_empty() {
        println!("No cards activated this turn.");
    } else {
        let mut unique_cards = cards.to_vec();
        unique_cards.sort_by_key(|card| card.card.kind);
        unique_cards.dedup_by_key(|card| card.card.kind);

        print!("Cards activated this turn:");
        for card in unique_cards.iter() {
            print!(" {}", card.card.get_title());
        }
        println!();
    }
}

pub fn ask_reroll() -> bool {
    loop {
        println!("Would you like to re-roll? (y, n):");
        return match get_input().trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => continue,
        };
    }
}

pub fn amusement_park_turn() {
    println!();
    println!("You rolled doubles and have the Amusement Park, take another turn!");
}

pub fn player_has_won(player: &Player) {
    println!();
    print!(
        "{} ({}) has won the game! Congratulations!",
        player.name,
        player.get_kind_name()
    );
}

fn get_player_except(players: &Vec<Player>, except_player_turn: usize) -> usize {
    let player_options: Vec<(usize, String, u8)> = players
        .iter()
        .filter(|player| player.turn != except_player_turn)
        .map(|player| (player.turn, player.name.clone(), player.coins))
        .collect();

    let player_display_options = players
        .iter()
        .filter(|player| player.turn != except_player_turn)
        .enumerate()
        .map(|(index, player)| {
            vec![
                index.to_string(),
                player.name.clone(),
                player.coins.to_string(),
                player
                    .cards
                    .map(|card| card.count)
                    .iter()
                    .sum::<u8>()
                    .to_string(),
            ]
        })
        .collect();

    print_table(
        "Select a player",
        vec!["#", "Name", "Coins", "Cards"],
        player_display_options,
    );

    loop {
        return match get_input().trim().parse::<usize>() {
            Ok(num) => match player_options.get(num) {
                Some(player) => player.0,
                None => {
                    println!("Please specify the player number:");
                    continue;
                }
            },
            Err(_) => {
                println!("Please specify the player number:");
                continue;
            }
        };
    }
}

fn print_table(header: &str, columns: Vec<&str>, data: Vec<Vec<String>>) {
    let table = Builder::from(data)
        .set_columns(columns)
        .build()
        .with(Header(header))
        .with(Style::modern())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::left())
                .with(Alignment::top()),
        )
        .with(Modify::new(Columns::new(0..)).with(MaxWidth::wrapping(70)));
    println!("{}", table);
}

fn get_non_major_card_kind(player: &Player) -> CardKind {
    let card_options: Vec<CardStack> = player
        .cards
        .into_iter()
        .filter(|card| card.count > 0 && card.get_icon() != CardIcon::Major)
        .collect();

    let mut index: usize = 0;
    let data = card_options
        .iter()
        .map(|card_stack| {
            let player_card_count = match player.cards.iter().find(|c| c.kind == card_stack.kind) {
                Some(c) => c.count,
                None => 0,
            };
            index += 1;
            vec![
                (index - 1).to_string(),
                card_stack.get_title().to_string(),
                card_stack.get_cost().to_string(),
                player_card_count.to_string(),
                card_stack.count.to_string(),
                card_stack.get_activation_description().to_string(),
                card_stack.get_order_title().to_string(),
                card_stack.get_icon_title().to_string(),
                card_stack.get_description().to_string(),
            ]
        })
        .collect();

    print_table(
        "Select an establishment",
        vec![
            "#",
            "Title",
            "Cost",
            "Owned",
            "Available",
            "Activation",
            "Class",
            "Icon",
            "Description",
        ],
        data,
    );
    println!("Which card would you like to choose:");

    loop {
        return match get_input().trim().parse::<usize>() {
            Ok(num) => match card_options.get(num) {
                Some(option) => option.kind,
                None => {
                    println!("Please specify the card number:");
                    continue;
                }
            },
            Err(_) => {
                println!("Please specify the card number:");
                continue;
            }
        };
    }
}

fn get_player_kind() -> PlayerKind {
    loop {
        println!("Human or computer (h, c):");
        return match get_input().trim().to_lowercase().as_str() {
            "c" => PlayerKind::RandomAI,
            "h" => PlayerKind::Human,
            _ => continue,
        };
    }
}

fn get_player_name(player_number: usize) -> String {
    loop {
        println!("Enter the name for player {}:", player_number);
        let name = get_input().trim().to_string();
        if !name.is_empty() {
            return name;
        }
    }
}

fn get_player_count() -> usize {
    println!("How many players (2-{}):", MAX_PLAYER_COUNT);
    loop {
        return match get_input().trim().parse() {
            Ok(num) => match num {
                2..=MAX_PLAYER_COUNT => num,
                _ => {
                    println!(
                        "Please specify a player count between 2 and {}: ",
                        MAX_PLAYER_COUNT
                    );
                    continue;
                }
            },
            Err(_) => {
                println!(
                    "Please specify a number between 2 and {}:",
                    MAX_PLAYER_COUNT
                );
                continue;
            }
        };
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}
