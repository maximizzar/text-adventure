mod center;
mod east;
mod north;
mod south;
mod west;

pub fn locations(mut next_location: (String, String)) -> (String, String) {
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
        return next_location;
    }
    panic!("invalid cardinal location!")
}

pub fn center(mut next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("plains") {
        return center::plains(next_location);
    }
    panic!("invalid location for center!")
}

pub fn east(mut next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("eastern_barn") {
        return east::eastern_barn(next_location);

    }
    panic!("invalid location for east!")
}

pub fn north(mut next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("cave") {
        return north::cave(next_location);

    } else if next_location.1.eq("cave_inside") {
        return north::cave_inside(next_location);

    }
    panic!("invalid location for north!")
}

pub fn south(mut next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("southern_forest") {
        return south::southern_forest(next_location);

    }
    panic!("invalid location for south!")
}

pub fn west(mut next_location: (String,String)) -> (String,String) {
    if next_location.1.eq("castle") {
        return west::castle(next_location);

    } else if next_location.1.eq("castle_tent") {
        return west::castle_tent(next_location);

    }


    panic!("invalid location for west!")

}

pub fn map(cardinal_point: &str, location: &str) {
    println!("Your located in the {} at the {}", location, cardinal_point);
}