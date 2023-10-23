use serde::{Deserialize, Serialize};

pub use crate::deed::Deed;

pub use crate::state::State;
pub use crate::sub_city::SubCity;

macro_rules! cities {
    ($([$c:tt: $s:tt] => ($lat:literal, $long:literal)),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum City { $([<$c _ $s>]),* }
            impl City {
                pub const fn cities() -> &'static [Self] {
                    &[$(Self::[<$c _ $s>]),*]
                }

                pub const fn state(&self) -> State {
                    match self {
                        $(Self::[<$c _ $s>] => State::$s),*
                    }
                }

                pub fn coordinates(&self) -> geoutils::Location {
                    match self {
                        $(Self::[<$c _ $s>] => geoutils::Location::new($lat, $long)),*
                    }
                }
            }

            impl std::fmt::Display for City {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        $(Self::[<$c _ $s>] => write!(f, "{}, {}", stringify!($c), stringify!($s))),*
                    }
                }
            }
        }
    }
}

cities! {
    [Albany: NY] => (42.6526, -73.7562),
    [Atlanta: GA] => (33.7490, -84.3879),
    [Baltimore: MD] => (39.2904, -76.6122),
    [Billings: MT] => (45.7833, -108.5007),
    [Birmingham: AL] => (33.5207, -86.8025),
    [Boston: MA] => (42.3601, -71.0589),
    [Buffalo: NY] => (42.8802, -78.8782),
    [Butte: MT] => (46.0038, -112.5339),
    [Casper: WY] => (42.8666, -106.3131),
    [Charleston: SC] => (32.7765, -79.9311),
    [Charlotte: TN] => (35.2271, -80.8431),
    [Chattanooga: TN] => (35.0456, -85.3097),
    [Chicago: IL] => (41.8781, -87.6298),
    [Cincinnati: OH] => (39.1031, -84.5120),
    [Cleveland: OH] => (41.4993, -81.6944),
    [Columbus: OH] => (39.9612, -82.9988),
    [Dallas: TX] => (32.7767, -96.7970),
    [Denver: CO] => (39.7392, -104.9903),
    [Des_Moines: IA] => (41.5868, -93.6250),
    [Detroit: MI] => (42.3314, -83.0458),
    [El_Paso: TX] => (31.7619, -106.4850),
    [Fargo: ND] => (46.8772, -96.7898),
    [Fort_Worth: TX] => (32.7555, -97.3308),
    [Houston: TX] => (29.7604, -95.3698),
    [Indianapolis: IN] => (39.7684, -86.1581),
    [Jacksonville: FL] => (30.3322, -81.6557),
    [Kansas_City: MO] => (39.0997, -94.5786),
    [Knoxville: TN] => (35.9606, -83.9207),
    [Las_Vegas: NV] => (36.1699, -115.1398),
    [Little_Rock: AK] => (34.7465, -92.2896),
    [Los_Angeles: CA] => (34.0522, -118.2437),
    [Louisville: KY] => (38.2527, -85.7585),
    [Memphis: TN] => (35.1495, -90.0490),
    [Miami: FL] => (25.7617, -80.1918),
    [Milwaukee: WI] => (43.0389, -87.9065),
    [Minneapolis: MN] => (44.9778, -93.2650),
    [Mobile: AL] => (30.6954, -88.0399),
    [Nashville: TN] => (36.1627, -86.7816),
    [New_Orleans: LA] => (29.9511, -90.0715),
    [New_York: NY] => (40.7128, -74.0060),
    [Norfolk: VA] => (36.8508, -76.2859),
    [Oklahoma_City: OK] => (35.4634, -97.5151),
    [Omaha: NE] => (41.2565, -95.9345),
    [Philadelphia: PA] => (39.9526, -75.1652),
    [Phoenix: AZ] => (33.4484, -112.0740),
    [Pittsburgh: PA] => (40.4406, -79.9959),
    [Pocatello: ID] => (42.8713, -112.4445),
    [Portland: ME] => (43.6591, -70.2568),
    [Portland: OR] => (45.5051, -122.6750),
    [Pueblo: CO] => (38.2544, -104.6091),
    [Rapid_City: SD] => (44.0805, -103.2310),
    [Reno: NV] => (39.5296, -119.8138),
    [Richmond: VA] => (37.5407, -77.4360),
    [Sacramento: CA] => (38.5816, -121.4944),
    [Salt_Lake_City: UT] => (40.7608, -111.8910),
    [San_Antonio: TX] => (29.4241, -98.4936),
    [San_Diego: CA] => (32.7157, -117.1611),
    [San_Francisco: CA] => (37.7749, -122.4194),
    [Seattle: WA] => (47.6062, -122.3321),
    [Shreveport: LA] => (32.5252, -93.7502),
    [Spokane: WA] => (47.6588, -117.4260),
    [St_Louis: MO] => (38.6270, -90.1994),
    [Tampa: FL] => (27.9506, -82.4572),
    [Tucumcari: NM] => (35.1717, -103.7250),
    [Washington: DC] => (38.8951, -77.0369)
}
