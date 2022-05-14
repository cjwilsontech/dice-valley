use crate::{
    game::{
        controller::create_player_deck,
        player::{Player, PlayerKind},
    },
    MAX_PLAYER_COUNT,
};
use std::io;

pub fn get_players() -> (Vec<Player>, usize) {
    let player_count = get_player_count();
    let mut players = Vec::new();

    for i in 0..player_count {
        let name = get_player_name(i + 1);
        players.push(Player {
            name,
            kind: get_player_kind(),
            cards: create_player_deck(),
            coins: 0,
        });
    }
    (players, player_count)
}

pub fn get_number_of_dice(can_roll_two_dice: bool) -> u8 {
    if !can_roll_two_dice {
        return 1;
    }

    println!("How many dice will you roll: (1-2):");
    loop {
        return match get_input().trim().parse() {
            Ok(num) => match num {
                1 | 2 => num,
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
    println!("");
    println!("It's {}'s ({}) turn.", player.name, player.get_kind_name());
}

pub fn roll_result(first: u8, second: Option<u8>, total: u8) {
    match second {
        Some(second) => println!("Rolled {} and {} for {}", first, second, total),
        None => println!("Rolled {}", first),
    }
}

fn get_player_kind() -> PlayerKind {
    loop {
        println!("Human or computer (h, c):");
        return match get_input().trim().to_lowercase().as_str() {
            "c" => PlayerKind::Computer,
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
