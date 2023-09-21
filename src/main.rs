use crate::locations::locations;

mod locations;
fn main() {
    let mut next_location: (String, String) = (String::from("center"), String::from("plains"));
    println!("{}", start_screen().1);

    loop {
        next_location = locations(next_location);
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
fn start_screen() -> (&'static str, String) {
    return match text_to_ascii_art::convert("text-adventure".to_string()) {
        Ok(string) => ("{}", string),
        Err(err) => ("Error: {}", err),
    }
}
fn exit() {
    println!("Thank you for playing. =)");
    std::process::exit(0);
}