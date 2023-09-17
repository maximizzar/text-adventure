use crate::locations;
const CURRENT_CARDINAL_POINT: &str = "south";

pub(crate) fn forest() {
    const CURRENT_LOCATION: &str = "forest";
    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);
        break;
    }
}