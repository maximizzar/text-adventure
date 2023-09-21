use crate::{console_read, exit};
use crate::locations::set_player_action;

pub(crate) fn cave(next_location: (String, String)) -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 3] = ["cave_inside", "cave", "plains"];

    println!("
    On the way to the cave some stormy clouds rise along the sky and rain begins to pour down.
	The rising storm made it hard to find the entrance of the dark cave.
	The inside of the cave is so dark, that as you stretch your arm inside you lose sight of it.
    ");

    loop {
        player_action = console_read("What you wanna do? ".to_string());
        if player_action.eq("go") {
            return cave_go(player_action, next_location);

        } else if player_action.eq("use") {
            cave_use();

        } else if player_action.eq("exit") {
            exit();
        }
    }

    fn cave_go(mut player_action: String, next_location: (String, String)) -> (String, String) {
        player_action = set_player_action(player_action, Box::from(LOCATIONS));

        if player_action.eq("inside") {
            return ("north".to_string(), "cave_inside".to_string());

        } else if player_action.eq("plains") {
            return ("center".to_string(), "plains".to_string());

        } else if player_action.eq("exit") {
            exit();
        }
        return next_location;
    }
    fn cave_use() {

    }
}

pub(crate) fn cave_inside(next_location: (String, String))  -> (String, String) {
    return next_location;
}