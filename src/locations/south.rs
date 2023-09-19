use crate::console_read;

pub(crate) fn forest() -> (String, String) {
    let mut player_action: String = String::new();
    const LOCATIONS: [&'static str; 1] = ["plains"];

    println!("
    As you wander further into the dark and gloomy forest the trees begin to look even worse, like they've gotten there life sucked out of them and the mist seems to thicken.
	You wander until you can make out a single person standing in the midst of it all. The air seems to get heavier.
	!Ghorkan, the Master of Forest, is here.");
    loop {
        player_action = console_read("What you wanna do? ".to_string());
        if player_action.to_lowercase().eq("go") {
            player_action = console_read(format!("Select a location you want to go to: {:?}", LOCATIONS));
            if player_action.eq("plains") {
                return ("center".to_string(), "plains".to_string());
            }
        }
    }
}
