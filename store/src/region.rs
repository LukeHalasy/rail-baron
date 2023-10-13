use crate::city::City;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum Region {
    North_East,
    North_Central,
    South_East,
    South_Central,
    Plains,
    North_West,
    South_West,
}

impl Region {
    pub const fn cities(&self) -> &[City] {
        match self {
            Self::North_East => &[
                City::Albany,
                City::Baltimore,
                City::Boston,
                City::Buffalo,
                City::New_York,
                City::Pittsburgh,
                City::Philadelphia,
                City::Portland_Maine,
                City::Washington,
            ],
            Self::North_Central => &[
                City::Chicago,
                City::Cincinnati,
                City::Cleveland,
                City::Columbus,
                City::Detroit,
                City::Indianapolis,
                City::Milwaukee,
                City::St_Louis,
            ],
            Self::South_East => &[
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
            Self::South_Central => &[
                City::Birmingham,
                City::Dallas,
                City::Fort_Worth,
                City::Houston,
                City::Little_Rock,
                City::Louisville,
                City::Memphis,
                City::Nashville,
                City::NewOrleans,
                City::San_Antonio,
                City::Shreveport,
            ],
            Self::Plains => &[
                City::Kansas_City,
                City::Oklahoma_City,
                City::Denver,
                City::Minneapolis,
                City::Des_Moines,
                City::Omaha,
                City::Pueblo,
                City::Fargo,
            ],
            Self::North_West => &[
                City::Spokane,
                City::Salt_Lake_City,
                City::Seattle,
                City::Portland_Oregon,
                City::Rapid_City,
                City::Casper,
                City::Pocatello,
                City::Billings,
                City::Butte,
            ],
            Self::South_West => &[
                City::San_Diego,
                City::LosAngeles,
                City::Reno,
                City::Sacramento,
                City::Las_Vegas,
                City::Phoenix,
                City::ElPaso,
                City::San_Francisco,
                City::Tucumcari,
            ],
        }
    }
}
