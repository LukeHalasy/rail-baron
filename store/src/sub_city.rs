pub use crate::main_city::City;
pub use crate::state::State;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

// ($($c:tt: $s:tt => [$($nc:tt: [$($d:tt),*$(,)?]),*$(,)?] => [$($nsc:tt: [$($sd:tt),*$(,)?]),*$(,)?]),*$(,)?) => {
macro_rules! sub_cities {
    ($([$c:tt: $s:tt] => ($lat:literal, $long:literal)),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, EnumIter)]
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
    // Canada
    [Brantford: ON] => (43.1334, -80.2664),
    [London: ON] => (42.9849, -81.2453),

    // North_East
    [Erie: PA] => (42.1292, -80.0851),
    [Concord: NH] => (43.2081, -71.5376),
    [Providence: RI] => (41.8240, -71.4128),
    [New_Haven: CT] => (41.3082, -72.9251),
    [Springfield: MA] => (42.1015, -72.5898),
    [Kingston: NY] => (41.9270, -73.9974),
    [Trenton: NJ] => (40.2206, -74.7597),
    [Rochester: NY] => (43.1573, -77.6152),
    [Syracuse: NY] => (43.0481, -76.1474),
    [Frederick: MD] => (39.4143, -77.4105),
    [Lancaster: PA] => (40.0379, -76.3055),
    [Bedford: PA] => (40.0187, -78.5039),
    [Pottstown: PA] => (40.2454, -75.6496),
    [Union_Town: PA] => (39.8979, -79.7216),

    // Mid_West
    [Terre_Haute: IN] => (39.4688, -87.3895),
    [West_Lafayette: IN] => (40.4259, -86.9081),
    [Perrysburg: OH] => (41.5567, -83.6272),
    [Muncie: IN] => (40.1934, -85.3864),
    [Fremont: OH] => (41.3503, -83.1219),
    [Findlay: OH] => (41.0442, -83.6499),
    [Youngstown: OH] => (41.0998, -80.6495),
    [Akron: OH] => (41.0814, -81.5190),
    [South_Bend: IN] => (41.6764, -86.2690),
    [Shipshewana: IN] => (41.6675, -85.5820),
    [Fort_Wayne: IN] => (41.0793, -85.1394),
    [Kenosha: WI] => (42.5847, -87.8212),
    [Madison: WI] => (43.0731, -89.4012),
    [Green_Bay: WI] => (44.5133, -88.0133),
    [Eau_Claire: WI] => (44.8113, -91.4985),
    [Wisconsin_Dells: WI] => (43.6275, -89.7711),
    [La_Crosse: WI] => (43.8014, -91.2396),
    [New_Philadelphia: OH] => (40.4898, -81.4457),
    [Effingham: IL] => (39.1200, -88.5434),
    [Champaign: IL] => (40.1164, -88.2434),
    [Dayton: OH] => (39.7589, -84.1916),
    [Columbus: IN] => (39.2014, -85.9214),
    [Vincennes: IN] => (38.6773, -87.5286),
    [Chillicothe: OH] => (39.3331, -82.9824),
    [Winchester: OH] => (38.1947, -83.6508),
    [Maysville: KY] => (38.6412, -83.7449),
    [Logansport: IN] => (40.7544, -86.3564),
    [Argos: IN] => (41.2347, -86.2500),
    [Ligonier: IN] => (41.4641, -85.5920),
    [Centralia: IL] => (38.5253, -89.1338),
    [Arcola: IL] => (39.6828, -88.3059),
    [Springfield: IL] => (39.7817, -89.6501),
    [Bloomington: IL] => (40.4842, -88.9937),
    [Peoria: IL] => (40.6936, -89.5880),
    [Rockford: IL] => (42.2711, -89.0937),
    [Galesburg: IL] => (40.9478, -90.3712),
    [Davenport: IA] => (41.5236, -90.5776),
    [Hayti: MO] => (36.2266, -89.7422),
    [Cape_Girardeau: MO] => (37.3059, -89.5180),
    [Piedmont: MO] => (37.1551, -90.6858),
    [Mornmouth: IL] => (40.9113, -90.6476),
    [Beardstown: IL] => (40.0120, -90.4281),
    [Mexico: MO] => (39.1698, -91.8823),
    [Union: MO] => (38.4509, -91.0155),
    [Sullivan: MO] => (38.2081, -91.1608),
    [St_Robert: MO] => (37.8225, -92.1631),
    [Boss: MO] => (37.0061, -91.0301),
    [Thayer: MO] => (36.5286, -91.5415),
    [Mansfield: MO] => (37.1134, -92.5806),
    [Bakersfield: MO] => (36.5009, -92.2715),
    
    // South_East
    [Spring_Hill: TN] => (35.7512, -86.9310),
    [Greenville: AL] => (31.8296, -86.6178),
    [West_Palm_Beach: FL] => (26.7153, -80.0534),
    [Sebring: FL] => (27.4953, -81.4410),
    [Winter_Haven: FL] => (28.0222, -81.7329),
    [Ocala: FL] => (29.1872, -82.1401),
    [Savannah: GA] => (32.0836, -81.0998),
    [Brunswick: GA] => (31.1499, -81.4915),
    [Cumberland: MD] => (39.6496, -78.7621),
    [Athens: TN] => (35.4429, -84.5930),
    [Tellico_Plains: TN] => (35.3598, -84.2810),
    [Huntington: WV] => (38.4192, -82.4452),
    [Terre_Alta: WV] => (39.4436, -79.5401),
    [Clarksburg: WV] => (39.2807, -80.3445),
    [Parkersburg: WV] => (39.2667, -81.5615),
    [Rome: GA] => (34.2564, -85.1647),
    [Ellijay: GA] => (34.6945, -84.4814),
    [Tallahassee: FL] => (30.4383, -84.2807),
    [Cedar_Key: FL] => (29.1369, -83.0377),
    [Lee: FL] => (30.3097, -83.3184),
    [Willacoochee: GA] => (31.4242, -83.0440),
    [Perry: GA] => (32.464940, -83.7316),
    [Arabi: GA] => (31.9270, -83.5414),
    [Moultrie: GA] => (31.1794, -83.7890),
    [Radium_Springs: GA] => (31.5449, -84.1824),
    [Roanoke: AL] => (32.8866, -85.3718),
    [Emporia: VA] => (36.6869, -77.5425),
    [Greenville: NC] => (35.6127, -77.3664),
    [Dunn: NC] => (35.3060, -78.6087),
    [Fayetteville: NC] => (35.0527, -78.8784),
    [Enfield: NC] => (36.1735, -77.6602),
    [Raleigh: NC] => (35.7804, -78.6391),
    [Warrenton: NC] => (36.3942, -78.1672),
    [Sanford: NC] => (35.4799, -79.1803),
    [Biscoe: NC] => (35.3553, -79.7779),
    [Conway: SC] => (33.8346, -79.0520),
    [Manning: SC] => (33.6960, -80.2100),
    [Montgomery: AL] => (32.3668, -86.2999),
    [Spring_Hill: FL] => (28.4851, -82.5361),
    [Evergreen: AL] => (31.4180, -86.9512),
    [Culpeper: VA] => (38.4739, -77.9968),
    [Bremen: GA] => (33.7245, -85.1442),
    [Blountstown: FL] => (30.4435, -85.0456),
    [Bonifay: FL] => (30.7816, -85.6800),
    [Niceville: FL] => (30.5169, -86.4814),
    [Dothan: AL] => (31.2232, -85.3905),
    [Midway: AL] => (32.0771, -85.5222),
    [Luverne: AL] => (31.7146, -86.2676),
    [Lexington: VA] => (37.7823, -79.4430),
    [Lexington: KY] => (38.0406, -84.5037),
    [Morehead: KY] => (38.1831, -83.4325),
    [Williamsburg: KY] => (36.7393, -84.1594),
    [Mt_Vernon: KY] => (37.3683, -84.3153),
    [Charlottesville: VA] => (38.0293, -78.4767),
    [Nitro: WV] => (38.4186, -81.8347),
    [Beaver: WV] => (37.8190, -81.1882),
    [Lexington: NC] => (35.8240, -80.2534),
    [Chesterfield: SC] => (34.7322, -80.0818),
    [Columbia: SC] => (34.0007, -81.0348),
    [Augusta: GA] => (33.4735, -81.9755),
    [Siloam: GA] => (33.6631, -83.4193),
    [Spartanburg: SC] => (34.9496, -81.9320),
    [Clemson: SC] => (34.6834, -82.8374),
    [Commerce: GA] => (34.2039, -83.4727),
    [Roanoke: VA] => (37.2710, -79.9414),
    [Brookneal: VA] => (37.0580, -78.9417),
    [Greensboro: NC] => (36.0726, -79.7919),
    [Bassett: VA] => (36.7573, -79.9981),
    [Wytheville: VA] => (36.9487, -81.0848),
    [Marion: VA] => (36.8354, -81.4829),
    [Bristol: VA] => (36.5951, -82.1887),
    [Mosheim: TN] => (36.1879, -83.0591),
    [Grundy: VA] => (37.2763, -82.1060),
    [Paintsville: KY] => (37.8150, -82.8070),

    // South_West
    [Irvine: CA] => (33.6846, -117.8265),
    [San_Bernardino: CA] => (34.1083, -117.2898),
    [Helper: UT] => (39.6886, -110.8562),
    [Green_River: UT] => (38.9962, -110.1593),
    [Fruita: CO] => (39.1582, -108.7286),
    [Gold_Run: CA] => (39.2333, -120.8434),
    [Herlong: CA] => (40.1608, -120.1450),
    [Chester: CA] => (40.2987, -121.2384),
    [Paradise: CA] => (39.7596, -121.6219),
    [Santa_Rosa: CA] => (38.4405, -122.7141),
    [Brawley: CA] => (32.9787, -115.5303),
    [Yuma: AZ] => (32.6927, -114.6277),
    [Aztec: NM] => (36.8226, -107.9917),
    [Crescent_City: CA] => (41.7559, -124.2017),
    [Fortuna: CA] => (40.5982, -124.1573),
    [San_Jose: CA] => (37.3541, -121.9552),
    [Santa_Barbara: CA] => (34.4208, -119.6982),
    [Santa_Maria: CA] => (34.9530, -120.4357),
    [San_Simeon: CA] => (35.6433, -121.1893),
    [Big_Sur: CA] => (36.2704, -121.8081),
    [Lancaster: CA] => (34.6868, -118.1542),
    [Bakersfield: CA] => (35.3733, -119.0187),
    [Modesto: CA] => (37.6391, -120.9969),
    [Amboy: CA] => (34.5574, -115.7467),
    [Nephi: UT] => (39.7107, -111.8358),
    [Mesquite: NV] => (36.8065, -114.1326),
    [Enoch: UT] => (37.8419, -113.0996),
    [Fillmore: UT] => (38.9667, -112.3298),
    [Deming: NM] => (32.2687, -107.7586),
    [Mariana: AZ] => (32.6953, -113.5913),
    [Lordsburg: NM] => (32.3500, -108.7153),
    [Pomerene: AZ] => (31.8958, -110.3439),
    [Sulphur: NV] => (40.87, -118.74),
    [Love_Lock: NV] => (40.1773, -118.4761),
    [Beowawe: NV] => (40.6349, -116.4157),
    [Wells: NV] => (41.1144, -114.9819),
    [Indio: CA] => (33.7206, -116.2156),
    [Blythe: CA] => (33.6178, -114.5889),
    [Kingman: AZ] => (35.1894, -114.0530),
    [Prescott: AZ] => (34.5400, -112.4677),
    [Wickenburg: AZ] => (33.9672, -112.7337),
    [Marfa: TX] => (30.3092, -104.0204),
    [Sierra_Blanca: TX] => (31.1784, -105.3506),
    [Toyah: TX] => (31.2312, -103.8765),
    [Santa_Rosa: NM] => (34.9382, -104.4418),
    [Albuquerque: NM] => (35.0844, -106.6504),
    [Socorro: NM] => (34.0584, -106.8908),
    [Arrey: NM] => (32.9013, -107.2362),
    [Ancho: NM] => (34.0499, -106.4666),
    [Alamogordo: NM] => (32.8995, -105.9603),
    [Grants: NM] => (35.1478, -107.8514),
    [Church_Rock: NM] => (35.4615, -108.7606),
    [Sedona: AZ] => (34.8697, -111.7601),
    [Taylor: AZ] => (34.4495, -110.1074),
    [Springer: NM] => (36.3692, -104.5965),
    [Watrous: NM] => (35.8034, -104.9221),
    [Clovis: NM] => (34.4048, -103.2052),
    [Fresno: CA] => (36.7468, -119.7726),
    [Tulare: CA] => (36.2077, -119.3473),

    // North_West
    [Custer: SD] => (43.7342, -103.6056),
    [Cheyenne: WY] => (41.1399, -104.8202),
    [Hardin: MT] => (45.7329, -107.6173),
    [Sheridan: WY] => (44.7970, -106.9562),
    [Kaycee: WY] => (43.7012, -106.6301),
    [Big_Timber: MT] => (45.8316, -109.9632),
    [Bozeman: MT] => (45.6769, -111.0429),
    [Townsend: MT] => (46.3346, -111.5079),
    [Provo: UT] => (40.2338, -111.6585),
    [Salem: OR] => (44.9429, -123.0351),
    [Eugene: OR] => (44.0521, -123.0868),
    [Grants_Pass: OR] => (42.4390, -123.3284),
    [Roseburg: OR] => (43.2165, -123.3417),
    [Declo: ID] => (42.5365, -113.6196),
    [Twin_Falls: ID] => (42.5464, -114.4609),
    [Biggs_Junction: OR] => (45.6929, -120.7046),
    [Pendleton: OR] => (45.6721, -118.7873),
    [Mountain_Home: ID] => (43.1316, -115.6860),
    [Boise: ID] => (43.6150, -116.2023),
    [Baker_City: OR] => (44.7749, -117.8344),
    [Huntington: OR] => (44.3692, -117.2121),
    [Harlem: MT] => (48.7945, -108.7585),
    [Havre: MT] => (48.5494, -109.6782),
    [Sandpoint: ID] => (48.2766, -116.5534),
    [Whitefish: MT] => (48.4111, -114.3376),
    [Chester: MT] => (48.5075, -110.9634),
    [Shelby: MT] => (48.5132, -111.8616),
    [Culbertson: MT] => (48.1480, -104.5090),
    [Wolf_Point: MT] => (48.0865, -105.6429),
    [Glasgow: MT] => (48.1970, -106.6355),
    [Malta: MT] => (48.3544, -107.8726),
    [Wilbur: WA] => (47.7426, -118.6984),
    [Marblemount: WA] => (48.5154, -121.3699),
    [Olympia: WA] => (47.0379, -122.9007),
    [Wapato: WA] => (46.4472, -120.4164),
    [Bristol: WA] => (48.8897, -118.6567),
    [Ritzville: WA] => (47.1221, -118.3785),
    [Colfax: WA] => (46.8885, -117.3637),
    [Klamath_Falls: OR] => (42.2249, -121.7817),
    [Chemult: OR] => (43.6702, -121.7867),
    [Seven_Mile: OR] => (44.4319, -123.2957),
    [Shaniko: OR] => (44.9975, -120.7530),
    [Idaho_Falls: ID] => (43.4916, -112.0340),
    [Dillon: MT] => (45.2166, -112.6371),
    [Monida: MT] => (44.5450, -111.8854),
    [Ogden: UT] => (41.2230, -111.9738),
    [Malad_City: ID] => (42.1940, -112.2511),
    [Montpelier: ID] => (42.3222, -111.2933),
    [Neihart: MT] => (46.9169, -110.6932),
    [Geraldine: MT] => (47.6011, -110.2647),
    [Forsyth: MT] => (46.2741, -106.6833),
    [Roundup: MT] => (46.4472, -108.5379),
    [Harlowton: MT] => (46.4261, -109.8369),
    [West_Wendover: NV] => (40.7391, -114.0733),
    [Baker: NV] => (39.0158, -114.0220),
    [Missoula: MT] => (46.8721, -113.9940),
    [Noxon: MT] => (48.1813, -115.7514),
    [Thompson_Falls: MT] => (47.5960, -115.3484),
    [St_Regis: MT] => (47.2993, -115.1003),
    [Cataldo: ID] => (47.5465, -116.2903),
    [Laramie: WY] => (41.3114, -105.5908),
    [Elk_Mountain: WY] => (41.6964, -106.4131),
    [Creston: MT] => (48.6649, -113.8603),
    [Evanston: WY] => (41.2684, -110.9636),
    [Little_America: WY] => (41.5878, -109.1760),
    [Point_Of_Rocks: WY] => (41.6672, -108.7713),
    [Bloomfield: MT] => (45.7947, -108.9869),
    [Terry: MT] => (46.7913, -105.3140),
    [Willbaux: MT] => (48.8505, -104.5248),
    [Orin: WY] => (42.6058, -105.8879),
    [Edgemont: SD] => (43.2942, -103.8161),
    [Wheatland: WY] => (42.0577, -104.9509),
    [Torrington: WY] => (42.0653, -104.1828),
    [Scottsbluff: NE] => (41.8666, -103.6677),
    [Baker: MT] => (46.3579, -104.2784),

    // South_Central
    [Longview: TX] => (32.5007, -94.7405),
    [Lindale: TX] => (32.5219, -95.4100),
    [Gulfport: MS] => (30.3674, -89.0928),
    [Brinkley: AR] => (34.8881, -91.1907),
    [Tullahoma: TN] => (35.3620, -86.2094),
    [Elizabethtown: KY] => (37.7035, -85.8649),
    [Bowling_Green: KY] => (36.9903, -86.4436),
    [Sealy: TX] => (29.7681, -96.1539),
    [Flatonia: TX] => (29.6858, -97.1110),
    [Camden: TN] => (36.0589, -88.0978),
    [Brownsville: TN] => (35.5934, -89.2641),
    [Cullman: AL] => (34.1743, -86.8445),
    [Lewisburg: TN] => (35.4495, -86.7889),
    [Pleasant_Hill: TN] => (35.9765, -85.1939),
    [Moulton: AL] => (34.4812, -87.2932),
    [Corinth: MS] => (34.9346, -88.5224),
    [Double_Springs: AL] => (34.1460, -87.4064),
    [Amory: MS] => (33.9844, -88.4889),
    [Windham_Springs: AL] => (33.5536, -87.5816),
    [Comstock: TX] => (29.5526, -101.1952),
    [Hondo: TX] => (29.3482, -99.1447),
    [Sanderson: TX] => (30.1382, -102.3950),
    [Ranger: TX] => (32.4743, -98.6822),
    [Big_Spring: TX] => (32.2504, -101.4787),
    [Odessa: TX] => (31.8457, -102.3676),
    [Mt_Ida: AR] => (34.5568, -93.6153),
    [Houma: LA] => (29.5958, -90.7195),
    [Lafayette: LA] => (30.2241, -92.0198),
    [Beaumont: TX] => (30.0802, -94.1266),
    [Nacogdoches: TX] => (31.6035, -94.6555),
    [Livingston: TX] => (30.7116, -94.9322),
    [Baton_Rouge: LA] => (30.4515, -91.1871),
    [Alexandria: LA] => (31.3113, -92.4451),
    [Natchitoches: LA] => (31.7607, -93.0863),
    [Ashdown: AR] => (33.6750, -94.1370),
    [Arkadelphia: AR] => (34.1209, -93.0538),
    [Meridian: MS] => (32.3643, -88.7037),
    [Starkville: MS] => (33.4504, -88.8184),
    [Hattiesburg: MS] => (31.3271, -89.2903),
    [Franklinton: LA] => (30.8471, -90.1529),
    [Hillsboro: TX] => (32.0062, -97.1304),
    [Waco: TX] => (31.5493, -97.1467),
    [College_Station: TX] => (30.6279, -96.3344),
    [Austin: TX] => (30.2672, -97.7431),
    [Sweetwater: TX] => (32.4704, -100.4064),
    [Abilene: TX] => (32.4487, -99.7331),
    [Comanche: TX] => (31.8974, -98.6068),
    [Amarillo: TX] => (35.2220, -101.8313),
    [Dalhart: TX] => (36.0609, -102.5189),
    [Lubbock: TX] => (33.5779, -101.8552),
    [Lamesa: TX] => (32.7374, -101.9514),
    [Fayetteville: AR] => (36.0822, -94.1719),
    [Heber_Springs: AR] => (35.4976, -92.0511),
    [Cave_City: AR] => (35.9511, -91.5607),
    [Yellville: AR] => (36.2277, -92.6859),
    [Rocky_Comfort: AR] => (36.2441, -94.4111),
    [Jackson: MS] => (32.2988, -90.1848),
    [Tallulah: LA] => (32.4074, -91.1949),
    [Ruston: LA] => (32.5232, -92.6379),
    [Rayville: LA] => (32.4776, -91.7520),
    [Brookhaven: MS] => (31.5795, -90.4402),
    [Amite_City: LA] => (30.7272, -90.5040),
    [Clarksdale: MS] => (34.2003, -90.5724),
    [Winona: MS] => (33.4957, -89.7310),
    [Cedar_Bluff: MS] => (33.7765, -88.1678),
    [Hopkinsville: KY] => (36.8656, -87.4886),
    [Owensboro: KY] => (37.7749, -87.1134),
    [Benton: KY] => (36.8588, -88.3621),
    [McDonald: TN] => (35.3716, -84.6369),
    [Wichita_Falls: TX] => (33.9137, -98.4934),
    [Quanah: TX] => (34.2876, -99.7421),
    [Hedley: TX] => (34.8241, -100.6586),
    [McLean: TX] => (35.2263, -100.5950),
    [Paris: TX] => (33.6609, -95.5555),
    [Kerens: TX] => (32.2289, -96.3236),
    [Madisonville: KY] => (37.3280, -87.4989),

    // Plains
    [Dubuque: IA] => (42.5006, -90.6646),
    [Sioux_Falls: SD] => (43.5446, -96.7311),
    [Chamberlain: SD] => (43.7878, -99.3207),
    [Kadoka: SD] => (43.8361, -101.5144),
    [Blue_Earth: MN] => (43.6397, -94.1004),
    [Luverne: MN] => (43.6220, -96.2123),
    [Albert_Lea: MN] => (43.6480, -93.3684),
    [Jackson: MN] => (43.6200, -94.9921),
    [Colorado_Springs: CO] => (38.8339, -104.8214),
    [Nathrop: CO] => (38.7477, -106.2384),
    [Aspen: CO] => (39.1911, -106.8175),
    [Glenwood_Springs: CO] => (39.5501, -107.3248),
    [Breckenridge: CO] => (39.4807, -106.0384),
    [St_Cloud: MN] => (45.5579, -94.1632),
    [Perham: MN] => (46.5834, -95.7083),
    [Fergus_Falls: MN] => (46.2834, -96.0776),
    [Williston: ND] => (48.1461, -103.6174),
    [Stanley: ND] => (48.3152, -102.3880),
    [Minot: ND] => (48.2325, -101.2963),
    [Devils_Lake: ND] => (48.1128, -98.8651),
    [Massena: IA] => (41.4740, -94.8390),
    [St_Joseph: MO] => (39.7675, -94.8467),
    [Nebraska_City: NE] => (40.6764, -95.8588),
    [Osceola: IA] => (41.0328, -93.7733),
    [Gallatin: MO] => (39.9148, -93.9810),
    [Clear_Lake: IA] => (43.1386, -93.3713),
    [Ellsworth: KS] => (38.7439, -98.2270),
    [Evansdale: IA] => (42.4640, -92.2705),
    [Montezuma: IA] => (41.5861, -92.5273),
    [Iowa_City: IA] => (41.6611, -91.5302),
    [Exline: IA] => (40.6208, -92.5963),
    [Chillicothe: MO] => (39.7954, -93.5521),
    [Sweet_Springs: MO] => (38.9618, -93.4213),
    [Ada: OK] => (34.7745, -96.6783),
    [Talahina: OK] => (34.7504, -95.0491),
    [Thackerville: OK] => (33.8004, -97.1279),
    [Elmore_City: OK] => (34.6376, -97.3820),
    [Medora: OK] => (34.9158, -99.0049),
    [Jamestown: ND] => (46.9105, -98.7084),
    [Bismarck: ND] => (46.8083, -100.7837),
    [Antelope: MT] => (47.2722, -108.7041),
    [Topeka: KS] => (39.0473, -95.6752),
    [Wichita: KS] => (37.6872, -97.3301),
    [South_Haven: KS] => (37.0041, -97.3936),
    [Perry: OK] => (36.2894, -97.2881),
    [Trinidad: CO] => (37.1690, -104.5005),
    [Pritchett: CO] => (37.2821, -102.9999),
    [Las_Animas: CO] => (38.0668, -103.2180),
    [Johnson_City: KS] => (37.5725, -101.7439),
    [Pratt: KS] => (37.6504, -98.7193),
    [Copeland: KS] => (37.3262, -100.8327),
    [Lincoln: KS] => (39.0405, -98.1443),
    [Central_City: NE] => (41.1167, -98.0016),
    [Sioux_City: IA] => (42.5006, -96.4059),
    [Norfolk: NE] => (42.0281, -97.4171),
    [Linn: KS] => (39.8668, -96.9361),
    [Sedalia: MO] => (38.7045, -93.2283),
    [Joplin: MO] => (37.0842, -94.5133),
    [Tulsa: OK] => (36.1540, -95.9928),
    [Vinita: OK] => (36.6387, -95.1543),
    [Butler: MO] => (38.2601, -94.3307),
    [Louisburg: KS] => (38.6195, -94.6808),
    [Harrisonville: MO] => (38.6533, -94.3488),
    [Springfield: MO] => (37.2086, -93.2926),
    [Pine_Ridge: SD] => (43.0243, -102.5533),
    [Mission: SD] => (43.3038, -100.6329),
    [Atkinson: NE] => (42.5456, -98.9737),
    [Sheldon: IA] => (43.1867, -95.8560),
    [Hutchinson: KS] => (38.0608, -97.9298),
    [Appleton: WI] => (44.2619, -88.4154),
    [Webster: SD] => (45.3271, -97.5211),
    [Bowdle: SD] => (45.4535, -99.6357),
    [McIntosh: SD] => (45.8039, -101.3497),
    [Reeder: ND] => (46.1205, -102.8540),
    [Weatherford: OK] => (35.5260, -98.7076),
    [Sayre: OK] => (35.2923, -99.6410),
    [Bayard: NE] => (41.7584, -103.3243),
    [Rush_Springs: OK] => (34.7935, -97.9637),
    [Waynoka: OK] => (36.5849, -98.8877),
    [Argonia: KS] => (37.2650, -97.7752),
    [Rattan: OK] => (34.0804, -95.2412),
    [Minneola: KS] => (37.4456, -100.0154),
    [Tyrone: OK] => (36.8834, -101.0783),
    [Texhoma: OK] => (36.5146, -101.7900),
    [Emporia: KS] => (38.4039, -96.1817),
    [Syracuse: KS] => (37.9889, -101.7574),
    [Ingalls: KS] => (37.9739, -101.3352),
    [Bucklin: KS] => (37.5653, -99.6309),
    [Jetmore: KS] => (38.0854, -99.8941),
    [Herington: KS] => (38.6667, -97.1720),
    [Lyons: KS] => (38.3525, -98.1967),
    [Ogalla: NE] => (41.1339, -101.7194),
    [Raveena: NE] => (41.5386, -101.7384),
    [Red_Oak: IA] => (41.0073, -95.2247),
    [Cozad: NE] => (40.8590, -99.9880),
    [Julesburg: CO] => (40.9909, -102.2636),
    [Hastings: NE] => (40.5862, -98.3895),
    [Manhattan: KS] => (39.1948, -96.5925),
    [Limon: CO] => (39.2634, -103.6790),
    [Brush: CO] => (40.2584, -103.6324),
    [Wray: CO] => (40.0758, -102.2275),
    [Bethune: CO] => (39.3032, -102.4202),
    [Trenton: NE] => (40.1746, -100.5520),
    [Grinell: KS] => (39.1645, -101.1190),
    [Osborne: KS] => (39.4401, -98.6940),
    [Concordia: KS] => (39.5700, -97.6618),
    [Daykin: NE] => (40.1577, -97.1799),
    [Araphoe: NE] => (40.3065, -95.4294),
    [Blue_Hill: NE] => (40.3499, -98.4079),
}
