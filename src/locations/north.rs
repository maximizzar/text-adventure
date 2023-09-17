use crate::locations;
const CURRENT_CARDINAL_POINT: &str = "north";
pub(crate) fn cave() {
    const CURRENT_LOCATION: &str = "cave";
    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);
        break;
    }
}
pub(crate) fn cave_inside() {
    const CURRENT_LOCATION: &str = "cave(inside)";
    loop {
        locations::map(CURRENT_LOCATION, CURRENT_CARDINAL_POINT);
        break;
    }
}