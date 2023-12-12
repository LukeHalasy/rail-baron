pub use crate::main_city::City;
pub use crate::sub_city::SubCity;
use serde::{Deserialize, Serialize};

use petgraph::graph::{NodeIndex, UnGraph};

pub use crate::Cash;

use strum::{EnumIter, IntoEnumIterator};

use std::collections::HashMap;
use std::fmt::Display;
// // Should replace the railroad part in subcities with the below syntax.. that way I can have all railroads
// // cleary documented and don't need to replicate paths twice (for start -> destination and destination -> start)
// // also it's clear as I am making progress
// // also I can have a test that ensures all *city* are reachable

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum C {
    D(City),    // Destination
    P(SubCity), // Path
}

impl C {
    pub fn coordinates(&self) -> geoutils::Location {
        match self {
            C::D(city) => city.coordinates(),
            C::P(city) => city.coordinates(),
        }
    }
}

impl std::convert::From<C> for petgraph::prelude::NodeIndex {
    fn from(c: C) -> Self {
        // iterate through C, find the index of the node
        let mut i = 0;

        // TODO: Iterate through all main cities
        for city in City::iter() {
            if let C::D(c) = c {
                if c == city {
                    return NodeIndex::new(i);
                }
            }

            i += 1;
        }

        for sub_city in SubCity::iter() {
            if let C::P(c) = c {
                if c == sub_city {
                    return NodeIndex::new(i);
                }
            }

            i += 1;
        }
        // TODO: Iterate through all path cities
        panic!("Could not find node index for {:?}", c);
    }
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            C::D(city) => write!(f, "{}", city),
            C::P(city) => write!(f, "{}", city),
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
                pub fn get_railroad_graph() -> UnGraph<C, Rail> {
                    let mut graph = UnGraph::<C, Rail>::new_undirected();

                    for city in City::iter() {
                        graph.add_node(C::D(city));
                    }

                    for sub_city in SubCity::iter() {
                        graph.add_node(C::P(sub_city));
                    }

                    $(
                        graph.add_edge($c1.into(), $c2.into(), Rail::$rr);
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

use City::*;
use SubCity::*;
use C::*;

graph_out_rails! {
    // B_AND_M
    P(Springfield_MA), D(Albany_NY), B_AND_M ;
    P(Springfield_MA), D(Boston_MA), B_AND_M ;
    P(Springfield_MA), P(Concord_NH), B_AND_M ;
    P(Concord_NH), D(Portland_ME), B_AND_M;

    // NYNH_AND_H
    P(Providence_RI), D(Boston_MA), NYNH_AND_H;
    P(Providence_RI), P(New_Haven_CT), NYNH_AND_H;
    P(New_Haven_CT), D(New_York_NY), NYNH_AND_H;

    // NYC
    P(Kingston_NY), D(New_York_NY), NYC;
    P(Kingston_NY), D(Albany_NY), NYC;
    P(Syracuse_NY), D(Albany_NY), NYC;
    P(Syracuse_NY), P(Rochester_NY), NYC;
    P(Rochester_NY), D(Buffalo_NY), NYC;
    P(Erie_PA), D(Buffalo_NY), NYC;
    P(Erie_PA), D(Cleveland_OH), NYC;
    P(Perrysburg_OH), D(Cleveland_OH), NYC;
    P(Perrysburg_OH), D(Detroit_MI), NYC;
    P(Perrysburg_OH), P(Shipshewana_IN), NYC;
    P(Shipshewana_IN), P(South_Bend_IN), NYC;
    P(South_Bend_IN), D(Chicago_IL), NYC;
    P(Perrysburg_OH), P(Fort_Wayne_IN), NYC;
    P(Fort_Wayne_IN), P(Dayton_OH), NYC;
    P(Dayton_OH), D(Cincinnati_OH), NYC;
    P(Fort_Wayne_IN), P(Muncie_IN), NYC;
    P(Muncie_IN), D(Indianapolis_IN), NYC;
    P(Terre_Haute_IN), D(Indianapolis_IN), NYC;
    P(Terre_Haute_IN), P(Arcola_IL), NYC;
    P(Arcola_IL), D(St_Louis_MO), NYC;

    // PA
    P(Trenton_NJ), D(New_York_NY), PA;
    P(Trenton_NJ), D(Philadelphia_PA), PA;
    P(Pottstown_PA), D(Philadelphia_PA), PA;
    P(Pottstown_PA), D(Baltimore_MD), PA;
    P(Pottstown_PA), P(Lancaster_PA), PA;
    P(Bedford_PA), P(Lancaster_PA), PA;
    P(Bedford_PA), D(Pittsburgh_PA), PA;
    P(Youngstown_OH), D(Pittsburgh_PA), PA;
    P(New_Philadelphia_OH), D(Pittsburgh_PA), PA;
    P(New_Philadelphia_OH), D(Columbus_OH), PA;
    P(Youngstown_OH), P(Akron_OH), PA;
    P(Youngstown_OH), P(Erie_PA), PA;
    P(Erie_PA), D(Buffalo_NY), PA;
    P(Akron_OH), D(Cleveland_OH), PA;
    P(Akron_OH), D(Columbus_OH), PA;
    P(Dayton_OH), D(Columbus_OH), PA;
    P(Dayton_OH), D(Cincinnati_OH), PA;
    P(Dayton_OH), D(Indianapolis_IN), PA;
    P(Columbus_IN), D(Indianapolis_IN), PA;
    P(Columbus_IN), D(Louisville_KY), PA;
    P(Terre_Haute_IN), D(Indianapolis_IN), PA;
    P(West_Lafayette_IN), D(Indianapolis_IN), PA;
    P(West_Lafayette_IN), D(Chicago_IL), PA;
    P(Terre_Haute_IN), P(Effingham_IL), PA;
    P(Effingham_IL), D(St_Louis_MO), PA;
    D(Baltimore_MD), D(Philadelphia_PA), PA;

    // RF_AND_P
    D(Baltimore_MD), D(Richmond_VA), RF_AND_P;

    // B_AND_O
    D(Baltimore_MD), D(Washington_DC), B_AND_O;
    D(Baltimore_MD), P(Frederick_MD), B_AND_O;
    D(Washington_DC), P(Frederick_MD), B_AND_O;
    P(Cumberland_MD), P(Frederick_MD), B_AND_O;
    P(Cumberland_MD), P(Uniontown_PA), B_AND_O;
    D(Pittsburgh_PA), P(Uniontown_PA), B_AND_O;
    D(Pittsburgh_PA), P(Youngstown_OH), B_AND_O;
    P(Akron_OH), P(Youngstown_OH), B_AND_O;
    P(Akron_OH), P(Fremont_OH), B_AND_O;
    P(Ligonier_IN), P(Fremont_OH), B_AND_O;
    P(Ligonier_IN), P(Argos_IN), B_AND_O;
    D(Chicago_IL), P(Argos_IN), B_AND_O;
    // P(SubCity::Cumberland_MD), P(SubCity::Brideport_WV), B_AND_;
    P(Clarksburg_WV), P(Parkersburg_WV), B_AND_O;
    P(Chillicothe_OH), P(Parkersburg_WV), B_AND_O;
    P(Chillicothe_OH), D(Cincinnati_OH), B_AND_O;
    P(Columbus_IN), D(Cincinnati_OH), B_AND_O;
    P(Columbus_IN), P(Vincennes_IN), B_AND_O;
    P(Centralia_IL), P(Vincennes_IN), B_AND_O;
    P(Centralia_IL), D(St_Louis_MO), B_AND_O;

    // C_AND_O
    D(Buffalo_NY), P(Brantford_ON), C_AND_O;
    P(London_ON), P(Brantford_ON), C_AND_O;
    D(Detroit_MI), P(London_ON), C_AND_O;
    D(Detroit_MI), P(Perrysburg_OH), C_AND_O;
}

lazy_static::lazy_static! {
    pub static ref RAILROAD_GRAPH: UnGraph<C, Rail> = Rail::get_railroad_graph();
}
