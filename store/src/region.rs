use crate::city::City;

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
