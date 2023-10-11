use serde::{Deserialize, Serialize};

type PlayerId = u64;
pub type Cash = u64;

pub mod payout;

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

// macro_rules! cities {
//     ($($region:tt => [$($city:tt),+$(,)?]),*$(,)?) => {
//         paste::paste! {
//             #[derive(Clone, Copy, Debug)]
//             pub enum Region { $($region),* }
//             impl Region {
//                 pub const fn regions() -> &'static [Self] {
//                     &[$(Self::$region),*]
//                 }
//                 pub const fn cities(&self) -> &[City] {
//                     match self {
//                         $(Self::$region => {
//                             &[$(
//                                 City::$city
//                             ),*]
//                         }),*
//                     }
//                 }
//             }

//             #[derive(Clone, Copy, Debug)]
//             pub enum City {
//                 $($city,),*
//             }
//         }
//     }
// }

#[derive(Clone, Copy, Debug)]
pub enum Region {
    NorthEast,
    NorthCentral,
    SouthEast,
    SouthCentral,
    Plains,
    NorthWest,
    SouthWest,
}

pub enum City {
    Albany,
    Atlanta,
    Baltimore,
    Billings,
    Birmingham,
    Boston,
    Buffalo,
    Butte,
    Casper,
    Charleston,
    Charlotte,
    Chattanooga,
    Chicago,
    Cincinnati,
    Cleveland,
    Columbus,
    Dallas,
    Denver,
    DesMoines,
    Detroit,
    ElPaso,
    Fargo,
    FortWorth,
    Houston,
    Indianapolis,
    Jacksonville,
    KansasCity,
    Knoxville,
    LasVegas,
    LittleRock,
    LosAngeles,
    Louisville,
    Memphis,
    Miami,
    Milwaukee,
    Minneapolis,
    Mobile,
    Nashville,
    NewOrleans,
    NewYork,
    Norfolk,
    OklahomaCity,
    Omaha,
    Philadelphia,
    Phoenix,
    Pittsburgh,
    Pocatello,
    PortlandMaine,
    PortlandOregon,
    Pueblo,
    RapidCity,
    Reno,
    Richmond,
    Sacramento,
    SaltLakeCity,
    SanAntonio,
    SanDiego,
    SanFrancisco,
    Seattle,
    Shreveport,
    Spokane,
    StLouis,
    Tampa,
    Tucumcari,
    Washington,
}

impl Region {
    pub const fn cities(&self) -> &[City] {
        match self {
            Self::NorthEast => &[
                City::Albany,
                City::Baltimore,
                City::Boston,
                City::Buffalo,
                City::NewYork,
                City::Pittsburgh,
                City::Philadelphia,
                City::PortlandMaine,
                City::Washington,
            ],
            Self::NorthCentral => &[
                City::Chicago,
                City::Cincinnati,
                City::Cleveland,
                City::Columbus,
                City::Detroit,
                City::Indianapolis,
                City::Milwaukee,
                City::StLouis,
            ],
            Self::SouthEast => &[
                City::Charlotte,
                City::Norfolk,
                City::Chattanooga,
                City::Atlanta,
                City::Charleston,
                City::Miami,
                City::Jacksonville,
                City::Richmond,
                City::Knoxville,
                City::Tampa,
                City::Mobile,
            ],
            Self::SouthCentral => &[
                City::Birmingham,
                City::Dallas,
                City::FortWorth,
                City::Houston,
                City::LittleRock,
                City::Louisville,
                City::Memphis,
                City::Nashville,
                City::NewOrleans,
                City::SanAntonio,
                City::Shreveport,
            ],
            Self::Plains => &[
                City::KansasCity,
                City::OklahomaCity,
                City::Denver,
                City::Minneapolis,
                City::DesMoines,
                City::Omaha,
                City::Pueblo,
                City::Fargo,
            ],
            Self::NorthWest => &[
                City::Spokane,
                City::SaltLakeCity,
                City::Seattle,
                City::PortlandOregon,
                City::RapidCity,
                City::Casper,
                City::Pocatello,
                City::Billings,
                City::Butte,
            ],
            Self::SouthWest => &[
                City::SanDiego,
                City::LosAngeles,
                City::Reno,
                City::Sacramento,
                City::LasVegas,
                City::Phoenix,
                City::ElPaso,
                City::SanFrancisco,
                City::Tucumcari,
            ],
        }
    }
}

// impl City {
// pub fn payout(city_pair: (City, City)) -> Cash {
//     match city_pair {
//         (City::Albany, City::Albany) => {
//             // Handle the combination Albany-Albany
//             todo!();
//         }
//         (City::Albany, City::Atlanta) | (City::Atlanta, City::Albany) => {
//             // Handle the combination Albany-Atlanta or Atlanta-Albany
//             todo!();
//         }
//         (City::Albany, City::Baltimore) | (City::Baltimore, City::Albany) => {
//             // Handle the combination Albany-Baltimore or Baltimore-Albany
//             todo!();
//         }
//         (City::Albany, City::Billings) | (City::Billings, City::Albany) => {
//             // Handle the combination Albany-Billings or Billings-Albany
//             todo!();
//         }
//         (City::Albany, City::Birmingham) | (City::Birmingham, City::Albany) => {
//             // Handle the combination Albany-Birmingham or Birmingham-Albany
//             todo!();
//         }
//         (City::Albany, City::Boston) | (City::Boston, City::Albany) => {
//             // Handle the combination Albany-Boston or Boston-Albany
//             todo!();
//         }
//         (City::Albany, City::Buffalo) | (City::Buffalo, City::Albany) => {
//             // Handle the combination Albany-Buffalo or Buffalo-Albany
//             todo!();
//         }
//         (City::Albany, City::Butte) | (City::Butte, City::Albany) => {
//             // Handle the combination Albany-Butte or Butte-Albany
//             todo!();
//         }
//         (City::Albany, City::Casper) | (City::Casper, City::Albany) => {
//             // Handle the combination Albany-Casper or Casper-Albany
//             todo!();
//         }
//         (City::Albany, City::Charleston) | (City::Charleston, City::Albany) => {
//             // Handle the combination Albany-Charleston or Charleston-Albany
//             todo!();
//         }
//         (City::Albany, City::Charlotte) | (City::Charlotte, City::Albany) => {
//             // Handle the combination Albany-Charlotte or Charlotte-Albany
//             todo!();
//         }
//         (City::Albany, City::Chattanooga) | (City::Chattanooga, City::Albany) => {
//             // Handle the combination Albany-Chattanooga or Chattanooga-Albany
//             todo!();
//         }
//         (City::Albany, City::Chicago) | (City::Chicago, City::Albany) => {
//             // Handle the combination Albany-Chicago or Chicago-Albany
//             todo!();
//         }
//         (City::Albany, City::Cincinnati) | (City::Cincinnati, City::Albany) => {
//             // Handle the combination Albany-Cincinnati or Cincinnati-Albany
//             todo!();
//         }
//         (City::Albany, City::Cleveland) | (City::Cleveland, City::Albany) => {
//             // Handle the combination Albany-Cleveland or Cleveland-Albany
//             todo!();
//         }
//         (City::Albany, City::Columbus) | (City::Columbus, City::Albany) => {
//             // Handle the combination Albany-Columbus or Columbus-Albany
//             todo!();
//         }
//         (City::Albany, City::Dallas) | (City::Dallas, City::Albany) => {
//             // Handle the combination Albany-Dallas or Dallas-Albany
//             todo!();
//         }
//         (City::Albany, City::Denver) | (City::Denver, City::Albany) => {
//             // Handle the combination Albany-Denver or Denver-Albany
//             todo!();
//         }
//         (City::Albany, City::DesMoines) | (City::DesMoines, City::Albany) => {
//             // Handle the combination Albany-DesMoines or DesMoines-Albany
//             todo!();
//         }
//         (City::Albany, City::Detroit) | (City::Detroit, City::Albany) => {
//             // Handle the combination Albany-Detroit or Detroit-Albany
//             todo!();
//         }
//         (City::Albany, City::ElPaso) | (City::ElPaso, City::Albany) => {
//             // Handle the combination Albany-ElPaso or ElPaso-Albany
//             todo!();
//         }
//         (City::Albany, City::Fargo) | (City::Fargo, City::Albany) => {
//             // Handle the combination Albany-Fargo or Fargo-Albany
//             todo!();
//         }
//         (City::Albany, City::FortWorth) | (City::FortWorth, City::Albany) => {
//             // Handle the combination Albany-FortWorth or FortWorth-Albany
//             todo!();
//         }
//         (City::Albany, City::Houston) | (City::Houston, City::Albany) => {
//             // Handle the combination Albany-Houston or Houston-Albany
//             todo!();
//         }
//         (City::Albany, City::Indianapolis) | (City::Indianapolis, City::Albany) => {
//             // Handle the combination Albany-Indianapolis or Indianapolis-Albany
//             todo!();
//         }
//         (City::Albany, City::Jacksonville) | (City::Jacksonville, City::Albany) => {
//             // Handle the combination Albany-Jacksonville or Jacksonville-Albany
//             todo!();
//         }
//         (City::Albany, City::KansasCity) | (City::KansasCity, City::Albany) => {
//             // Handle the combination Albany-KansasCity or KansasCity-Albany
//             todo!();
//         }
//         (City::Albany, City::Knoxville) | (City::Knoxville, City::Albany) => {
//             // Handle the combination Albany-Knoxville or Knoxville-Albany
//             todo!();
//         }
//         (City::Albany, City::LasVegas) | (City::LasVegas, City::Albany) => {
//             // Handle the combination Albany-LasVegas or LasVegas-Albany
//             todo!();
//         }
//         (City::Albany, City::LittleRock) | (City::LittleRock, City::Albany) => {
//             // Handle the combination Albany-LittleRock or LittleRock-Albany
//             todo!();
//         }
//         (City::Albany, City::LosAngeles) | (City::LosAngeles, City::Albany) => {
//             // Handle the combination Albany-LosAngeles or LosAngeles-Albany
//             todo!();
//         }
//         (City::Albany, City::Louisville) | (City::Louisville, City::Albany) => {
//             // Handle the combination Albany-Louisville or Louisville-Albany
//             todo!();
//         }
//         (City::Albany, City::Memphis) | (City::Memphis, City::Albany) => {
//             // Handle the combination Albany-Memphis or Memphis-Albany
//             todo!();
//         }
//         (City::Albany, City::Miami) | (City::Miami, City::Albany) => {
//             // Handle the combination Albany-Miami or Miami-Albany
//             todo!();
//         }
//         (City::Albany, City::Milwaukee) | (City::Milwaukee, City::Albany) => {
//             // Handle the combination Albany-Milwaukee or Milwaukee-Albany
//             todo!();
//         }
//         (City::Albany, City::Minneapolis) | (City::Minneapolis, City::Albany) => {
//             // Handle the combination Albany-Minneapolis or Minneapolis-Albany
//             todo!();
//         }
//         (City::Albany, City::Mobile) | (City::Mobile, City::Albany) => {
//             // Handle the combination Albany-Mobile or Mobile-Albany
//             todo!();
//         }
//         (City::Albany, City::Nashville) | (City::Nashville, City::Albany) => {
//             // Handle the combination Albany-Nashville or Nashville-Albany
//             todo!();
//         }
//         (City::Albany, City::NewOrleans) | (City::NewOrleans, City::Albany) => {
//             // Handle the combination Albany-NewOrleans or NewOrleans-Albany
//             todo!();
//         }
//         (City::Albany, City::NewYork) | (City::NewYork, City::Albany) => {
//             // Handle the combination Albany-NewYork or NewYork-Albany
//             todo!();
//         }
//         (City::Albany, City::Norfolk) | (City::Norfolk, City::Albany) => {
//             // Handle the combination Albany-Norfolk or Norfolk-Albany
//             todo!();
//         }
//         (City::Albany, City::Oakland) | (City::Oakland, City::Albany) => {
//             // Handle the combination Albany-Oakland or Oakland-Albany
//             todo!();
//         }
//         (City::Albany, City::OklahomaCity) | (City::OklahomaCity, City::Albany) => {
//             // Handle the combination Albany-OklahomaCity or OklahomaCity-Albany
//             todo!();
//         }
//         (City::Albany, City::Omaha) | (City::Omaha, City::Albany) => {
//             // Handle the combination Albany-Omaha or Omaha-Albany
//             todo!();
//         }
//         (City::Albany, City::Philadelphia) | (City::Philadelphia, City::Albany) => {
//             // Handle the combination Albany-Philadelphia or Philadelphia-Albany
//             todo!();
//         }
//         (City::Albany, City::Phoenix) | (City::Phoenix, City::Albany) => {
//             // Handle the combination Albany-Phoenix or Phoenix-Albany
//             todo!();
//         }
//         (City::Albany, City::Pittsburgh) | (City::Pittsburgh, City::Albany) => {
//             // Handle the combination Albany-Pittsburgh or Pittsburgh-Albany
//             todo!();
//         }
//         (City::Albany, City::Pocatello) | (City::Pocatello, City::Albany) => {
//             // Handle the combination Albany-Pocatello or Pocatello-Albany
//             todo!();
//         }
//         (City::Albany, City::PortlandMaine) | (City::PortlandMaine, City::Albany) => {
//             // Handle the combination Albany-PortlandMaine or PortlandMaine-Albany
//             todo!();
//         }
//         (City::Albany, City::PortlandOregon) | (City::PortlandOregon, City::Albany) => {
//             // Handle the combination Albany-PortlandOregon or PortlandOregon-Albany
//             todo!();
//         }
//         (City::Albany, City::Pueblo) | (City::Pueblo, City::Albany) => {
//             // Handle the combination Albany-Pueblo or Pueblo-Albany
//             todo!();
//         }
//         (City::Albany, City::RapidCity) | (City::RapidCity, City::Albany) => {
//             // Handle the combination Albany-RapidCity or RapidCity-Albany
//             todo!();
//         }
//         (City::Albany, City::Reno) | (City::Reno, City::Albany) => {
//             // Handle the combination Albany-Reno or Reno-Albany
//             todo!();
//         }
//         (City::Albany, City::Richmond) | (City::Richmond, City::Albany) => {
//             // Handle the combination Albany-Richmond or Richmond-Albany
//             todo!();
//         }
//         (City::Albany, City::Sacramento) | (City::Sacramento, City::Albany) => {
//             // Handle the combination Albany-Sacramento or Sacramento-Albany
//             todo!();
//         }
//         (City::Albany, City::SaltLakeCity) | (City::SaltLakeCity, City::Albany) => {
//             // Handle the combination Albany-SaltLakeCity or SaltLakeCity-Albany
//             todo!();
//         }
//         (City::Albany, City::SanAntonio) | (City::SanAntonio, City::Albany) => {
//             // Handle the combination Albany-SanAntonio or SanAntonio-Albany
//             todo!();
//         }
//         (City::Albany, City::SanDiego) | (City::SanDiego, City::Albany) => {
//             // Handle the combination Albany-SanDiego or SanDiego-Albany
//             todo!();
//         }
//         (City::Albany, City::SanFrancisco) | (City::SanFrancisco, City::Albany) => {
//             // Handle the combination Albany-SanFrancisco or SanFrancisco-Albany
//             todo!();
//         }
//         (City::Albany, City::Seattle) | (City::Seattle, City::Albany) => {
//             // Handle the combination Albany-Seattle or Seattle-Albany
//             todo!();
//         }
//         (City::Albany, City::Shreveport) | (City::Shreveport, City::Albany) => {
//             // Handle the combination Albany-Shreveport or Shreveport-Albany
//             todo!();
//         }
//         (City::Albany, City::Spokane) | (City::Spokane, City::Albany) => {
//             // Handle the combination Albany-Spokane or Spokane-Albany
//             todo!();
//         }
//         (City::Albany, City::StLouis) | (City::StLouis, City::Albany) => {
//             // Handle the combination Albany-StLouis or StLouis-Albany
//             todo!();
//         }
//         (City::Albany, City::StPaul) | (City::StPaul, City::Albany) => {
//             // Handle the combination Albany-StPaul or StPaul-Albany
//             todo!();
//         }
//         (City::Albany, City::Tampa) | (City::Tampa, City::Albany) => {
//             // Handle the combination Albany-Tampa or Tampa-Albany
//             todo!();
//         }
//         (City::Albany, City::Tucumcari) | (City::Tucumcari, City::Albany) => {
//             // Handle the combination Albany-Tucumcari or Tucumcari-Albany
//             todo!();
//         }
//         (City::Albany, City::Washington) | (City::Washington, City::Albany) => {
//             // Handle the combination Albany-Washington or Washington-Albany
//             todo!();
//         }
//     }
// }
// }

// NOTE: This macro will need to do two things.. create a Regions enum and a cities
// cities! {
//     NorthEast => [
//         Albany, Baltimore, Boston, Buffalo, NewYork, Pittsburgh, Philadalphia, PortlandMaine, Washington
//     ],
//     NorthCentral => [
//         Chicago, Cincinnati, Cleveland, Columbus, Detroit, Indianapolis, Milwaukee, StLouis
//     ],
//     SouthEast => [
//         Charlotte, Norfolk, Chattanooga, Atlanta, Charleston, Miami, Jacksonville, Richmond, Knoxville, Tampa, Mobile
//     ],
//     SouthCentral => [
//         Birmingham, Dallas, FortWorth, Houston, LittleRock, Louisville, Memphis, Nashville, NewOrleans, SanAntonio, Shreveport
//     ],
//     Plains => [
//         KansasCity, OklahomaCity, StPaul, Denver, Minneapolis, DesMoines, Omaha, Pueblo, Fargo
//     ],
//     NorthWest => [
//         Spokane, SaltLakeCity, Seattle, PortlandOregon, RapidCity, Casper, Pocatello, Billings, Butte
//     ],
//     SouthWest => [
//         SanDiego, LosAngeles, Oakland, Reno, Sacramento, LasVegas, Phoenix, ElPaso, SanFrancisco, Tucumcari
//     ],
// }

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

// lazy_static! {
//     static ref PAYOFFS: HashMap<(City, City), Cash> = {
//         let mut map = HashMap::new();
//         map.insert((City::NorthEast(northeast::City::Albany), City::NorthEast(northeast::City::Baltimore), 3.5);
//         // map.insert("Key2", 100);
//         map
//     };
// }
// static PAYOFFS: HashMap<(City, City), Cash> = HashMap::new();

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
