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

enum AllActivitiesHealth {
    Cooking,
    Laundry,
    ElectricalRepair,
    Mowing,
    Sleeping,
}

fn add_activity_head(activity_health: AllActivitiesHealth) -> u32 {
    match activity_health {
        AllActivitiesHealth::Cooking => 3,
        AllActivitiesHealth::Laundry => 3,
        AllActivitiesHealth::ElectricalRepair => 6,
        AllActivitiesHealth::Mowing => 6,
        AllActivitiesHealth::Sleeping => 20,
    }
}

// fn goto_chores(destination: &Destination, game_stats: &mut GameStats) {
//     let chores_message = vec![
//         "Choose a chore from the following:".to_string(),
//         "1. Cooking [+3]".to_string(),
//         "2. Laundry [+3]".to_string(),
//         "3. Electrical Repair [+6]".to_string(),
//         "4. Mowing +[6]".to_string(),
//     ];

//     print_destination_requirments(&game_stats, &chores_message);
// }

// fn goto_sleep(destination: &Destination, game_stats: &mut GameStats) {}

// fn goto_trivia(destination: &Destination, game_stats: &mut GameStats) {}

fn perfom_taks(destination: &Destination, game_stats: &mut GameStats, choice: u8) {
    let chores_rules = vec![
        "Choose a chore from the following:".to_string(),
        "1. Cooking [+3]".to_string(),
        "2. Laundry [+3]".to_string(),
        "3. Electrical Repair [+6]".to_string(),
        "4. Mowing +[6]".to_string(),
    ];
    let sleeping_rules = Vec::new();
    let trivia_rules = Vec::new();

    if choice == 1 {
        print_destination_requirments(&game_stats, &chores_rules);
    } else if choice == 2 {
        print_destination_requirments(&game_stats, &sleeping_rules);
    } else {
        print_destination_requirments(&game_stats, &trivia_rules);
    }

    let chore_lenght = chores_rules.len() - 1;

    let user_choice = take_user_choice(chore_lenght);

    let cooking = AllActivitiesHealth::Cooking;
    let laundry = AllActivitiesHealth::Cooking;
    let electrical_repair = AllActivitiesHealth::ElectricalRepair;
    let mowing = AllActivitiesHealth::Mowing;
    let sleeping = AllActivitiesHealth::Sleeping;
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
        // if choice == 1 {
        //     goto_chores(&destination, &mut game_stats);
        // } else if choice == 2 {
        //     goto_sleep(&destination, &mut game_stats);
        // } else if choice == 3 {
        //     goto_trivia(&destination, &mut game_stats);
        // }
    } else {
        dbg!("You loose");
    }
}

fn main() {
    chores_game_init();
}
