use crate::main_city::MainCity;
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
    pub const fn cities(&self) -> &[MainCity] {
        match self {
            Self::North_East => &[
                MainCity::Albany_NY,
                MainCity::Baltimore_MD,
                MainCity::Boston_MA,
                MainCity::Buffalo_NY,
                MainCity::New_York_NY,
                MainCity::Pittsburgh_PA,
                MainCity::Philadelphia_PA,
                MainCity::Portland_ME,
                MainCity::Washington_DC,
            ],
            Self::North_Central => &[
                MainCity::Chicago_IL,
                MainCity::Cincinnati_OH,
                MainCity::Cleveland_OH,
                MainCity::Columbus_OH,
                MainCity::Detroit_MI,
                MainCity::Indianapolis_IN,
                MainCity::Milwaukee_WI,
                MainCity::St_Louis_MO,
            ],
            Self::South_East => &[
                MainCity::Charlotte_TN,
                MainCity::Norfolk_VA,
                MainCity::Chattanooga_TN,
                MainCity::Atlanta_GA,
                MainCity::Charleston_SC,
                MainCity::Miami_FL,
                MainCity::Jacksonville_FL,
                MainCity::Richmond_VA,
                MainCity::Knoxville_TN,
                MainCity::Tampa_FL,
                MainCity::Mobile_AL,
            ],
            Self::South_Central => &[
                MainCity::Birmingham_AL,
                MainCity::Dallas_TX,
                MainCity::Fort_Worth_TX,
                MainCity::Houston_TX,
                MainCity::Little_Rock_AK,
                MainCity::Louisville_KY,
                MainCity::Memphis_TN,
                MainCity::Nashville_TN,
                MainCity::New_Orleans_LA,
                MainCity::San_Antonio_TX,
                MainCity::Shreveport_LA,
            ],
            Self::Plains => &[
                MainCity::Kansas_City_MO,
                MainCity::Oklahoma_City_OK,
                MainCity::Denver_CO,
                MainCity::Minneapolis_MN,
                MainCity::Des_Moines_IA,
                MainCity::Omaha_NE,
                MainCity::Pueblo_CO,
                MainCity::Fargo_ND,
            ],
            Self::North_West => &[
                MainCity::Spokane_WA,
                MainCity::Salt_Lake_City_UT,
                MainCity::Seattle_WA,
                MainCity::Portland_OR,
                MainCity::Rapid_City_SD,
                MainCity::Casper_WY,
                MainCity::Pocatello_ID,
                MainCity::Billings_MT,
                MainCity::Butte_MT,
            ],
            Self::South_West => &[
                MainCity::San_Diego_CA,
                MainCity::Los_Angeles_CA,
                MainCity::Reno_NV,
                MainCity::Sacramento_CA,
                MainCity::Las_Vegas_NV,
                MainCity::Phoenix_AZ,
                MainCity::El_Paso_TX,
                MainCity::San_Francisco_CA,
                MainCity::Tucumcari_NM,
            ],
        }
    }
}
