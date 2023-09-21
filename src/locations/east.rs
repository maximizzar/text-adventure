use crate::{console_read, exit};
use crate::locations::set_player_action;

pub(crate) fn eastern_barn(next_location: (String, String)) -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 2] = ["eastern_barn", "plains"];

    println!("
        Walking towards the brittle barn you recognize a bearded man with a dog nearby.
        The man does not seem pretty chatty.
        Nevertheless, he hands you a bag with sticks and a hilt.
        !Typhon, a shepherd, is here.
	");

    loop {
        player_action = console_read("What you wanna do? ".to_string());
        if player_action.eq("go") {
            return eastern_barn_go(player_action, next_location);

        } else if player_action.eq("talk") {
            eastern_barn_talk(player_action);

        } else if player_action.eq("take") {
            eastern_barn_take(player_action);

        } else if player_action.eq("exit") {
            exit();
        }
    }

    fn eastern_barn_go(mut player_action: String, next_location: (String, String)) -> (String, String) {
        player_action = set_player_action(player_action, Box::from(LOCATIONS));

        if player_action.eq("plains") {
            return ("center".to_string(), "plains".to_string());

        } else if player_action.eq("exit") {
            exit();
        }
        return next_location;
    }
    fn eastern_barn_talk(mut player_action: String) -> &'static str {
        player_action = console_read("Who you wanna talk to? ".to_string());

        if player_action.eq("self") {
            return "Why do I even exist?";
        } else if player_action.eq("typhon") {
            return "... he recognizes you, but does not seem to want to converse, odd fellow.";

        }
        return "...";
    }
    fn eastern_barn_take(mut player_action: String) -> (String, String) {
        player_action = console_read("What do you wanna take with you? ".to_string());

        if player_action.eq("sticks_and_hilt") {
            println!("
                You take a bag with sticks and a hilt from Typhon.\n\
                After you take them, he nods at you, like saying you'll need them.\
            ");

            //add sticks_and_hilt to inventory
        }

        return ("".to_string(), "".to_string())
    }
}