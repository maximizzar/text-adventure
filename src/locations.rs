mod center;
mod east;
mod north;
mod south;
mod west;

pub fn center(mut next_location: (String, String)) -> (String, String) {

    loop {

        if next_location.1.eq("plains") {
            next_location = center::plains();

        } else if next_location.1.eq("forest") {
            return ("south".to_string(), "forest".to_string());

        } else if next_location.1.eq("") {
            return ("".to_string(),"".to_string());
        }
    }
}

pub fn east(mut next_location: (String,String)) -> (String,String) {
    return ("".to_string(),"".to_string());
}
pub fn north(mut next_location: (String,String)) -> (String,String) {
    return ("".to_string(),"".to_string());
}
pub fn south(mut next_location: (String,String)) -> (String,String) {
    return ("".to_string(),"".to_string());
}
pub fn west(mut next_location: (String,String)) -> (String,String) {
    return ("".to_string(),"".to_string());
}

pub fn map(cardinal_point: &str, location: &str) {
    println!("Your located in the {} at the {}", location, cardinal_point);
}