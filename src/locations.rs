pub(crate) mod center;
pub(crate) mod east;
pub(crate) mod north;
pub(crate) mod south;
pub(crate) mod west;
pub fn map(cardinal_point: &str, location: &str) {
    println!("Your located in the {} at the {}", location, cardinal_point);
}