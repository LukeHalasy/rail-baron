pub use crate::main_city::MainCity;
pub use crate::sub_city::SubCity;
use serde::{Deserialize, Serialize};

pub use crate::Cash;

use strum::EnumIter;

use std::collections::HashMap;
// // Should replace the railroad part in subcities with the below syntax.. that way I can have all railroads
// // cleary documented and don't need to replicate paths twice (for start -> destination and destination -> start)
// // also it's clear as I am making progress
// // also I can have a test that ensures all *city* are reachable

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum C {
    D(MainCity), // Destination
    P(SubCity),  // Path
}

impl C {
    pub fn coordinates(&self) -> geoutils::Location {
        match self {
            C::D(city) => city.coordinates(),
            C::P(city) => city.coordinates(),
        }
    }
}

macro_rules! rails {
    ($(($abbrev:tt, $full_name:literal, $cost:literal)),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, EnumIter, Serialize, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum Rail { $($abbrev),* }
            impl Rail {
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

            impl std::fmt::Display for Rail {
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
rails! {
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

macro_rules! graph_out_rails {
    // Will need to change this so that you can specify either a city or subcity in both parts
    ($($c1:expr, $c2:expr, $rr:tt);*$(;)?) => {
        paste::paste! {
            impl Rail {
                pub fn get_railroad_graph() -> HashMap<C, Vec<(C, Rail)>> {
                    let mut graph: HashMap<C, Vec<(C, Rail)>> = HashMap::new();

                    $(
                    if let Some(rf) = graph.get_mut(&$c1) {
                        rf.push(($c2, Rail::$rr))
                    } else {
                        graph.insert($c1, vec![($c2, Rail::$rr)]);
                    }

                    if let Some(rf) = graph.get_mut(&$c2) {
                        rf.push(($c1, Rail::$rr))
                    } else {
                        graph.insert($c2, vec![($c1, Rail::$rr)]);
                    }
                    )*

                    graph
                }

                // Create a method here that is just for rendering
                pub fn get_edges() -> HashMap<(C, C), Vec<Rail>> {
                    let mut graph: HashMap<(C, C), Vec<Rail>> = HashMap::new();

                    $(
                    if let Some(edge) = graph.get_mut(&($c1, $c2)) {
                        edge.push(Rail::$rr);
                    } else {
                        graph.insert(($c1, $c2), vec![Rail::$rr]);
                    }
                    )*

                    graph
                }
            }
        }
    };
}

graph_out_rails! {
    // B_AND_M
    C::P(SubCity::Springfield_MA), C::D(MainCity::Albany_NY), B_AND_M ;
    C::P(SubCity::Springfield_MA), C::D(MainCity::Boston_MA), B_AND_M ;
    C::P(SubCity::Springfield_MA), C::P(SubCity::Concord_NH), B_AND_M ;
    C::P(SubCity::Concord_NH), C::D(MainCity::Portland_ME), B_AND_M;

    // NYNH_AND_H
    C::P(SubCity::Providence_RI), C::D(MainCity::Boston_MA), NYNH_AND_H;
    C::P(SubCity::Providence_RI), C::P(SubCity::New_Haven_CT), NYNH_AND_H;
    C::P(SubCity::New_Haven_CT), C::D(MainCity::New_York_NY), NYNH_AND_H;

    // NYC
    C::P(SubCity::Kingston_NY), C::D(MainCity::New_York_NY), NYC;
    C::P(SubCity::Kingston_NY), C::D(MainCity::Albany_NY), NYC;
    C::P(SubCity::Syracuse_NY), C::D(MainCity::Albany_NY), NYC;
    C::P(SubCity::Syracuse_NY), C::P(SubCity::Rochester_NY), NYC;
    C::P(SubCity::Rochester_NY), C::D(MainCity::Buffalo_NY), NYC;
    C::P(SubCity::Erie_PA), C::D(MainCity::Buffalo_NY), NYC;
    C::P(SubCity::Erie_PA), C::D(MainCity::Cleveland_OH), NYC;
    C::P(SubCity::Perrysburg_OH), C::D(MainCity::Cleveland_OH), NYC;
    C::P(SubCity::Perrysburg_OH), C::D(MainCity::Detroit_MI), NYC;
    C::P(SubCity::Perrysburg_OH), C::P(SubCity::Shipshewana_IN), NYC;
    C::P(SubCity::Shipshewana_IN), C::P(SubCity::South_Bend_IN), NYC;
    C::P(SubCity::South_Bend_IN), C::D(MainCity::Chicago_IL), NYC;
    C::P(SubCity::Perrysburg_OH), C::P(SubCity::Fort_Wayne_IN), NYC;
    C::P(SubCity::Fort_Wayne_IN), C::P(SubCity::Dayton_OH), NYC;
    C::P(SubCity::Dayton_OH), C::D(MainCity::Cincinnati_OH), NYC;
    C::P(SubCity::Fort_Wayne_IN), C::P(SubCity::Muncie_IN), NYC;
    C::P(SubCity::Muncie_IN), C::D(MainCity::Indianapolis_IN), NYC;
    C::P(SubCity::Terre_Haute_IN), C::D(MainCity::Indianapolis_IN), NYC;
    C::P(SubCity::Terre_Haute_IN), C::P(SubCity::Arcola_IL), NYC;
    C::P(SubCity::Arcola_IL), C::D(MainCity::St_Louis_MO), NYC;

    // PA
    C::P(SubCity::Trenton_NJ), C::D(MainCity::New_York_NY), PA;
    C::P(SubCity::Trenton_NJ), C::D(MainCity::Philadelphia_PA), PA;
    C::P(SubCity::Pottstown_PA), C::D(MainCity::Philadelphia_PA), PA;
    C::P(SubCity::Pottstown_PA), C::D(MainCity::Baltimore_MD), PA;
    C::P(SubCity::Pottstown_PA), C::P(SubCity::Lancaster_PA), PA;
    C::P(SubCity::Bedford_PA), C::P(SubCity::Lancaster_PA), PA;
    C::P(SubCity::Bedford_PA), C::D(MainCity::Pittsburgh_PA), PA;
    C::P(SubCity::Youngstown_OH), C::D(MainCity::Pittsburgh_PA), PA;
    C::P(SubCity::New_Philadelphia_OH), C::D(MainCity::Pittsburgh_PA), PA;
    C::P(SubCity::New_Philadelphia_OH), C::D(MainCity::Columbus_OH), PA;
    C::P(SubCity::Youngstown_OH), C::P(SubCity::Akron_OH), PA;
    C::P(SubCity::Youngstown_OH), C::P(SubCity::Erie_PA), PA;
    C::P(SubCity::Erie_PA), C::D(MainCity::Buffalo_NY), PA;
    C::P(SubCity::Akron_OH), C::D(MainCity::Cleveland_OH), PA;
    C::P(SubCity::Akron_OH), C::D(MainCity::Columbus_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(MainCity::Columbus_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(MainCity::Cincinnati_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(MainCity::Indianapolis_IN), PA;
    C::P(SubCity::Columbus_IN), C::D(MainCity::Indianapolis_IN), PA;
    C::P(SubCity::Columbus_IN), C::D(MainCity::Louisville_KY), PA;
    C::P(SubCity::Terre_Haute_IN), C::D(MainCity::Indianapolis_IN), PA;
    C::P(SubCity::West_Lafayette_IN), C::D(MainCity::Indianapolis_IN), PA;
    C::P(SubCity::West_Lafayette_IN), C::D(MainCity::Chicago_IL), PA;
    C::P(SubCity::Terre_Haute_IN), C::P(SubCity::Effingham_IL), PA;
    C::P(SubCity::Effingham_IL), C::D(MainCity::St_Louis_MO), PA;
    C::D(MainCity::Baltimore_MD), C::D(MainCity::Philadelphia_PA), PA;

    // RF_AND_P
    C::D(MainCity::Baltimore_MD), C::D(MainCity::Richmond_VA), RF_AND_P;

    // B_AND_O
    C::D(MainCity::Baltimore_MD), C::D(MainCity::Washington_DC), B_AND_O;
    C::D(MainCity::Baltimore_MD), C::P(SubCity::Frederick_MD), B_AND_O;
    C::D(MainCity::Washington_DC), C::P(SubCity::Frederick_MD), B_AND_O;
    C::P(SubCity::Cumberland_MD), C::P(SubCity::Frederick_MD), B_AND_O;
    C::P(SubCity::Cumberland_MD), C::P(SubCity::Uniontown_PA), B_AND_O;
    C::D(MainCity::Pittsburgh_PA), C::P(SubCity::Uniontown_PA), B_AND_O;
    C::D(MainCity::Pittsburgh_PA), C::P(SubCity::Youngstown_OH), B_AND_O;
    C::P(SubCity::Akron_OH), C::P(SubCity::Youngstown_OH), B_AND_O;
    C::P(SubCity::Akron_OH), C::P(SubCity::Fremont_OH), B_AND_O;
    C::P(SubCity::Ligonier_IN), C::P(SubCity::Fremont_OH), B_AND_O;
    C::P(SubCity::Ligonier_IN), C::P(SubCity::Argos_IN), B_AND_O;
    C::D(MainCity::Chicago_IL), C::P(SubCity::Argos_IN), B_AND_O;
    // C::P(SubCity::Cumberland_MD), C::P(SubCity::Brideport_WV), B_AND_O;
    C::P(SubCity::Clarksburg_WV), C::P(SubCity::Parkersburg_WV), B_AND_O;
    C::P(SubCity::Chillicothe_OH), C::P(SubCity::Parkersburg_WV), B_AND_O;
    C::P(SubCity::Chillicothe_OH), C::D(MainCity::Cincinnati_OH), B_AND_O;
    C::P(SubCity::Columbus_IN), C::D(MainCity::Cincinnati_OH), B_AND_O;
    C::P(SubCity::Columbus_IN), C::P(SubCity::Vincennes_IN), B_AND_O;
    C::P(SubCity::Centralia_IL), C::P(SubCity::Vincennes_IN), B_AND_O;
    C::P(SubCity::Centralia_IL), C::D(MainCity::St_Louis_MO), B_AND_O;

    // C_AND_O
    C::D(MainCity::Buffalo_NY), C::P(SubCity::Brantford_ON), C_AND_O;
    C::P(SubCity::London_ON), C::P(SubCity::Brantford_ON), C_AND_O;
    C::D(MainCity::Detroit_MI), C::P(SubCity::London_ON), C_AND_O;
    C::D(MainCity::Detroit_MI), C::P(SubCity::Perrysburg_OH), C_AND_O;

}

lazy_static::lazy_static! {
    pub static ref RAILROAD_GRAPH: HashMap<C, Vec<(C, Rail)>> = Rail::get_railroad_graph();
}
