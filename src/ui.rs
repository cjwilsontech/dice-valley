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

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}
