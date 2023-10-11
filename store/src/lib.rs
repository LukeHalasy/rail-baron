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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    Yellow,
    Green,
    Blue,
    White,
    Black,
    Red,
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub cash: u64,
    pub name: String,
    pub piece: Piece,
    pub home_city: City,
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
