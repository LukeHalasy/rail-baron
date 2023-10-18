pub use crate::city::City;
pub use crate::deed::Deed;
pub use crate::sub_city::SubCity;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
// // Should replace the railroad part in subcities with the below syntax.. that way I can have all railroads
// // cleary documented and don't need to replicate paths twice (for start -> destination and destination -> start)
// // also it's clear as I am making progress
// // also I can have a test that ensures all *city* are reachable

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum C {
    D(City),    // Destination
    P(SubCity), // Path
}

macro_rules! rail_roads {
    // Will need to change this so that you can specify either a city or subcity in both parts
    ($($c1:expr, $c2:expr, $rr:tt);*$(;)?) => {
        paste::paste! {
            impl Deed {
                pub fn get_railroad_graph() -> HashMap<C, Vec<(C, Deed)>> {
                    let mut graph: HashMap<C, Vec<(C, Deed)>> = HashMap::new();

                    $(
                    if let Some(rf) = graph.get_mut(&$c1) {
                        rf.push(($c2, Deed::$rr))
                    } else {
                        graph.insert($c1, vec![($c2, Deed::$rr)]);
                    }

                    if let Some(rf) = graph.get_mut(&$c2) {
                        rf.push(($c1, Deed::$rr))
                    } else {
                        graph.insert($c2, vec![($c1, Deed::$rr)]);
                    }
                    )*

                    graph
                }
            }
        }
    };
}

rail_roads! {
    // B_AND_M
    C::P(SubCity::Springfield_MA), C::D(City::Albany_NY), B_AND_M ;
    C::P(SubCity::Springfield_MA), C::D(City::Boston_MA), B_AND_M ;
    C::P(SubCity::Springfield_MA), C::P(SubCity::Concord_NH), B_AND_M ;
    C::P(SubCity::Concord_NH), C::D(City::Portland_ME), B_AND_M;

    // NYNH_AND_H
    C::P(SubCity::Providence_RI), C::D(City::Boston_MA), NYNH_AND_H;
    C::P(SubCity::Providence_RI), C::P(SubCity::New_Haven_CT), NYNH_AND_H;
    C::P(SubCity::New_Haven_CT), C::D(City::New_York_NY), NYNH_AND_H;

    // NYC
    C::P(SubCity::Kingston_NY), C::D(City::New_York_NY), NYC;
    C::P(SubCity::Kingston_NY), C::D(City::Albany_NY), NYC;
    C::P(SubCity::Syracuse_NY), C::D(City::Albany_NY), NYC;
    C::P(SubCity::Syracuse_NY), C::P(SubCity::Rochester_NY), NYC;
    C::P(SubCity::Rochester_NY), C::D(City::Buffalo_NY), NYC;
    C::P(SubCity::Erie_PA), C::D(City::Buffalo_NY), NYC;
    C::P(SubCity::Erie_PA), C::D(City::Cleveland_OH), NYC;
    C::P(SubCity::Perrysburg_OH), C::D(City::Cleveland_OH), NYC;
    C::P(SubCity::Perrysburg_OH), C::D(City::Detroit_MI), NYC;
    C::P(SubCity::Perrysburg_OH), C::P(SubCity::Shipshewana_IN), NYC;
    C::P(SubCity::Shipshewana_IN), C::P(SubCity::South_Bend_IN), NYC;
    C::P(SubCity::South_Bend_IN), C::D(City::Chicago_IL), NYC;
    C::P(SubCity::Perrysburg_OH), C::P(SubCity::Fort_Wayne_IN), NYC;
    C::P(SubCity::Fort_Wayne_IN), C::P(SubCity::Dayton_OH), NYC;
    C::P(SubCity::Dayton_OH), C::D(City::Cincinnati_OH), NYC;
    C::P(SubCity::Fort_Wayne_IN), C::P(SubCity::Muncie_IN), NYC;
    C::P(SubCity::Muncie_IN), C::D(City::Indianapolis_IN), NYC;
    C::P(SubCity::Terre_Haute_IN), C::D(City::Indianapolis_IN), NYC;
    C::P(SubCity::Terre_Haute_IN), C::P(SubCity::Arcola_IL), NYC;
    C::P(SubCity::Arcola_IL), C::D(City::St_Louis_MO), NYC;

    // PA
    C::P(SubCity::Trenton_NJ), C::D(City::New_York_NY), PA;
    C::P(SubCity::Trenton_NJ), C::D(City::Philadelphia_PA), PA;
    C::P(SubCity::Pottstown_PA), C::D(City::Philadelphia_PA), PA;
    C::P(SubCity::Pottstown_PA), C::D(City::Baltimore_MD), PA;
    C::P(SubCity::Pottstown_PA), C::P(SubCity::Lancaster_PA), PA;
    C::P(SubCity::Bedford_PA), C::P(SubCity::Lancaster_PA), PA;
    C::P(SubCity::Bedford_PA), C::D(City::Pittsburgh_PA), PA;
    C::P(SubCity::Youngstown_OH), C::D(City::Pittsburgh_PA), PA;
    C::P(SubCity::New_Philadelphia_OH), C::D(City::Pittsburgh_PA), PA;
    C::P(SubCity::New_Philadelphia_OH), C::D(City::Columbus_OH), PA;
    C::P(SubCity::Youngstown_OH), C::P(SubCity::Akron_OH), PA;
    C::P(SubCity::Youngstown_OH), C::P(SubCity::Erie_PA), PA;
    C::P(SubCity::Erie_PA), C::D(City::Buffalo_NY), PA;
    C::P(SubCity::Akron_OH), C::D(City::Cleveland_OH), PA;
    C::P(SubCity::Akron_OH), C::D(City::Columbus_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(City::Columbus_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(City::Cincinnati_OH), PA;
    C::P(SubCity::Dayton_OH), C::D(City::Indianapolis_IN), PA;
    C::P(SubCity::Columbus_IN), C::D(City::Indianapolis_IN), PA;
    C::P(SubCity::Columbus_IN), C::D(City::Louisville_KY), PA;
    C::P(SubCity::Terre_Haute_IN), C::D(City::Indianapolis_IN), PA;
    C::P(SubCity::West_Lafayette_IN), C::D(City::Indianapolis_IN), PA;
    C::P(SubCity::West_Lafayette_IN), C::D(City::Chicago_IL), PA;
    C::P(SubCity::Terre_Haute_IN), C::P(SubCity::Effingham_IL), PA;
    C::P(SubCity::Effingham_IL), C::D(City::St_Louis_MO), PA;
    C::D(City::Baltimore_MD), C::D(City::Philadelphia_PA), PA;

    // RF_AND_P
    C::D(City::Baltimore_MD), C::D(City::Richmond_VA), RF_AND_P;

    // B_AND_O
    C::D(City::Baltimore_MD), C::D(City::Washington_DC), B_AND_O;
    C::D(City::Baltimore_MD), C::P(SubCity::Frederick_MD), B_AND_O;
    C::D(City::Washington_DC), C::P(SubCity::Frederick_MD), B_AND_O;
    C::P(SubCity::Cumberland_MD), C::P(SubCity::Frederick_MD), B_AND_O;
    C::P(SubCity::Cumberland_MD), C::P(SubCity::Uniontown_PA), B_AND_O;
    C::D(City::Pittsburgh_PA), C::P(SubCity::Uniontown_PA), B_AND_O;
    C::D(City::Pittsburgh_PA), C::P(SubCity::Youngstown_OH), B_AND_O;
    C::P(SubCity::Akron_OH), C::P(SubCity::Youngstown_OH), B_AND_O;
    C::P(SubCity::Akron_OH), C::P(SubCity::Fremont_OH), B_AND_O;
    C::P(SubCity::Ligonier_IN), C::P(SubCity::Fremont_OH), B_AND_O;
    C::P(SubCity::Ligonier_IN), C::P(SubCity::Argos_IN), B_AND_O;
    C::D(City::Chicago_IL), C::P(SubCity::Argos_IN), B_AND_O;
    C::P(SubCity::Cumberland_MD), C::P(SubCity::Brideport_WV), B_AND_O;
    C::P(SubCity::Clarksburg_WV), C::P(SubCity::Parkersburg_WV), B_AND_O;
    C::P(SubCity::Chillicothe_OH), C::P(SubCity::Parkersburg_WV), B_AND_O;
    C::P(SubCity::Chillicothe_OH), C::D(City::Cincinnati_OH), B_AND_O;
    C::P(SubCity::Columbus_IN), C::D(City::Cincinnati_OH), B_AND_O;
    C::P(SubCity::Columbus_IN), C::P(SubCity::Vincennes_IN), B_AND_O;
    C::P(SubCity::Centralia_IL), C::P(SubCity::Vincennes_IN), B_AND_O;
    C::P(SubCity::Centralia_IL), C::D(City::St_Louis_MO), B_AND_O;
}
