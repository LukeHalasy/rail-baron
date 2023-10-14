use crate::deed::Deed;

pub use crate::city::City;

use serde::{Deserialize, Serialize};

pub enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

// $($subkey:literal: $subcmd:tt),+$(,)?
macro_rules! sub_cities {
    ($(($c:tt, $s:tt, [$($nc:tt: [$($d:tt),+$(,)?]),+$(,)?], [$($nsc:tt: [$($sd:tt),+$(,)?]),+$(,)?])),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
            #[allow(non_camel_case_types)]
            // Can I make this titled $city_$state ?
            pub enum SubCity { $($c),* }
            impl SubCity {
                pub const fn sub_cities() -> &'static [Self] {
                    &[$(Self::$c),*]
                }

                pub const fn state(&self) -> State {
                    match self {
                        $(Self::$c => State::$s),*
                    }
                }

                pub const fn neighbor_cities(&self) -> &[(City, &[Deed])] {
                    match self {
                        $(Self::$c => {
                            &[$((
                                City::$nc,
                                &[$(Deed::$d),*],
                            )),*]
                        }),*
                    }
                }

                pub const fn neighbor_sub_cities(&self) -> &[(Self, &[Deed])] {
                    match self {
                        $(Self::$c => {
                            &[$((
                                Self::$nsc,
                                &[$(Deed::$sd),*],
                            )),*]
                        }),*
                    }
                }

                // Also need a method for getting the name of the abbred with _AND_ replaced with &
            }
        }
    }
}

// would be nice to make this verbose
sub_cities! {
    (Concord, NewHampshire, [Boston: [B_AND_M], Portland_Maine: [B_AND_M]], [Springfield: [B_AND_M]]),
    (Springfield, Massachusetts, [Albany: [B_AND_M], Boston: [B_AND_M]], [Concord: [B_AND_M]]),
}

// What does each city need ??
// Create a macro that makes an enum
//
// pub enum SubCity {
//   Each_City_Name
// }
//
// And then have a method to get the surrounding Cities and SubCities in a Vec (Option ??)
// That method should also return the rail-roads associated with each connected city / subcity
// pub enum CityTypes {
//   Main(City)
//   Side(City)
// }
// Vec<(CityTypes, Vec<Deed>)>
//
// I'll need to replicate this in city.rs in case the user is on a main city..
// perhaps I should rename that file to main_city.rs and make a file city.rs with the above enum
//
// What happens in the case of duplicate names ?
//
//
//
//
//
//
//
//
// * State (for finding coordinates and mapping)
// A list of sub-cities and cities that connect to it. (And the rail-roads for each situation)

//   --------------------------------------------------------------
//   ----------------------------------y---------------------------
//   --------------------------------------------------------------
//   ---------------------------x----------------------------------
//   ------------------a-------------------------------------------
//   -----------------------------------b--------------------------
//   --------------------------------------------------------------

// In the examble above let's say their are two railroads that both get from a to x.
// One of those railroads, 1, goes from a->x->b, while 2 goes a->x->y

// How do I represent this in data ?
//

// Note

// // North-West

// // Rail-Road NYC:

// // From left to right

// // NYC Railroad
// // More to the left into the MidWest
// Erie -> ((City::Buffalo)) -> Rochester -> Syracuse -> ((City::Albany)) -> Kingston -> ((City::NewYork))

// // B & M
// ((City::Albany)) -> Springfield ->both Concord, Boston ->concord ((City::Portland))

// // NYNH&H
// ((City::NewYork)) -> New Haven -> Providence -> ((City::Boston))

// Concord
// Providence
// New Haven
// Springfield
// Kingston
// Trenton
// Rochester
// Frederick
// Lancaster
// Bedford
// Pottstown
// Cumberland
// Union Town
