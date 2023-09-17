use crate::locations;

pub fn plains() {
    const CURRENT_LOCATION: &str = "plains";
    const CURRENT_CARDINAL_POINT: &str = "center";

    println!("
        As you are standing on a great plain you can only see the blue sky with clouds.
        After looking around, you recognize a dark cave in the north.
        In the west you can see a peaky castle behind some mountains.
        A misty and dead forest stretches across the southern horizon and a barn made out of brittle wood in the east.
        A sparkle between the trees in the south catches your eye."
    );

    let mut player_action:String = String::new();

    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);

        if player_action.contains("go.south.forest") {
            locations::south::forest();
        } else if player_action.contains("go.east.barn") {
            locations::east::barn();
        } else if player_action.contains("go.west.castle") {
            locations::west::castle();
        } else {
            println!("Unknown location. Please try again!");
        }
        break;
    }
}