use serde::{Deserialize, Serialize};

pub use crate::deed::Deed;
pub use crate::state::State;
pub use crate::sub_city::SubCity;

macro_rules! cities {
    ($($c:tt: $s:tt => [$($nc:tt: [$($d:tt),*$(,)?]),*$(,)?] => [$($nsc:tt: [$($sd:tt),*$(,)?]),*$(,)?]),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
            #[allow(non_camel_case_types)]
            pub enum City { $([<$c _ $s>]),* }
            impl City {
                pub const fn cities() -> &'static [Self] {
                    &[$(Self::[<$c _ $s>]),*]
                }

                pub const fn state(&self) -> State {
                    match self {
                        $(Self::[<$c _ $s>] => State::$s),*
                    }
                }

                pub const fn neighbor_cities(&self) -> &[(Self, &[Deed])] {
                    match self {
                        $(Self::[<$c _ $s>] => {
                            &[$((
                                City::$nc,
                                &[$(Deed::$d),*],
                            )),*]
                        }),*
                    }
                }

                pub const fn neighbor_sub_cities(&self) -> &[(SubCity, &[Deed])] {
                    match self {
                        $(Self::[<$c _ $s>] => {
                            &[$((
                                SubCity::$nsc,
                                &[$(Deed::$sd),*],
                            )),*]
                        }),*
                    }
                }
            }
        }
    }
}

cities! {
    Albany: NY => [] => [],
    Atlanta: GA => [] => [],
    Baltimore: MD => [] => [],
    Billings: MT => [] => [],
    Birmingham: AL => [] => [],
    Boston: MA => [] => [Concord_NH: [B_AND_M], Springfield_MA: [B_AND_M], Providence_RI: [NYNH_AND_H]], // Done
    Buffalo: NY => [] => [],
    Butte: MT => [] => [],
    Casper: WY => [] => [],
    Charleston: SC => [] => [],
    Charlotte: TN => [] => [],
    Chattanooga: TN => [] => [],
    Chicago: IL => [] => [],
    Cincinnati: OH => [] => [],
    Cleveland: OH => [] => [],
    Columbus: OH => [] => [],
    Dallas: TX => [] => [],
    Denver: CO => [] => [],
    Des_Moines: IA => [] => [],
    Detroit: MI => [] => [],
    El_Paso: TX => [] => [],
    Fargo: ND => [] => [],
    Fort_Worth: TX => [] => [],
    Houston: TX => [] => [],
    Indianapolis: IN => [] => [],
    Jacksonville: FL => [] => [],
    Kansas_City: MO => [] => [],
    Knoxville: TN => [] => [],
    Las_Vegas: NV => [] => [],
    Little_Rock: AK => [] => [],
    Los_Angeles: CA => [] => [],
    Louisville: KY => [] => [],
    Memphis: TN => [] => [],
    Miami: FL => [] => [],
    Milwaukee: WI => [] => [],
    Minneapolis: MN => [] => [],
    Mobile: AL => [] => [],
    Nashville: TN => [] => [],
    New_Orleans: LA => [] => [],
    New_York: NY => [] => [],
    Norfolk: VA => [] => [],
    Oklahoma_City: OK => [] => [],
    Omaha: NE => [] => [],
    Philadelphia: PA => [] => [],
    Phoenix: AZ => [] => [],
    Pittsburgh: PA => [] => [],
    Pocatello: ID => [] => [],
    Portland: ME => [] => [Concord_NH: [B_AND_M]], // Done
    Portland: OR => [] => [],
    Pueblo: CO => [] => [],
    Rapid_City: SD => [] => [],
    Reno: NV => [] => [],
    Richmond: VA => [] => [],
    Sacramento: CA => [] => [],
    Salt_Lake_City: UT => [] => [],
    San_Antonio: TX => [] => [],
    San_Diego: CA => [] => [],
    San_Francisco: CA => [] => [],
    Seattle: WA => [] => [],
    Shreveport: LA => [] => [],
    Spokane: WA => [] => [],
    St_Louis: MO => [] => [],
    Tampa: FL => [] => [],
    Tucumcari: NM => [] => [],
    Washington: DC => [] => [],
}
