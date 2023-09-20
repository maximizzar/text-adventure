use crate::console_read;

pub(crate) fn eastern_barn(next_location: (String, String)) -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 1] = ["plains"];

    println!("
    Walking towards the brittle barn you recognize a bearded man with a dog nearby.
    The man does not seem pretty chatty.
    Nevertheless, he hands you a bag with sticks and a hilt.
	!Typhon, a shepherd, is here.
	");

    loop {
        player_action = console_read("What you wanna do? ".to_string());
        if player_action.eq("go") {
            return eastern_barn_go(player_action);

        } else if player_action.eq("talk") {
            eastern_barn_talk(player_action);

        } else if player_action.eq("take") {
            eastern_barn_take(player_action);

        }
        else if player_action.eq("exit") {
            std::process::exit(0);
        }
    }

    fn eastern_barn_go(mut player_action: String) -> (String, String) {
        loop {
            player_action = console_read(format!(
                "Select a location you want to go to: {:?}", LOCATIONS));
        }

        return ("".to_string(), "".to_string())
    }
    fn eastern_barn_talk(mut player_action: String) -> (String, String) {
        return ("".to_string(), "".to_string())
    }
    fn eastern_barn_take(mut player_action: String) -> (String, String) {
        return ("".to_string(), "".to_string())
    }
}

