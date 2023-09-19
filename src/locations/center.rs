use crate::console_read;

pub(crate) fn plains() -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 2] = ["plains","forest"];

    println!("
        As you are standing on a great plain you can only see the blue sky with clouds.
        After looking around, you recognize a dark cave in the north.
        In the west you can see a peaky castle behind some mountains.
        A misty and dead forest stretches across the southern horizon and a barn made out of brittle wood in the east.
        A sparkle between the trees in the south catches your eye."
    );

    loop {
        player_action = console_read("What you wanna do? ".to_string());
        if player_action.eq("go") {
            return plains_go(player_action);

        } else if player_action.eq("talk") {
            plains_talk(player_action);

        } else if player_action.eq("attack") {
            plains_attack(player_action);

        }
        else if player_action.eq("exit") {
            std::process::exit(0);
        }
    }

    fn plains_go(mut player_action: String) -> (String, String) {
        loop {
            player_action = console_read(format!("Select a location you want to go to: {:?}", LOCATIONS));

            if player_action.eq("forest") {
                return ("south".to_string(), "forest".to_string());

            } else if player_action.eq("plains") {
                break;

            }
        }
        return ("".to_string(), "".to_string())
    }

    fn plains_talk(mut player_action: String) {
        player_action = console_read("Who you wanna talk to? ".to_string());
    }

    fn plains_attack(mut player_action: String) {
        player_action = console_read("Who you wanna attack? ".to_string());

    }
}
