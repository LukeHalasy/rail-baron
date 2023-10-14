use crate::deed::Deed;

pub use crate::city::City;
pub use crate::state::State;

use serde::{Deserialize, Serialize};

macro_rules! sub_cities {
    ($($c:tt: $s:tt => [$($nc:tt: [$($d:tt),*$(,)?]),*$(,)?] => [$($nsc:tt: [$($sd:tt),*$(,)?]),*$(,)?]),*$(,)?) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
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

                pub const fn neighbor_cities(&self) -> &[(City, &[Deed])] {
                    match self {
                        $(Self::[<$c _ $s>] => {
                            &[$((
                                City::$nc,
                                &[$(Deed::$d),*],
                            )),*]
                        }),*
                    }
                }

                pub const fn neighbor_sub_cities(&self) -> &[(Self, &[Deed])] {
                    match self {
                        $(Self::[<$c _ $s>] => {
                            &[$((
                                Self::$nsc,
                                &[$(Deed::$sd),*],
                            )),*]
                        }),*
                    }
                }
            }
        }
    }
}

sub_cities! {
    // North East
    Concord: NH => [Boston_MA: [B_AND_M], Portland_ME: [B_AND_M]] => [Springfield_MA: [B_AND_M]],
    Springfield: MA => [Albany_NY: [B_AND_M], Boston_MA: [B_AND_M]] => [Concord_NH: [B_AND_M]],
    Providence: RI => [Boston_MA: [NYNH_AND_H]] => [New_Haven_CT: [NYNH_AND_H]],
    New_Haven: CT => [New_York_NY: [NYNH_AND_H]] => [Providence_RI: [NYNH_AND_H]],
    Kingston: NY => [Albany_NY: [NYC], New_York_NY: [NYC]] => [],
    Syracuse: NY => [Albany_NY: [NYC]] => [Rochester_NY: [NYC]],
    Rochester: NY => [Buffalo_NY: [NYC]] => [Syracuse_NY: [NYC]],
    Erie: PA => [Buffalo_NY: [NYC, PA], Cleveland_OH: [NYC]] => [Youngstown_OH: [PA]],
    Trenton: NJ => [Philadelphia_PA: [PA], New_York_NY: [PA]] => [],
    Pottstown: PA => [Boston_MA: [PA], Philadelphia_PA: [PA]] => [Lancaster_PA: [PA]],
    Lancaster: PA => [] => [Pottstown_PA: [PA], Bedford_PA: [PA]],
    Bedford: PA => [Pittsburgh_PA: [PA]] => [Lancaster_PA: [PA]],
    Uniontown: PA => [Pittsburgh_PA: [B_AND_O]] => [Cumberland_MD: [B_AND_O]],
    Frederick: MD => [Baltimore_MD: [B_AND_O], Washington_DC: [B_AND_O]] => [Cumberland_MD: [B_AND_O]],


    // Canada :)
    Brantford: ON => [Buffalo_NY: [C_AND_O]] => [London_ON: [C_AND_O]],
    London: ON => [Detroit_MI: [C_AND_O]] => [Brantford_ON: [C_AND_O]],


    // Mid-West



}
