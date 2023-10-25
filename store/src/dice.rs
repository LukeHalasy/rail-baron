use std::ops::Add;

use rand::Rng;

pub use crate::main_city::City;
pub use crate::region::Region;
use crate::Engine;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Dice {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Dice {
    pub fn new() -> Dice {
        let mut rng = rand::thread_rng();

        match rng.gen_range(1..=6) {
            1 => Dice::One,
            2 => Dice::Two,
            3 => Dice::Three,
            4 => Dice::Four,
            5 => Dice::Five,
            6 => Dice::Six,
            _ => panic!(),
        }
    }

    pub fn is_odd(self) -> bool {
        (self as u8) % 2 != 0
    }
}

impl Add<Dice> for Dice {
    type Output = u8;

    fn add(self, dice: Dice) -> u8 {
        (self as u8) + (dice as u8)
    }
}

impl Add<u8> for Dice {
    type Output = u8;

    fn add(self, value: u8) -> u8 {
        (self as u8) + value
    }
}

impl Add<Dice> for u8 {
    type Output = u8;

    fn add(self, dice: Dice) -> u8 {
        self + (dice as u8)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct DiceRoll {
    white_dice: (Dice, Dice),
    red_dice: Option<Dice>,
}

impl DiceRoll {
    fn red_and_white() -> Self {
        Self {
            white_dice: (Dice::new(), Dice::new()),
            red_dice: Some(Dice::new()),
        }
    }

    fn white() -> Self {
        Self {
            white_dice: (Dice::new(), Dice::new()),
            red_dice: None,
        }
    }

    pub fn sum(self) -> u8 {
        if let Some(red_dice) = self.red_dice {
            red_dice + self.white_dice.0 + self.white_dice.1
        } else {
            self.white_dice.0 + self.white_dice.1
        }
    }

    pub fn movement_roll(train: &Engine) -> Self {
        match train {
            Engine::Freight => {
                let mut roll = Self::white();

                if roll.white_dice.0 == Dice::Six && roll.white_dice.1 == Dice::Six {
                    roll.red_dice = Some(Dice::new());
                }

                roll
            }
            Engine::Express => {
                let mut roll = Self::white();

                if roll.white_dice.0 == roll.white_dice.1 {
                    roll.red_dice = Some(Dice::new());
                }

                roll
            }
            Engine::SuperChief => Self::red_and_white(),
        }
    }
}

impl DiceRoll {
    pub fn region() -> (Self, Region) {
        let dice_roll: Self = Self::red_and_white();
        let red_dice = dice_roll.red_dice.unwrap();
        let white_dice = dice_roll.white_dice;

        let region_match = if red_dice.is_odd() {
            match white_dice.0 + white_dice.1 {
                2 => Region::Plains,
                3 => Region::South_East,
                4 => Region::South_East,
                5 => Region::South_East,
                6 => Region::North_Central,
                7 => Region::North_Central,
                8 => Region::North_East,
                9 => Region::North_East,
                10 => Region::North_East,
                11 => Region::North_East,
                12 => Region::North_East,
                _ => panic!("White dice sum > 12 || < 2"),
            }
        } else {
            match white_dice.0 + white_dice.1 {
                2 => Region::South_West,
                3 => Region::South_Central,
                4 => Region::South_Central,
                5 => Region::South_Central,
                6 => Region::South_West,
                7 => Region::South_West,
                8 => Region::Plains,
                9 => Region::North_West,
                10 => Region::North_West,
                11 => Region::Plains,
                12 => Region::North_West,
                _ => panic!("White dice sum > 12 || < 2"),
            }
        };

        (dice_roll, region_match)
    }
}

macro_rules! city_from_dice_roll {
    ($($region:tt => {$($sum:literal: ($c_odd:tt, $c_even:tt)),*$(,)?}),*$(,)?) => {
        paste::paste! {
            impl DiceRoll {
                pub fn city_in_region(region: Region) -> (Self, City) {
                    let dice_roll: Self = Self::red_and_white();
                    let red_dice = dice_roll.red_dice.unwrap();
                    let white_dice = dice_roll.white_dice;

                    let city_match = match region {
                        $(Region::$region => {
                            match (white_dice.0 + white_dice.1) {
                                $($sum => {
                                    if red_dice.is_odd() {
                                        City::$c_odd
                                    } else {
                                        City::$c_even
                                    }
                                }),*
                                _ => panic!("White dice sum (two dice) < 2 or > 12")
                            }
                        }),*
                    };

                    (dice_roll, city_match)
                }
            }
        }
    };
}

city_from_dice_roll! {
    North_East => {
        //  (Odd,           Even)
        2:	(New_York_NY,	New_York_NY),
        3:	(New_York_NY,	Washington_DC),
        4:	(New_York_NY,	Pittsburgh_PA),
        5:	(Albany_NY,	    Pittsburgh_PA),
        6:	(Boston_MA,	    Philadelphia_PA),
        7:	(Buffalo_NY,	Washington_DC),
        8:	(Boston_MA,	    Philadelphia_PA),
        9:	(Portland_ME,	Baltimore_MD),
        10:	(New_York_NY,	Baltimore_MD),
        11:	(New_York_NY,	Baltimore_MD),
        12:	(New_York_NY,	New_York_NY)
    },
    North_Central => {
        //  (Odd,             Even)
        2:	(Cleveland_OH,	  Cincinnati_OH),
        3:	(Cleveland_OH,	  Chicago_IL),
        4:	(Cleveland_OH,	  Cincinnati_OH),
        5:	(Cleveland_OH,	  Cincinnati_OH),
        6:	(Detroit_MI,	  Columbus_OH),
        7:	(Detroit_MI,	  Chicago_IL),
        8:	(Indianapolis_IN, Chicago_IL),
        9:	(Milwaukee_WI,	  St_Louis_MO),
        10:	(Milwaukee_WI,	  St_Louis_MO),
        11:	(Chicago_IL,	  St_Louis_MO),
        12:	(Milwaukee_WI,	  Chicago_IL)
    },
    South_East => {
        //  (Odd,            Even)
        2:	(Charlotte_TN,	 Norfolk_VA),
        3:	(Charlotte_TN,	 Norfolk_VA),
        4:	(Chattanooga_TN, Norfolk_VA),
        5:	(Atlanta_GA,	 Charleston_SC),
        6:	(Atlanta_GA,	 Miami_FL),
        7:	(Atlanta_GA,	 Jacksonville_FL),
        8:	(Richmond_VA,	 Miami_FL),
        9:	(Knoxville_TN,	 Tampa_FL),
        10:	(Mobile_AL,	     Tampa_FL),
        11:	(Knoxville_TN,	 Mobile_AL),
        12:	(Mobile_AL,	     Norfolk_VA),
    },
    South_Central => {
        //  (Odd,            Even)
        2:	(Memphis_TN,	 Shreveport_LA),
        3:	(Memphis_TN,	 Shreveport_LA),
        4:	(Memphis_TN,	 Dallas_TX),
        5:	(Little_Rock_AK, New_Orleans_LA),
        6:	(New_Orleans_LA, Dallas_TX),
        7:	(Birmingham_AL,	 San_Antonio_TX),
        8:	(Louisville_KY,	 Houston_TX),
        9:	(Nashville_TN,	 Houston_TX),
        10:	(Nashville_TN,	 Fort_Worth_TX),
        11:	(Louisville_KY,	 Fort_Worth_TX),
        12:	(Memphis_TN,	 Fort_Worth_TX),
    },
    Plains => {
        //  (Odd,              Even)
        2:	(Kansas_City_MO,   Oklahoma_City_OK),
        3:	(Kansas_City_MO,   Minneapolis_MN),
        4:	(Denver_CO,	       Minneapolis_MN),
        5:	(Denver_CO,	       Minneapolis_MN),
        6:	(Denver_CO,	       Minneapolis_MN),
        7:	(Kansas_City_MO,   Oklahoma_City_OK),
        8:	(Kansas_City_MO,   Des_Moines_IA),
        9:	(Kansas_City_MO,   Omaha_NE),
        10:	(Pueblo_CO,	       Omaha_NE),
        11:	(Pueblo_CO,	       Fargo_ND),
        12:	(Oklahoma_City_OK, Fargo_ND),
    },
    North_West => {
        //  (Odd,          Even)
        2:	(Spokane_WA,	Spokane_WA),
        3:	(Spokane_WA,	Salt_Lake_City_UT),
        4:	(Seattle_WA,	Salt_Lake_City_UT),
        5:	(Seattle_WA,	Salt_Lake_City_UT),
        6:	(Seattle_WA,	Portland_OR),
        7:	(Seattle_WA,	Portland_OR),
        8:	(Rapid_City_SD,	Portland_OR),
        9:	(Casper_WY,	    Pocatello_ID),
        10:	(Billings_MT,	Butte_MT),
        11:	(Billings_MT,	Butte_MT),
        12:	(Spokane_WA,	Portland_OR),
    },
    South_West => {
        //  (Odd,           Even)
        2:	(San_Diego_CA,	Los_Angeles_CA),
        3:	(San_Diego_CA,	San_Francisco_CA),
        4:	(Reno_NV,	San_Francisco_CA),
        5:	(San_Diego_CA,	San_Francisco_CA),
        6:	(Sacramento_CA,	Los_Angeles_CA),
        7:	(Las_Vegas_NV,	Los_Angeles_CA),
        8:	(Phoenix_AZ,	Los_Angeles_CA),
        9:	(El_Paso_TX,	San_Francisco_CA),
        10:	(Tucumcari_NM,	San_Francisco_CA),
        11:	(Phoenix_AZ,	San_Francisco_CA),
        12:	(Phoenix_AZ,	San_Francisco_CA),
    },
}
