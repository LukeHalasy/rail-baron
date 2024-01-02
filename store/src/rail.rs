pub use crate::main_city::City;
pub use crate::main_city::SubCity;

use serde::{Deserialize, Serialize};

use petgraph::graph::{NodeIndex, UnGraph};

pub use crate::Cash;

use strum::{EnumIter, IntoEnumIterator};

use std::collections::{HashMap, HashSet};
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

// impl convert from NodeIndex to C
// TODO: Need to refactor this to not repeate logic between the two conversions
impl std::convert::From<petgraph::prelude::NodeIndex> for C {
    fn from(node_index: petgraph::prelude::NodeIndex) -> Self {
        // iterate through C, find the index of the node
        let mut i = 0;

        for city in City::iter() {
            if i == node_index.index() {
                return C::D(city);
            }

            i += 1;
        }

        for sub_city in SubCity::iter() {
            if i == node_index.index() {
                return C::P(sub_city);
            }

            i += 1;
        }

        panic!("Could not find city for node index: {:?}", node_index)
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

                pub fn rail_to_cities_map() -> HashMap<Rail, HashSet<C>> {
                    let mut graph: HashMap<Rail, HashSet<C>> = HashMap::new();

                    $(
                        if let Some(city_list) = graph.get_mut(&Rail::$rr) {
                            city_list.insert($c1);
                            city_list.insert($c2);
                        } else {
                            let mut set = HashSet::new();
                            set.insert($c1);
                            set.insert($c2);
                            graph.insert(Rail::$rr, set);
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
    D(Washington_DC), D(Richmond_VA), RF_AND_P;

    // B_AND_O
    D(Baltimore_MD), D(Washington_DC), B_AND_O;
    D(Baltimore_MD), P(Frederick_MD), B_AND_O;
    D(Washington_DC), P(Frederick_MD), B_AND_O;
    P(Cumberland_MD), P(Frederick_MD), B_AND_O;
    P(Cumberland_MD), P(Union_Town_PA), B_AND_O;
    D(Pittsburgh_PA), P(Union_Town_PA), B_AND_O;
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

    // WP 
    P(Klamath_Falls_OR), P(Chester_CA), WP;
    P(Chester_CA), P(Herlong_CA), WP;
    P(Chester_CA), P(Paradise_CA), WP;
    P(Paradise_CA), D(Sacramento_CA), WP;
    P(Modesto_CA), D(Sacramento_CA), WP;
    P(Modesto_CA), D(San_Francisco_CA), WP;
    D(Reno_NV), P(Herlong_CA), WP;
    P(Sulphur_NV), P(Herlong_CA), WP;
    P(Sulphur_NV), P(Beowawe_NV), WP;
    P(Wells_NV), P(Beowawe_NV), WP;
    P(Wells_NV), P(Baker_NV), WP;
    D(Salt_Lake_City_UT), P(Baker_NV), WP;

    // SP 
    D(New_Orleans_LA), P(Houma_LA), SP;
    P(Lafayette_LA), P(Houma_LA), SP;
    P(Lafayette_LA), P(Beaumont_TX), SP;
    D(Houston_TX), P(Beaumont_TX), SP;
    D(Houston_TX), P(Livingston_TX), SP;
    P(Nacogdoches_TX), P(Livingston_TX), SP;
    P(Nacogdoches_TX), D(Shreveport_LA), SP;
    P(Flatonia_TX), P(Sealy_TX), SP;
    D(Houston_TX), P(Sealy_TX), SP;
    P(Flatonia_TX), D(San_Antonio_TX), SP;
    P(Hondo_TX), D(San_Antonio_TX), SP;
    P(Hondo_TX), P(Comstock_TX), SP;
    P(Sanderson_TX), P(Comstock_TX), SP;
    P(Sanderson_TX), P(Marfa_TX), SP;
    P(Sierra_Blanca_TX), P(Marfa_TX), SP;
    P(Sierra_Blanca_TX), D(El_Paso_TX), SP;
    P(Alamogordo_NM), D(El_Paso_TX), SP;
    P(Alamogordo_NM), P(Ancho_NM), SP;
    P(Santa_Rosa_NM), P(Ancho_NM), SP;
    P(Santa_Rosa_NM), D(Tucumcari_NM), SP;
    P(Deming_NM), D(El_Paso_TX), SP;
    P(Deming_NM), P(Lordsburg_NM), SP;
    P(Pomerene_AZ), P(Lordsburg_NM), SP;
    P(Pomerene_AZ), P(Mariana_AZ), SP;
    D(Phoenix_AZ), P(Mariana_AZ), SP;
    D(Phoenix_AZ), P(Aztec_NM), SP;
    P(Yuma_AZ), P(Aztec_NM), SP;
    P(Yuma_AZ), P(Brawley_CA), SP;
    P(Brawley_CA), P(San_Bernardino_CA), SP;
    D(Los_Angeles_CA), P(San_Bernardino_CA), SP;
    D(Los_Angeles_CA), P(Santa_Barbara_CA), SP;
    P(Santa_Maria_CA), P(Santa_Barbara_CA), SP;
    P(Santa_Maria_CA), P(San_Simeon_CA), SP;
    P(Big_Sur_CA), P(San_Simeon_CA), SP;
    P(Big_Sur_CA), P(San_Jose_CA), SP;
    D(San_Francisco_CA), P(San_Jose_CA), SP;
    D(San_Francisco_CA), P(Santa_Rosa_CA), SP;
    P(Fortuna_CA), P(Santa_Rosa_CA), SP;
    D(Sacramento_CA), P(Santa_Rosa_CA), SP;
    D(Sacramento_CA), P(Gold_Run_CA), SP;
    D(Reno_NV), P(Gold_Run_CA), SP;
    D(Reno_NV), P(Love_Lock_NV), SP;
    P(Beowawe_NV), P(Love_Lock_NV), SP;
    P(Beowawe_NV), P(Wells_NV), SP;
    P(West_Wendover_NV), P(Wells_NV), SP;
    P(West_Wendover_NV), P(Ogden_UT), SP;
    P(Fortuna_CA), P(Crescent_City_CA), SP;
    P(Grants_Pass_OR), P(Crescent_City_CA), SP;
    P(Grants_Pass_OR), P(Roseburg_OR), SP;
    P(Eugene_OR), P(Roseburg_OR), SP;
    P(Eugene_OR), P(Salem_OR), SP;
    D(Portland_OR), P(Salem_OR), SP;

    // ACL
    D(Richmond_VA), P(Emporia_VA), ACL;
    P(Enfield_NC), P(Emporia_VA), ACL;
    P(Enfield_NC), P(Greenville_NC), ACL;
    P(Dunn_NC), P(Greenville_NC), ACL;
    P(Dunn_NC), P(Fayetteville_NC), ACL;
    P(Manning_SC), P(Fayetteville_NC), ACL;
    P(Manning_SC), D(Charleston_SC), ACL;
    P(Savannah_GA), D(Charleston_SC), ACL;
    P(Savannah_GA), P(Willacoochee_GA), ACL;
    D(Jacksonville_FL), P(Willacoochee_GA), ACL;
    P(Arabi_GA), P(Willacoochee_GA), ACL;
    P(Arabi_GA), P(Perry_GA), ACL;
    D(Atlanta_GA), P(Perry_GA), ACL;
    P(Roanoke_AL), P(Perry_GA), ACL;
    P(Roanoke_AL), D(Birmingham_AL), ACL;
    P(Willacoochee_GA), P(Moultrie_GA), ACL;
    P(Radium_Springs_GA), P(Moultrie_GA), ACL;
    P(Radium_Springs_GA), P(Arabi_GA), ACL;
    P(Radium_Springs_GA), P(Dothan_AL), ACL;
    P(Radium_Springs_GA), P(Tallahassee_FL), ACL;
    P(Moultrie_GA), P(Dothan_AL), ACL;
    P(Luverne_AL), P(Dothan_AL), ACL;
    P(Luverne_AL), P(Montgomery_AL), ACL;
    P(Cedar_Key_FL), P(Tallahassee_FL), ACL;
    P(Cedar_Key_FL), D(Jacksonville_FL), ACL;
    P(Cedar_Key_FL), P(Spring_Hill_FL), ACL;
    D(Tampa_FL), P(Spring_Hill_FL), ACL;
    
    // N_AND_W
    D(Norfolk_VA), P(Emporia_VA), N_AND_W;
    P(Brookneal_VA), P(Emporia_VA), N_AND_W;
    P(Brookneal_VA), P(Roanoke_VA), N_AND_W;
    P(Wytheville_VA), P(Roanoke_VA), N_AND_W;
    P(Wytheville_VA), P(Marion_VA), N_AND_W;
    P(Wytheville_VA), P(Grundy_VA), N_AND_W;
    P(Paintsville_KY), P(Grundy_VA), N_AND_W;
    P(Paintsville_KY), P(Huntington_WV), N_AND_W;
    P(Winchester_OH), P(Huntington_WV), N_AND_W;
    P(Chillicothe_OH), P(Huntington_WV), N_AND_W;
    P(Chillicothe_OH), D(Columbus_OH), N_AND_W;
    P(Winchester_OH), D(Cincinnati_OH), N_AND_W;
    P(Bristol_VA), P(Marion_VA), N_AND_W;
    
    // SAL
    // TODO: Bremen appears between cedar bluff and atlanta and looks like it's on the route despite not being. Need to fix this to not confuse players.
    D(Miami_FL), P(West_Palm_Beach_FL), SAL;
    P(Sebring_FL), P(West_Palm_Beach_FL), SAL;
    P(Sebring_FL), P(Winter_Haven_FL), SAL;
    P(Spring_Hill_FL), P(Winter_Haven_FL), SAL;
    P(Spring_Hill_FL), D(Tampa_FL), SAL;
    P(Spring_Hill_FL), P(Ocala_FL), SAL;
    D(Jacksonville_FL), P(Ocala_FL), SAL;
    D(Jacksonville_FL), P(Lee_FL), SAL;
    D(Jacksonville_FL), P(Brunswick_GA), SAL;
    P(Savannah_GA), P(Brunswick_GA), SAL;
    P(Savannah_GA), D(Charleston_SC), SAL;
    P(Conway_SC), D(Charleston_SC), SAL;
    P(Conway_SC), P(Fayetteville_NC), SAL;
    P(Biscoe_NC), P(Fayetteville_NC), SAL;
    P(Biscoe_NC), P(Sanford_NC), SAL;
    P(Raleigh_NC), P(Sanford_NC), SAL;
    P(Raleigh_NC), P(Warrenton_NC), SAL;
    P(Emporia_VA), P(Warrenton_NC), SAL;
    P(Emporia_VA), D(Richmond_VA), SAL;
    P(Biscoe_NC), P(Chesterfield_SC), SAL;
    D(Charlotte_TN), P(Chesterfield_SC), SAL;
    P(Columbia_SC), P(Chesterfield_SC), SAL;
    P(Columbia_SC), P(Augusta_GA), SAL;
    P(Siloam_GA), P(Augusta_GA), SAL;
    P(Siloam_GA), D(Atlanta_GA), SAL;
    P(Cedar_Bluff_MS), D(Atlanta_GA), SAL;
    P(Cedar_Bluff_MS), D(Birmingham_AL), SAL;
    P(Tallahassee_FL), P(Lee_FL), SAL;
    P(Tallahassee_FL), P(Blountstown_FL), SAL;
    P(Tallahassee_FL), P(Dothan_AL), SAL;
    P(Blountstown_FL), P(Dothan_AL), SAL;
    P(Midway_AL), P(Dothan_AL), SAL;
    P(Midway_AL), P(Montgomery_AL), SAL;

    // SOU
    D(New_Orleans_LA), P(Franklinton_LA), SOU;
    P(Hattiesburg_MS), P(Franklinton_LA), SOU;
    P(Hattiesburg_MS), P(Meridian_MS), SOU;
    // P(Tuscaloosa_AL), P(Meridian_MS), SOU;
    // P(Tuscaloosa_AL), D(Birmingham_AL), SOU;
    P(Double_Springs_AL), D(Birmingham_AL), SOU;
    P(Double_Springs_AL), P(Corinth_MS), SOU;
    D(Memphis_TN), P(Corinth_MS), SOU;
    P(Moulton_AL), P(Corinth_MS), SOU;
    P(Moulton_AL), P(Pleasant_Hill_TN), SOU;
    D(Chattanooga_TN), P(Pleasant_Hill_TN), SOU;
    D(Chattanooga_TN), P(Athens_TN), SOU;
    D(Chattanooga_TN), P(Rome_GA), SOU;
    P(Athens_TN), P(Rome_GA), SOU;
    P(Athens_TN), D(Knoxville_TN), SOU;
    P(Mosheim_TN), D(Knoxville_TN), SOU;
    P(Mosheim_TN), P(Bristol_VA), SOU;
    D(Atlanta_GA), P(Rome_GA), SOU;
    D(Atlanta_GA), P(Bremen_GA), SOU;
    D(Birmingham_AL), P(Bremen_GA), SOU;
    D(Atlanta_GA), P(Commerce_GA), SOU;
    P(Clemson_SC), P(Commerce_GA), SOU;
    P(Clemson_SC), P(Spartanburg_SC), SOU;
    D(Charlotte_TN), P(Spartanburg_SC), SOU;
    D(Charlotte_TN), P(Greensboro_NC), SOU;
    P(Bassett_VA), P(Greensboro_NC), SOU;
    P(Bassett_VA), P(Roanoke_VA), SOU;
    P(Charlottesville_VA), P(Roanoke_VA), SOU;
    P(Charlottesville_VA), P(Culpeper_VA), SOU;
    // TODO: Look into tank fight tactics

    // T_AND_P
    D(El_Paso_TX), P(Sierra_Blanca_TX), T_AND_P;
    P(Toyah_TX), P(Sierra_Blanca_TX), T_AND_P;
    P(Toyah_TX), P(Odessa_TX), T_AND_P;
    P(Big_Spring_TX), P(Odessa_TX), T_AND_P;
    P(Big_Spring_TX), P(Abilene_TX), T_AND_P;
    P(Ranger_TX), P(Abilene_TX), T_AND_P;
    P(Ranger_TX), D(Fort_Worth_TX), T_AND_P;
    D(Dallas_TX), D(Fort_Worth_TX), T_AND_P;
    D(Dallas_TX), P(Lindale_TX), T_AND_P;
    P(Longview_TX), P(Lindale_TX), T_AND_P;
    P(Longview_TX), D(Shreveport_LA), T_AND_P;
    P(Ashdown_AR), D(Shreveport_LA), T_AND_P;
    P(Natchitoches_LA), D(Shreveport_LA), T_AND_P;
    P(Natchitoches_LA), P(Alexandria_LA), T_AND_P;
    P(Baton_Rouge_LA), P(Alexandria_LA), T_AND_P;
    P(Baton_Rouge_LA), D(New_Orleans_LA), T_AND_P;

    // L_AND_N
    D(Louisville_KY), P(Elizabethtown_KY), L_AND_N;
    P(Bowling_Green_KY), P(Elizabethtown_KY), L_AND_N;
    P(Bowling_Green_KY), D(Nashville_TN), L_AND_N;
    P(Lewisburg_TN), D(Nashville_TN), L_AND_N;
    P(Lewisburg_TN), P(Cullman_AL), L_AND_N;
    D(Birmingham_AL), P(Cullman_AL), L_AND_N;
    D(Birmingham_AL), P(Montgomery_AL), L_AND_N;
    P(Greenville_AL), P(Montgomery_AL), L_AND_N;
    P(Greenville_AL), P(Evergreen_AL), L_AND_N;
    P(Niceville_FL), P(Evergreen_AL), L_AND_N;
    D(Mobile_AL), P(Evergreen_AL), L_AND_N;
    D(Mobile_AL), P(Gulfport_MS), L_AND_N;
    D(New_Orleans_LA), P(Gulfport_MS), L_AND_N;
    P(Niceville_FL), P(Bonifay_FL), L_AND_N;
    P(Blountstown_FL), P(Bonifay_FL), L_AND_N;
    P(Hopkinsville_KY), D(Nashville_TN), L_AND_N;
    P(Camden_TN), D(Nashville_TN), L_AND_N;
    P(Camden_TN), P(Brownsville_TN), L_AND_N;
    D(Memphis_TN), P(Brownsville_TN), L_AND_N;
    P(Tullahoma_TN), D(Nashville_TN), L_AND_N;
    P(Tullahoma_TN), D(Chattanooga_TN), L_AND_N;
    P(Ellijay_GA), D(Chattanooga_TN), L_AND_N;
    P(Ellijay_GA), D(Atlanta_GA), L_AND_N;
    P(Ellijay_GA), P(Tellico_Plains_TN), L_AND_N;
    D(Knoxville_TN), P(Tellico_Plains_TN), L_AND_N;
    D(Knoxville_TN), P(Williamsburg_KY), L_AND_N;
    P(Mt_Vernon_KY), P(Williamsburg_KY), L_AND_N;
    P(Mt_Vernon_KY), P(Lexington_KY), L_AND_N;
    D(Cincinnati_OH), P(Lexington_KY), L_AND_N;
    
}

lazy_static::lazy_static! {
    pub static ref RAILROAD_GRAPH: UnGraph<C, Rail> = Rail::get_railroad_graph();
    #[allow(dead_code)]
    pub static ref RAILS_TO_CITIES_MAP: HashMap<Rail, HashSet<C>> = Rail::rail_to_cities_map();
}
