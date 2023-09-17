use crate::locations;
const CURRENT_CARDINAL_POINT: &str = "west";
pub(crate) fn castle() {
    const CURRENT_LOCATION: &str = "castle";
    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);
        break;
    }
}
pub(crate) fn castle_tent() {
    const CURRENT_LOCATION: &str = "tent(castle)";
    loop {
        locations::map(CURRENT_LOCATION, CURRENT_CARDINAL_POINT);
        break;
    }
}