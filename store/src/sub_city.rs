pub use crate::main_city::City;
pub use crate::state::State;

use serde::{Deserialize, Serialize};

// ($($c:tt: $s:tt => [$($nc:tt: [$($d:tt),*$(,)?]),*$(,)?] => [$($nsc:tt: [$($sd:tt),*$(,)?]),*$(,)?]),*$(,)?) => {
macro_rules! sub_cities {
    ($([$c:tt: $s:tt] => ($lat:literal, $long:literal)),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum SubCity { $([<$c _ $s>]),* }
            impl SubCity {
                pub const fn sub_cities() -> &'static [Self] {
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

            impl std::fmt::Display for SubCity {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        $(Self::[<$c _ $s>] => write!(f, "{}, {}", stringify!($c), stringify!($s))),*
                    }
                }
            }
        }
    }
}

// I think I should get rid of the railroad part of this..
sub_cities! {
    [Concord: NH] => (43.2081, -71.5376),
    [Springfield: MA] => (42.1015, -72.5898),
    [Providence: RI] => (41.824, -71.4128),
    [New_Haven: CT] => (41.3082, -72.9251),
    [Kingston: NY] => (41.9267, -73.9974),
    [Syracuse: NY] => (43.0481, -76.1474),
    [Rochester: NY] => (43.161, -77.6109),
    [Erie: PA] => (42.1292, -80.0851),
    [Trenton: NJ] => (40.2206, -74.7697),
    [Pottstown: PA] => (40.2454, -75.6496),
    [Lancaster: PA] => (40.0381, -76.3057),
    [Bedford: PA] => (40.0185, -78.5036),
    [Uniontown: PA] => (39.8974, -79.7497),
    [Frederick: MD] => (39.4142, -77.4105),
    [Brantford: ON] => (43.1394, -80.2517),
    [London: ON] => (42.9849, -81.2453),
    [Youngstown: OH] => (41.0998, -80.6495),
    [Akron: OH] => (41.0814, -81.519),
    [New_Philadelphia: OH] => (40.4897, -81.4457),
    [Fremont: OH] => (41.3503, -83.1219),
    [Findlay: OH] => (41.0442, -83.6499),
    [Chillicothe: OH] => (39.3331, -82.9824),
    [Winchester: OH] => (39.2719, -83.4409),
    [Maysville: OH] => (38.6412, -83.744),
    [Perrysburg: OH] => (41.5574, -83.6271),
    [Dayton: OH] => (39.7589, -84.1916),
    [Shipshewana: IN] => (41.6621, -85.5822),
    [South_Bend: IN] => (41.6769, -86.269),
    [Fort_Wayne: IN] => (41.0793, -85.1394),
    [Muncie: IN] => (40.1934, -85.3864),
    [Columbus: IN] => (39.2014, -85.9214),
    [Vincennes: IN] => (38.6773, -87.5286),
    [Terre_Haute: IN] => (39.4688, -87.4139),
    [Arcola: IL] => (39.6876, -88.3076),
    [Effingham: IL] => (39.1201, -88.5434),
    [West_Lafayette: IN] => (40.4259, -86.9081),
    [Cumberland: MD] => (39.6528, -78.7624),
    [Clarksburg: WV] => (39.2817, -80.3428),
    [Centralia: IL] => (38.5252, -89.1333),
    [Parkersburg: WV] => (39.2667, -81.5615),
    [Bridgeport: WV] => (39.2862, -80.2569),
    [Argos: IN] => (41.2376, -86.2502),
    [Ligonier: IN] => (41.4649, -85.5892),
    [Kenosha: WI] => (42.5847, -87.8212),
    [Madison: WI] => (43.0731, -89.4012),
    [Green_Bay: WI] => (44.5133, -88.0134),
    [Eau_Claire: WI] => (44.8113, -91.4985),
    [Wisconsin_Dells: WI] => (43.6275, -89.7702),
    [La_Crosse: WI] => (43.8014, -91.2396),
    [Gold_Run: CA] => (39.0818, -120.8135),
    [Herlong: CA] => (40.1662, -120.1438),
    [Santa_Rosa: CA] => (38.4405, -122.7141),
    [San_Jose: CA] => (37.3541, -121.9552),
    [Santa_Barbara: CA] => (34.4208, -119.6982),
    [Santa_Maria: CA] => (34.953, -120.4357),
    [San_Simeon: CA] => (35.6433, -121.1894),
    [Big_Sur: CA] => (36.2538, -121.7868),
    [Lancaster: CA] => (34.6868, -118.1542),
    [Bakersfield: CA] => (35.3733, -119.0187),
    [Modesto: CA] => (37.6469, -121.0624),
    [Amboy: CA] => (34.5531, -115.7487),
    [Sulphur: OK] => (34.5074, -96.9917),
    [Lovelock: NV] => (40.1768, -118.479),
    [Beowawe: NV] => (40.6437, -116.4275),
    [Indio: CA] => (33.7206, -116.2156),
    [Blythe: CA] => (33.6175, -114.5883),
    [Fresno: CA] => (36.7468, -119.7726),
    [Tulare: CA] => (36.2077, -119.3473),
    [Helper: UT] => (39.6843, -110.8562),
    [Green_River: UT] => (38.9961, -110.1595),
    [Fruita: CO] => (39.1582, -108.7287),
    [Nephi: UT] => (39.7106, -111.8356),
    [Fillmore: UT] => (38.9682, -112.3209),
    [San_Bernardino: CA] => (34.1083, -117.2898),
    [Chester: CA] => (40.3014, -121.2394),
    [Paradise: CA] => (39.7596, -121.6219),
    [Brawley: CA] => (32.9787, -115.5303),
    [Yuma: AZ] => (32.6927, -114.6286),
    [Aztec: NM] => (36.8293, -107.9922),
    [Crescent_City: CA] => (41.7566, -124.2021),
    [Fortuna: CA] => (40.5982, -124.1573),
    [Mesquite: NV] => (36.8095, -114.0682),
    [Enoch: UT] => (37.8217, -113.5572),
    [Deming: NM] => (32.2732, -107.7544),
    [Mariana: AZ] => (32.1808, -111.318),
    [Lordsburg: NM] => (32.3556, -108.7046),
    [Pomerene: AZ] => (32.0168, -110.0467),
    [Wickenburg: AZ] => (33.9687, -112.7326),
    [Marfa: TX] => (30.309, -104.0209),
    [Sierra_Blanca: TX] => (31.1795, -105.3504),
    [Toyah: TX] => (31.3848, -103.9033),
    [Santa_Rosa: NM] => (34.9387, -104.4084),
    [Albuquerque: NM] => (35.1107, -106.6099),
    [Socorro: NM] => (34.0584, -106.8906),
    [Arrey: NM] => (32.9702, -107.1023),
    [Ancho: NM] => (34.2713, -106.5488),
    [Alamogordo: NM] => (32.8995, -105.9603),
    [Grants: NM] => (35.1473, -107.8514),
    [Church_Rock: NM] => (35.4959, -108.7516),
    [Sedona: AZ] => (34.8697, -111.7609),
    [Taylor: AZ] => (34.4664, -110.078),
    [Springer: NM] => (36.3615, -104.5955),
    [Watrous: NM] => (35.7964, -104.4275),
    [Clovis: NM] => (34.4048, -103.2052),
    [Salem: OR] => (44.9429, -123.0351),
    [Eugene: OR] => (44.0521, -123.0868),
    [Grants_Pass: OR] => (42.439, -123.3284),
    [Roseburg: OR] => (43.2165, -123.3417),
    [Twin_Falls: ID] => (42.5622, -114.4607),
    [Biggs_Junction: OR] => (45.7106, -120.1163),
    [Pendleton: OR] => (45.6721, -118.7886),
    [Mountain_Home: ID] => (43.1311, -115.6882),
    [Boise: ID] => (43.6150, -116.2023),
    [Baker_City: OR] => (44.7749, -117.8321),
    [Huntington: OR] => (44.3467, -117.2293),
    [Shelby: MT] => (48.5171, -111.8562),
    [Wilbur: WA] => (47.7744, -118.6904),
    [Olympia: WA] => (47.0379, -122.9007),
    [Wapato: WA] => (46.4485, -120.4323),
    [Bristol: WA] => (46.2336, -119.2078),
    [Ritzville: WA] => (47.1294, -118.3759),
    [Colfax: WA] => (46.8792, -117.3597),
    [Klamath_Falls: OR] => (42.2249, -121.7817),
    [Chemult: OR] => (43.2169, -121.7836),
    [Seven_Mile: WA] => (48.1029, -117.2418),
    [Shaniko: OR] => (45.0214, -120.7553),
    [Dillon: MT] => (45.2158, -112.6831),
    [Monida: MT] => (44.5573, -111.7169),
    [Missoula: MT] => (46.8721, -113.994),
    [Noxon: MT] => (48.1975, -115.7796),
    [Thompson_Falls: MT] => (47.5964, -115.3433),
    [St_Regis: MT] => (47.3095, -115.0137),
    [Cataldo: ID] => (47.5488, -116.2903),
    [Custer: SD] => (43.7319, -103.6175),
    [Cheyenne: WY] => (41.1399, -104.8202),
    [Hardin: MT] => (45.7313, -107.6107),
    [Sheridan: WY] => (44.7972, -106.9569),
    [Kaycee: WY] => (43.7071, -106.6329),
    [Big_Timber: MT] => (45.835, -109.9632),
    [Bozeman: MT] => (45.677, -111.0429),
    [Townsend: MT] => (46.3314, -111.4954),
    [Provo: UT] => (40.2338, -111.6585),
    [Declo: ID] => (42.5248, -113.6434),
    [Harlem: MT] => (48.5337, -108.7852),
    [Havre: MT] => (48.5487, -109.6845),
    [Sandpoint: ID] => (48.2766, -116.5534),
    [Whitefish: MT] => (48.4111, -114.3376),
    [Chester: MT] => (48.5125, -110.9935),
    [Culbertson: MT] => (48.1479, -105.7422),
    [Wolf_Point: MT] => (48.0869, -105.6354),
    [Glasgow: MT] => (48.1972, -106.6359),
    [Malta: MT] => (48.3566, -107.863),
    [Marblemount: WA] => (48.5102, -121.4029),
    [Idaho_Falls: ID] => (43.4926, -112.0401),
    [Ogden: UT] => (41.223, -111.9738),
    [Malad_City: ID] => (42.1937, -112.2526),
    [Montpelier: ID] => (42.3223, -111.2942),
    [Neihart: MT] => (46.9229, -110.6958),
    [Geraldine: MT] => (47.5083, -110.2661),
    [Forsyth: MT] => (46.2725, -106.6862),
    [Roundup: MT] => (46.4512, -108.5421),
    [Harlowton: MT] => (46.4293, -109.8407),
    [West_Wendover: NV] => (40.7361, -114.0737),
    [Baker: MT] => (46.3589, -104.2782),
    [Laramie: WY] => (41.3114, -105.5904),
    [Elk_Mountain: WY] => (41.6831, -106.4176),
    [Creston: MT] => (48.4231, -113.8408),
    [Evanston: WY] => (41.2683, -110.9632),
    [Little_America: WY] => (41.4386, -109.9632),
    [Point_Of_Rocks: WY] => (41.6663, -108.7739),
    [Bloomfield: MT] => (47.5285, -104.4985),
    [Terry: MT] => (46.7838, -105.3162),
    [Wibaux: MT] => (46.9857, -104.1921)

}
