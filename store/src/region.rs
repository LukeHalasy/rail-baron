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
                City::Albany_NY,
                City::Baltimore_MD,
                City::Boston_MA,
                City::Buffalo_NY,
                City::New_York_NY,
                City::Pittsburgh_PA,
                City::Philadelphia_PA,
                City::Portland_ME,
                City::Washington_DC,
            ],
            Self::North_Central => &[
                City::Chicago_IL,
                City::Cincinnati_OH,
                City::Cleveland_OH,
                City::Columbus_OH,
                City::Detroit_MI,
                City::Indianapolis_IN,
                City::Milwaukee_WI,
                City::St_Louis_MO,
            ],
            Self::South_East => &[
                City::Charlotte_TN,
                City::Norfolk_VA,
                City::Chattanooga_TN,
                City::Atlanta_GA,
                City::Charleston_SC,
                City::Miami_FL,
                City::Jacksonville_FL,
                City::Richmond_VA,
                City::Knoxville_TN,
                City::Tampa_FL,
                City::Mobile_AL,
            ],
            Self::South_Central => &[
                City::Birmingham_AL,
                City::Dallas_TX,
                City::Fort_Worth_TX,
                City::Houston_TX,
                City::Little_Rock_AK,
                City::Louisville_KY,
                City::Memphis_TN,
                City::Nashville_TN,
                City::New_Orleans_LA,
                City::San_Antonio_TX,
                City::Shreveport_LA,
            ],
            Self::Plains => &[
                City::Kansas_City_MO,
                City::Oklahoma_City_OK,
                City::Denver_CO,
                City::Minneapolis_MN,
                City::Des_Moines_IA,
                City::Omaha_NE,
                City::Pueblo_CO,
                City::Fargo_ND,
            ],
            Self::North_West => &[
                City::Spokane_WA,
                City::Salt_Lake_City_UT,
                City::Seattle_WA,
                City::Portland_OR,
                City::Rapid_City_SD,
                City::Casper_WY,
                City::Pocatello_ID,
                City::Billings_MT,
                City::Butte_MT,
            ],
            Self::South_West => &[
                City::San_Diego_CA,
                City::Los_Angeles_CA,
                City::Reno_NV,
                City::Sacramento_CA,
                City::Las_Vegas_NV,
                City::Phoenix_AZ,
                City::El_Paso_TX,
                City::San_Francisco_CA,
                City::Tucumcari_NM,
            ],
        }
    }

    pub const fn region_from_dice_roll(red_dice: i32, white_dice: (i32, i32)) -> Self {
        if red_dice % 2 != 0 {
            match white_dice.0 + white_dice.1 {
                2 => Self::Plains,
                3 => Self::South_East,
                4 => Self::South_East,
                5 => Self::South_East,
                6 => Self::North_Central,
                7 => Self::North_Central,
                8 => Self::North_East,
                9 => Self::North_East,
                10 => Self::North_East,
                11 => Self::North_East,
                12 => Self::North_East,
                _ => panic!("White dice sum > 12 || < 2"),
            }
        } else {
            match white_dice.0 + white_dice.1 {
                2 => Self::South_West,
                3 => Self::South_Central,
                4 => Self::South_Central,
                5 => Self::South_Central,
                6 => Self::South_West,
                7 => Self::South_West,
                8 => Self::Plains,
                9 => Self::North_West,
                10 => Self::North_West,
                11 => Self::Plains,
                12 => Self::North_West,
                _ => panic!("White dice sum > 12 || < 2"),
            }
        }
    }
}
