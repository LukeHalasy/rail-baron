use crate::rail_road::{City, SubCity};
pub use crate::Cash;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! deeds {
    ($(($abbrev:tt, $full_name:literal, $cost:literal)),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum Deed { $($abbrev),* }
            impl Deed {
                pub const fn rails() -> &'static [Self] {
                    &[$(Self::$abbrev),*]
                }

                pub const fn cost(&self) -> Cash {
                    match self {
                        $(Self::$abbrev => $cost),*
                    }
                }

                pub const fn full_name(&self) -> &str {
                    match self {
                        $(Self::$abbrev => $full_name),*
                    }
                }
            }

            impl std::fmt::Display for Deed {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        $(Self::$abbrev => write!(f, "{}", stringify!($abbrev))),*
                    }
                }
            }
        }
    }
}

// NOTE: TEMPORARY: LM = Label's made for every city (and subcity) on the route
deeds! {
    (ACL,	            "Atlantic Coast Line",	                    12000),
    (AT_AND_SF,	        "Atchison, Topeka, & Santa Fe",	            40000), // LM
    (B_AND_M,	        "Boston & Maine",	                        4000), // LM
    (B_AND_O,	        "Baltimore & Ohio",	                        24000), // LM
    (CB_AND_Q,	        "Chicago, Burlington & Quincy",	            20000),
    (CMSTP_AND_P,	    "Chicago, Milwaukee, St. Paul, & Pacific",	18000),
    (C_AND_NW,	        "Chicago & NorthWestern",	                14000),
    (C_AND_O,	        "Chesapeake & Ohio",	                    20000),
    (CRI_AND_P,	        "Chicago, Rock Island, & Pacific",	        29000),
    (D_AND_RGW,	        "Denver & Rio Grande Western",	            6000), // LM
    (GM_AND_O,	        "Gulf, Mobile & Ohio",	                    12000), // LM
    (GN,	            "Great Northern",	                        17000), // LM
    (IC,	            "Illinois Central",	                        14000),
    (MP,	            "Missouri Pacific",	                        21000),
    (L_AND_N,	        "Louisville & Nashville",	                18000),
    (NP,	            "Northern Pacific",	                        14000), // LM
    (N_AND_W,	        "Norfolk & Western",	                    12000),
    (NYC,	            "New York Central",	                        28000), // LM
    (NYNH_AND_H,	    "New York, New Haven & Hartford",	        4000), // LM
    (PA,	            "Pennsylvania",	                            30000), // LM
    (RF_AND_P,	        "Richmond, Fredericksburg, & Potomac",	    4000), // LM
    (SAL,	            "Seaboard Air Line",	                    14000),
    (SOU,	            "Southern",	                                20000),
    (SP,	            "Southern Pacific",	                        42000), // LM
    (SLSF,	            "St. Louis-San Francisco",	                19000),
    (T_AND_P,	        "Texas & Pacific",	                        10000), // LM
    (UP,	            "Union Pacific",	                        40000),
    (WP,	            "Western Pacific",	                        8000), // LM
}
