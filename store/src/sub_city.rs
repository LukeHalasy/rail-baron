pub use crate::city::City;
pub use crate::state::State;

use serde::{Deserialize, Serialize};

// ($($c:tt: $s:tt => [$($nc:tt: [$($d:tt),*$(,)?]),*$(,)?] => [$($nsc:tt: [$($sd:tt),*$(,)?]),*$(,)?]),*$(,)?) => {
macro_rules! sub_cities {
    ($($c:tt: $s:tt),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum SubCity { $([<$c _ $s>]),* }
            impl SubCity {
                pub const fn sub_cities() -> &'static [Self] {
                    &[$(Self::[<$c _ $s>]),*]
                }

                pub const fn state(&self) -> State {
                    match self {
                        $(Self::[<$c _ $s>] => State::$s),*
                    }
                }
            }
        }
    }
}

// I think I should get rid of the railroad part of this..
sub_cities! {
    // North East
    Concord: NH,
    Springfield: MA,
    Providence: RI,
    New_Haven: CT,
    Kingston: NY,
    Syracuse: NY,
    Rochester: NY,
    Erie: PA,
    Trenton: NJ,
    Pottstown: PA,
    Lancaster: PA,
    Bedford: PA,
    Uniontown: PA,
    Frederick: MD,

    // Canada :)
    Brantford: ON,
    London: ON,

    // Mid-West
    Youngstown: OH,
    Akron: OH,
    New_Philadelphia: OH,
    Fremont: OH,
    Findlay: OH,
    Chillicothe: OH,
    Winchester: OH,
    Maysville: OH,
    Perrysburg: OH,
    Dayton: OH,
    Shipshewana: IN,
    South_Bend: IN,
    Fort_Wayne: IN,
    Muncie: IN,
    Columbus: IN,
    Vincennes: IN,
    Terre_Haute: IN,
    Arcola: IL,
}
