use crate::{game::player::PlayerType, MAX_PLAYER_COUNT};
use std::io;

pub fn get_player_types(
    player_count: usize,
    player_names: &[Option<String>; MAX_PLAYER_COUNT],
) -> [Option<PlayerType>; MAX_PLAYER_COUNT] {
    let mut player_types: [Option<PlayerType>; MAX_PLAYER_COUNT] = Default::default();
    for i in 1..=player_count as usize {
        while player_types[i - 1].is_none() {
            println!(
                "Is the player {} a human or computer (h, c):",
                player_names[i - 1].as_ref().expect("Name array OOB")
            );
            player_types[i - 1] = match get_input().trim().to_lowercase().as_str() {
                "c" => Some(PlayerType::Computer),
                "h" => Some(PlayerType::Human),
                _ => continue,
            };
        }
    }
    player_types
}

pub fn get_player_names(player_count: usize) -> [Option<String>; MAX_PLAYER_COUNT] {
    let mut names: [Option<String>; MAX_PLAYER_COUNT] = Default::default();
    for i in 1..=player_count {
        while names[i - 1].is_none() {
            println!("Enter the name for player {}:", i);
            let name = get_input().trim().to_string();
            if !name.is_empty() {
                names[i - 1] = Some(name);
            }
        }
    }
    names
}

pub fn get_player_count() -> usize {
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

pub fn get_number_of_dice(can_roll_two_dice: bool) -> u8 {
    if (!can_roll_two_dice) {
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

pub fn start_player_turn(player_name: &String, player_type: &str) {
    println!("");
    println!("It's {}'s ({}) turn.", player_name, player_type);
}

pub fn roll_result(first: u8, second: Option<u8>, total: u8) {
    match second {
        Some(second) => println!("Rolled {} and {} for {}", first, second, total),
        None => println!("Rolled {}", first),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}
