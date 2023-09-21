use crate::{console_read, exit};

mod center;
mod east;
mod north;
mod south;
mod west;

pub fn locations(next_location: (String, String)) -> (String, String) {
    if next_location.0.eq("center") {
        return center(next_location);

    } else if next_location.0.eq("east") {
        return east(next_location);

    } else if next_location.0.eq("north") {
        return north(next_location);

    } else if next_location.0.eq("south") {
        return south(next_location);

    } else if next_location.0.eq("west") {
        return west(next_location);

    } else if next_location.0.eq("exit") {
        exit();
    }
    panic!("invalid cardinal location!")
}

pub fn center(next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("plains") {
        return center::plains(next_location);
    }
    panic!("invalid location for center!")
}

pub fn east(next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("eastern_barn") {
        return east::eastern_barn(next_location);

    }
    panic!("invalid location for east!")
}

pub fn north(next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("cave") {
        return north::cave(next_location);

    } else if next_location.1.eq("cave_inside") {
        return north::cave_inside(next_location);

    }
    panic!("invalid location for north!")
}

pub fn south(next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("southern_forest") {
        return south::southern_forest(next_location);

    }
    panic!("invalid location for south!")
}

pub fn west(next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("castle") {
        return west::castle(next_location);

    } else if next_location.1.eq("castle_tent") {
        return west::castle_tent(next_location);

    }
    panic!("invalid location for west!")

}

fn set_player_action(mut player_action: String, locations: Box<[&'static str]>) -> String {
    loop {
        player_action = console_read(format!(
            "Select a location you want to go to: {:?}", locations));

        if locations.contains(&&*player_action) {
            return player_action;
        }
    }
}