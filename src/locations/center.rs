use crate::console_read;

pub(crate) fn plains(next_location: (String, String)) -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 5] = ["plains","castle", "cave", "eastern_barn", "southern_forest"];

    println!("
        As you are standing on a great plain you can only see the blue sky with clouds.
        After looking around, you recognize a dark cave in the north.
        In the west you can see a peaky castle behind some mountains.
        A misty and dead forest stretches across the southern horizon and a barn made out of brittle wood in the east.
        A sparkle between the trees in the south catches your eye."
    );

    loop {
        player_action = console_read("What you wanna do? (go, talk, attack)".to_string());
        if player_action.eq("go") {
            return plains_go(player_action, next_location);

        } else if player_action.eq("talk") {
            plains_talk(player_action);

        } else if player_action.eq("attack") {
            plains_attack(player_action);

        }
        else if player_action.eq("exit") {
            std::process::exit(0);
        }
    }

    fn plains_go(mut player_action: String, next_location: (String, String)) -> (String, String) {
        loop {
            player_action = console_read(format!(
                "Select a location you want to go to: {:?}", LOCATIONS));

            if LOCATIONS.contains(&&*player_action) {
                break;
            } else if player_action.eq("exit") {
                return ("exit".to_string(), "exit".to_string())
            }
        }

        if player_action.eq("castle") {
            return ("west".to_string(), "castle".to_string());

        } else if player_action.eq("cave") {
            return ("north".to_string(), "cave".to_string());

        } else if player_action.eq("eastern_barn") {
            return ("east".to_string(), "eastern_barn".to_string())

        } else if player_action.eq("southern_forest") {
            return ("south".to_string(), "southern_forest".to_string())

        }
        return next_location;
    }

    fn plains_talk(mut player_action: String) {
        player_action = console_read("Who you wanna talk to? ".to_string());
    }

    fn plains_attack(mut player_action: String) {
        player_action = console_read("Who you wanna attack? ".to_string());
    }
}
