use store::{
    rail_road::{Deed, C},
    sub_city::SubCity,
};

fn main() {
    print!(
        "{:?}",
        Deed::NYC.rail_roads().get(&C::P(SubCity::Perrysburg_OH))
    )
}
