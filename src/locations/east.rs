use crate::locations;
const CURRENT_CARDINAL_POINT: &str = "east";
pub(crate) fn barn() {
    const CURRENT_LOCATION: &str = "barn";
    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);
        break;
    }
}