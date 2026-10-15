/*
 *File:  main.rs
 *Build: cargo build  || rustc main.rs
 *run:   cargo run   || ./main
*/

#![allow(unused)]

use std::io::{self, Write};

fn flush_output() {
    let _ = io::stdout().flush();
}

const COOKING_HEALTH: u32 = 3;
const LAUNDRY_HEALTH: u32 = 3;
const ELECTRICAL_REPAIR_HEALTH: u32 = 6;
const MOWING_HEALTH: u32 = 6;
const COOKING_COINS: u32 = 3;
const LAUNDRY_COINS: u32 = 3;
const ELECTRICAL_REPAIR_COINS: u32 = 3;
const MOWING_COINS: u32 = 3;
const SLEEP_HEALTH: u32 = 3;
const SLEEPING_COINS: u32 = 3;

fn print_game_rules() {
    println!("Welcome to Chores game");
    println!("Select [task] [sleep] or [trivia] by entering their number index");
    println!("Each activity reduces your health and adds coints to chest");
    println!("When either chest or health hits [0], you DIE!");
    println!();
}

struct GameStats {
    health: u32,
    chest: u32,
}

struct Destination {
    destination: Vec<String>,
}

fn take_user_choice(max_len: usize) -> u8 {
    let mut buffer = String::new();

    print!("> ");
    flush_output();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let buffer = buffer
        .trim()
        .parse::<u8>()
        .expect("Failed to convert to u8");
    println!();

    if buffer > max_len as u8 {
        // Default to option one if user enters nummber
        return 1; // above acceptable range.
    } else {
        return buffer;
    }
}

fn initialize_destination() -> Destination {
    Destination {
        destination: vec![
            "Chores".to_string(),
            "Sleep".to_string(),
            "Trivia".to_string(),
        ],
    }
}

fn set_destination() -> (u8, Destination) {
    let destination = initialize_destination();

    let number_of_destination = destination.destination.len();
    let mut index = 1;
    for dest in &destination.destination {
        println!("{index}. {dest}");
        index += 1;
    }

    println!();

    (take_user_choice(number_of_destination), destination)
}

fn print_destination_requirments(game_stats: &GameStats, message: &Vec<String>) {
    println!("---------------------------------------------------------------");
    println!("Current game stats");
    println!("---------------------------------------------------------------");
    println!(
        "[Health]: [{}] | [chest]: [{}]",
        game_stats.health, game_stats.chest
    );
    println!();
    for mesg in message {
        println!("{mesg}");
    }
}

enum AllActivitiesAdditions {
    Cooking(u32, u32),
    Laundry(u32, u32),
    ElectricalRepair(u32, u32),
    Mowing(u32, u32),
    Sleeping(u32, u32),
    Trivia(u32, u32),
}

fn perform_chores(chores_stats: &[AllActivitiesAdditions]) {}

fn perfom_taks(destination: &Destination, game_stats: &mut GameStats, choice: u8) {
    let chores_rules = vec![
        "Choose a chore from the following:".to_string(),
        "1. Cooking [+3]".to_string(),
        "2. Laundry [+3]".to_string(),
        "3. Electrical Repair [+6]".to_string(),
        "4. Mowing +[6]".to_string(),
    ];
    println!("We got here: call");
    let sleeping_rules = vec!["Slept:-. [H: +20] [C: -20]".to_string()];
    //let trivia_rules = Vec::new();

    // if choice == 1 {
    //     print_destination_requirments(&game_stats, &chores_rules);
    // } else if choice == 2 {
    //     print_destination_requirments(&game_stats, &sleeping_rules);
    // } else {
    //     print_destination_requirments(&game_stats, &trivia_rules);
    // }
    println!("We got here: 135");
    let chore_lenght = chores_rules.len() - 1;
    println!("We got here: 137");

    // let user_choice = take_user_choice(chore_lenght);
    println!("We got here: 140");
    let chores_stats = vec![
        AllActivitiesAdditions::Cooking(COOKING_HEALTH, COOKING_COINS),
        AllActivitiesAdditions::Laundry(LAUNDRY_HEALTH, LAUNDRY_COINS),
        AllActivitiesAdditions::ElectricalRepair(ELECTRICAL_REPAIR_HEALTH, ELECTRICAL_REPAIR_COINS),
        AllActivitiesAdditions::Mowing(MOWING_HEALTH, MOWING_COINS),
    ];
    println!("We got here: 147");
    let sleep_health = vec![AllActivitiesAdditions::Sleeping(
        SLEEP_HEALTH,
        SLEEPING_COINS,
    )];
    println!("We got here: 152");
    let choice_tracker = 1;
    println!("We got here: 2");
    for item in &chores_stats {
        println!("We got here: 1");
        if choice == choice_tracker {
            println!("We got here");
            print_destination_requirments(&game_stats, &chores_rules);
            perform_chores(&chores_stats);
        }
    }
}

fn chores_game_init() {
    print_game_rules();
    /*
     *Initialization of inital values of health and chest count.
     */
    let mut game_stats = GameStats {
        health: 20,
        chest: 20,
    };

    let (choice, destination) = set_destination();

    /*
    *Warning:   Comparison values are hardcoded and need to be manually editied
                in any addition in v0.1.0
    */
    if game_stats.chest > 0 && game_stats.health > 0 {
        perfom_taks(&destination, &mut game_stats, choice);
        println!("We got here: send param");
    } else {
        dbg!("You loose");
    }
}

fn main() {
    chores_game_init();
}
