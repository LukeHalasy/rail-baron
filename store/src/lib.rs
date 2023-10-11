use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

type PlayerId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}

// Should I put the hex code's within server code, or have the client able to determine the respective colors
// for each piece ?
// NOTE: This is a user-choosable option, but no two players can have the same color

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    Yellow,
    Green,
    Blue,
    White,
    Black,
    Red,
}

// pub struct Deed {
//     s
// }

macro_rules! cities {
    ($($region:tt => [$($city:tt),+$(,)?]),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug)]
            pub enum Region { $($region),* }
            impl Region {
                pub const fn regions() -> &'static [Self] {
                    &[$(Self::$region),*]
                }
                pub const fn cities(&self) -> &[City] {
                    match self {
                        $(Self::$region => {
                            &[$(
                                City::$region([<$region:lower>]::City::$city)
                            ),*]
                        }),*
                    }
                }
            }

            #[derive(Clone, Copy, Debug)]
            pub enum City { $($region([<$region:lower>]::City)),* }
            impl fmt::Display for City {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match self { $(Self::$region(city) => write!(f, "{city}")),* }
                }
            }

            $(
                pub mod [<$region:lower>] {
                    use std::fmt;
                    #[derive(Debug, Clone, Copy)]
                    pub enum City { $($city),* }
                    impl fmt::Display for City {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            match self {
                                $(Self::$city => write!(f, stringify!([<$city:lower>]))),*
                            }
                        }
                    }
                }
            )*
        }
    }
}

// NOTE: This macro will need to do two things.. create a Regions enum and a cities
cities! {
    NorthEast =>    [Albany, Baltimore, Boston, Buffalo, NewYork, Pittsburgh, Philadalphia, Portland, Washington],
    NorthCentral => [Chicago, Cincinnati, Cleveland, Columbus, Detroit, Indianapolis, Milwaukee, StLouis],
    SouthEast => [Charlotte, Norfolk, Chattanooga, Atlanta, Charleston, Miami, Jacksonville, Richmond, Knoxville, Tampa, Mobile],
    SouthCentral => [Birmingham, Dallas, FortWorth, Houston, LittleRock, Louisville, Memphis, Nashville, NewOrleans, SanAntonio, Shreveport],
    Plains => [KansasCity, OklahomaCity, StPaul, Denver, Minneapolis, DesMoines, Omaha, Pueblo, Fargo],
    NorthWest => [Spokane, SaltLakeCity, Seattle, Portland, RapidCity, Casper, Pocatello, Billings, Butte],
    SouthWest => [SanDiego, LosAngeles, Oakland, Reno, Sacramento, LasVegas, Phoenix, ElPaso, SanFrancisco, Tucumcari],
}

// macro_rules! payoffs {
//     ($($city_one:path,$city_two:path => $payoff:literal),*$(,)?) => {
//         paste::paste! {

//         }
//     };
// }

// Payoff information here:
// https://www.ewert-technologies.ca/assets/downloads/railBaronTools/PayoffsChart.pdf

// Need a unit-test to ensure right number of entries
// payoffs! {
//     NorthEast, NorthEast => [
//         [],
//         [3.5],
//         [2, 4],
//         [3, 4, 5],
//         [1.5, 2, 2.5, 4],
//         [2.5, 1, 3, 4, 1],
//         [5.5, 3.5, 6.5, 2.5, 4.5, 3.5],
//         [3, 5.5, 1, 6, 3.5, 4.5, 8],
//         [3.5, 0.5, 4.5, 4.5, 2, 1.5, 3, 5.5]
//     ],
//     SouthEast, NorthEast => [
//         [10, 7, 11, 9.5, 8.5, 7.5, 8, 12, 6.5],
//         [9.5, 5.5, 9.5, 9.5, 7.5, 8, 8, 11, 5],
//         [7.5, 4, 8.5, 8, 6]
//     ]
// }

// payoffs! {
//     northeast::City::Albany, northeast::City::Albany => 0,
//     northeast::City::Albany, northeast::City::Baltimo => 0,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub cash: u64,
    pub name: String,
    pub piece: Piece,
    // pub home_city: City
    // A list of railroads
    // pub deeds: Vec<Deed>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    // pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>, // Might want a smarter data structure hre
                                     // pub history: Vec<GameEvent>,
}

// impl Default for State {
//     fn default() -> Self {
//         // Self {
//         //     stage: Stage::PreGame,
//         //     active_player_id: 0,
//         //     players: HashMap::new(),
//         // }
//     }
// }

// INFO: Not stored in state, nor is an option,
// TODO: Payoff table for all source + destination route payoffs
// INFO: Also things that are global like how much cash you must have to win
