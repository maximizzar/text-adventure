mod locations;
pub const LOCATIONS_CENTER: [&str;1] = ["plains"];
pub const LOCATIONS_NORTH: [&str;1] = [""];

fn main() {
    let mut next_location: (String, String) = (String::from("center"), String::from("plains"));
    loop {
        if next_location.0.eq("center") {
            next_location = locations::center(next_location);
        } else if next_location.0.eq("east") {
            next_location = locations::east(next_location);
        } else if next_location.0.eq("north") {
            next_location = locations::north(next_location);
        } else if next_location.0.eq("south") {
            next_location = locations::south(next_location);
        } else if next_location.0.eq("west") {
            next_location = locations::west(next_location);
        } else {
            return;
        }
    }
}
fn console_read(input_prompt_text: String) -> String {
    use std::io::Write;
    let mut input_string: String = String::new();
    print!("{}: ", input_prompt_text);

    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input_string).expect("Did not enter a correct string");

    if let Some('\n') = input_string.chars().next_back() {
        input_string.pop();
    }
    if let Some('\r') = input_string.chars().next_back() {
        input_string.pop();
    }
    return input_string;
}