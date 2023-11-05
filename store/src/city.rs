use crate::main_city::{MainCity, SubCity};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum C {
    D(MainCity), // Destination
    P(SubCity),  // Path
}

impl C {
    pub fn coordinates(&self) -> geoutils::Location {
        match self {
            C::D(city) => city.coordinates(),
            C::P(city) => city.coordinates(),
        }
    }
}
