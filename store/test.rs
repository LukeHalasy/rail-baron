#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
use std::fmt;
type PlayerId = u64;
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}
#[automatically_derived]
impl ::core::fmt::Debug for Stage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Stage::PreGame => "PreGame",
                Stage::InGame => "InGame",
                Stage::Ended => "Ended",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Stage {
    #[inline]
    fn clone(&self) -> Stage {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Stage {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Stage {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Stage {
    #[inline]
    fn eq(&self, other: &Stage) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Stage {}
#[automatically_derived]
impl ::core::cmp::Eq for Stage {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for Stage {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Stage {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Stage::PreGame => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "Stage",
                    0u32,
                    "PreGame",
                ),
                Stage::InGame => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "Stage",
                    1u32,
                    "InGame",
                ),
                Stage::Ended => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Stage", 2u32, "Ended")
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Stage {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "PreGame" => _serde::__private::Ok(__Field::__field0),
                        "InGame" => _serde::__private::Ok(__Field::__field1),
                        "Ended" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"PreGame" => _serde::__private::Ok(__Field::__field0),
                        b"InGame" => _serde::__private::Ok(__Field::__field1),
                        b"Ended" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Stage>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Stage;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Stage")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Stage::PreGame)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Stage::InGame)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Stage::Ended)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["PreGame", "InGame", "Ended"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Stage",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Stage>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub enum Piece {
    Yellow,
    Green,
    Blue,
    White,
    Black,
    Red,
}
#[automatically_derived]
impl ::core::fmt::Debug for Piece {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Piece::Yellow => "Yellow",
                Piece::Green => "Green",
                Piece::Blue => "Blue",
                Piece::White => "White",
                Piece::Black => "Black",
                Piece::Red => "Red",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Piece {
    #[inline]
    fn clone(&self) -> Piece {
        match self {
            Piece::Yellow => Piece::Yellow,
            Piece::Green => Piece::Green,
            Piece::Blue => Piece::Blue,
            Piece::White => Piece::White,
            Piece::Black => Piece::Black,
            Piece::Red => Piece::Red,
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Piece {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Piece {
    #[inline]
    fn eq(&self, other: &Piece) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Piece {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Piece::Yellow => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "Piece",
                    0u32,
                    "Yellow",
                ),
                Piece::Green => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Piece", 1u32, "Green")
                }
                Piece::Blue => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Piece", 2u32, "Blue")
                }
                Piece::White => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Piece", 3u32, "White")
                }
                Piece::Black => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Piece", 4u32, "Black")
                }
                Piece::Red => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "Piece", 5u32, "Red")
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Piece {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 6",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Yellow" => _serde::__private::Ok(__Field::__field0),
                        "Green" => _serde::__private::Ok(__Field::__field1),
                        "Blue" => _serde::__private::Ok(__Field::__field2),
                        "White" => _serde::__private::Ok(__Field::__field3),
                        "Black" => _serde::__private::Ok(__Field::__field4),
                        "Red" => _serde::__private::Ok(__Field::__field5),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Yellow" => _serde::__private::Ok(__Field::__field0),
                        b"Green" => _serde::__private::Ok(__Field::__field1),
                        b"Blue" => _serde::__private::Ok(__Field::__field2),
                        b"White" => _serde::__private::Ok(__Field::__field3),
                        b"Black" => _serde::__private::Ok(__Field::__field4),
                        b"Red" => _serde::__private::Ok(__Field::__field5),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Piece>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Piece;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Piece")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::Yellow)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::Green)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::Blue)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::White)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::Black)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Piece::Red)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] =
                &["Yellow", "Green", "Blue", "White", "Black", "Red"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Piece",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Piece>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub enum Region {
    NorthEast,
    NorthCentral,
    SouthEast,
    SouthCentral,
    Plains,
    NorthWest,
    SouthWest,
}
#[automatically_derived]
impl ::core::clone::Clone for Region {
    #[inline]
    fn clone(&self) -> Region {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Region {}
#[automatically_derived]
impl ::core::fmt::Debug for Region {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Region::NorthEast => "NorthEast",
                Region::NorthCentral => "NorthCentral",
                Region::SouthEast => "SouthEast",
                Region::SouthCentral => "SouthCentral",
                Region::Plains => "Plains",
                Region::NorthWest => "NorthWest",
                Region::SouthWest => "SouthWest",
            },
        )
    }
}
impl Region {
    pub const fn regions() -> &'static [Self] {
        &[
            Self::NorthEast,
            Self::NorthCentral,
            Self::SouthEast,
            Self::SouthCentral,
            Self::Plains,
            Self::NorthWest,
            Self::SouthWest,
        ]
    }
    pub const fn cities(&self) -> &[City] {
        match self {
            Self::NorthEast => &[
                City::NorthEast(northeast::City::Albany),
                City::NorthEast(northeast::City::Baltimore),
                City::NorthEast(northeast::City::Boston),
                City::NorthEast(northeast::City::Buffalo),
                City::NorthEast(northeast::City::NewYork),
                City::NorthEast(northeast::City::Pittsburgh),
                City::NorthEast(northeast::City::Philadalphia),
                City::NorthEast(northeast::City::Portland),
                City::NorthEast(northeast::City::Washington),
            ],
            Self::NorthCentral => &[
                City::NorthCentral(northcentral::City::Chicago),
                City::NorthCentral(northcentral::City::Cincinnati),
                City::NorthCentral(northcentral::City::Cleveland),
                City::NorthCentral(northcentral::City::Columbus),
                City::NorthCentral(northcentral::City::Detroit),
                City::NorthCentral(northcentral::City::Indianapolis),
                City::NorthCentral(northcentral::City::Milwaukee),
                City::NorthCentral(northcentral::City::StLouis),
            ],
            Self::SouthEast => &[
                City::SouthEast(southeast::City::Charlotte),
                City::SouthEast(southeast::City::Norfolk),
                City::SouthEast(southeast::City::Chattanooga),
                City::SouthEast(southeast::City::Atlanta),
                City::SouthEast(southeast::City::Charleston),
                City::SouthEast(southeast::City::Miami),
                City::SouthEast(southeast::City::Jacksonville),
                City::SouthEast(southeast::City::Richmond),
                City::SouthEast(southeast::City::Knoxville),
                City::SouthEast(southeast::City::Tampa),
                City::SouthEast(southeast::City::Mobile),
            ],
            Self::SouthCentral => &[
                City::SouthCentral(southcentral::City::Birmingham),
                City::SouthCentral(southcentral::City::Dallas),
                City::SouthCentral(southcentral::City::FortWorth),
                City::SouthCentral(southcentral::City::Houston),
                City::SouthCentral(southcentral::City::LittleRock),
                City::SouthCentral(southcentral::City::Louisville),
                City::SouthCentral(southcentral::City::Memphis),
                City::SouthCentral(southcentral::City::Nashville),
                City::SouthCentral(southcentral::City::NewOrleans),
                City::SouthCentral(southcentral::City::SanAntonio),
                City::SouthCentral(southcentral::City::Shreveport),
            ],
            Self::Plains => &[
                City::Plains(plains::City::KansasCity),
                City::Plains(plains::City::OklahomaCity),
                City::Plains(plains::City::StPaul),
                City::Plains(plains::City::Denver),
                City::Plains(plains::City::Minneapolis),
                City::Plains(plains::City::DesMoines),
                City::Plains(plains::City::Omaha),
                City::Plains(plains::City::Pueblo),
                City::Plains(plains::City::Fargo),
            ],
            Self::NorthWest => &[
                City::NorthWest(northwest::City::Spokane),
                City::NorthWest(northwest::City::SaltLakeCity),
                City::NorthWest(northwest::City::Seattle),
                City::NorthWest(northwest::City::Portland),
                City::NorthWest(northwest::City::RapidCity),
                City::NorthWest(northwest::City::Casper),
                City::NorthWest(northwest::City::Pocatello),
                City::NorthWest(northwest::City::Billings),
                City::NorthWest(northwest::City::Butte),
            ],
            Self::SouthWest => &[
                City::SouthWest(southwest::City::SanDiego),
                City::SouthWest(southwest::City::LosAngeles),
                City::SouthWest(southwest::City::Oakland),
                City::SouthWest(southwest::City::Reno),
                City::SouthWest(southwest::City::Sacramento),
                City::SouthWest(southwest::City::LasVegas),
                City::SouthWest(southwest::City::Phoenix),
                City::SouthWest(southwest::City::ElPaso),
                City::SouthWest(southwest::City::SanFrancisco),
                City::SouthWest(southwest::City::Tucumcari),
            ],
        }
    }
}
pub enum City {
    NorthEast(northeast::City),
    NorthCentral(northcentral::City),
    SouthEast(southeast::City),
    SouthCentral(southcentral::City),
    Plains(plains::City),
    NorthWest(northwest::City),
    SouthWest(southwest::City),
}
#[automatically_derived]
impl ::core::clone::Clone for City {
    #[inline]
    fn clone(&self) -> City {
        let _: ::core::clone::AssertParamIsClone<northeast::City>;
        let _: ::core::clone::AssertParamIsClone<northcentral::City>;
        let _: ::core::clone::AssertParamIsClone<southeast::City>;
        let _: ::core::clone::AssertParamIsClone<southcentral::City>;
        let _: ::core::clone::AssertParamIsClone<plains::City>;
        let _: ::core::clone::AssertParamIsClone<northwest::City>;
        let _: ::core::clone::AssertParamIsClone<southwest::City>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for City {}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NorthEast(city) => f.write_fmt(format_args!("{0}", city)),
            Self::NorthCentral(city) => f.write_fmt(format_args!("{0}", city)),
            Self::SouthEast(city) => f.write_fmt(format_args!("{0}", city)),
            Self::SouthCentral(city) => f.write_fmt(format_args!("{0}", city)),
            Self::Plains(city) => f.write_fmt(format_args!("{0}", city)),
            Self::NorthWest(city) => f.write_fmt(format_args!("{0}", city)),
            Self::SouthWest(city) => f.write_fmt(format_args!("{0}", city)),
        }
    }
}
pub mod northeast {
    use std::fmt;
    pub enum City {
        Albany,
        Baltimore,
        Boston,
        Buffalo,
        NewYork,
        Pittsburgh,
        Philadalphia,
        Portland,
        Washington,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::Albany => "Albany",
                    City::Baltimore => "Baltimore",
                    City::Boston => "Boston",
                    City::Buffalo => "Buffalo",
                    City::NewYork => "NewYork",
                    City::Pittsburgh => "Pittsburgh",
                    City::Philadalphia => "Philadalphia",
                    City::Portland => "Portland",
                    City::Washington => "Washington",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Albany => f.write_fmt(format_args!("albany")),
                Self::Baltimore => f.write_fmt(format_args!("baltimore")),
                Self::Boston => f.write_fmt(format_args!("boston")),
                Self::Buffalo => f.write_fmt(format_args!("buffalo")),
                Self::NewYork => f.write_fmt(format_args!("newyork")),
                Self::Pittsburgh => f.write_fmt(format_args!("pittsburgh")),
                Self::Philadalphia => f.write_fmt(format_args!("philadalphia")),
                Self::Portland => f.write_fmt(format_args!("portland")),
                Self::Washington => f.write_fmt(format_args!("washington")),
            }
        }
    }
}
pub mod northcentral {
    use std::fmt;
    pub enum City {
        Chicago,
        Cincinnati,
        Cleveland,
        Columbus,
        Detroit,
        Indianapolis,
        Milwaukee,
        StLouis,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::Chicago => "Chicago",
                    City::Cincinnati => "Cincinnati",
                    City::Cleveland => "Cleveland",
                    City::Columbus => "Columbus",
                    City::Detroit => "Detroit",
                    City::Indianapolis => "Indianapolis",
                    City::Milwaukee => "Milwaukee",
                    City::StLouis => "StLouis",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Chicago => f.write_fmt(format_args!("chicago")),
                Self::Cincinnati => f.write_fmt(format_args!("cincinnati")),
                Self::Cleveland => f.write_fmt(format_args!("cleveland")),
                Self::Columbus => f.write_fmt(format_args!("columbus")),
                Self::Detroit => f.write_fmt(format_args!("detroit")),
                Self::Indianapolis => f.write_fmt(format_args!("indianapolis")),
                Self::Milwaukee => f.write_fmt(format_args!("milwaukee")),
                Self::StLouis => f.write_fmt(format_args!("stlouis")),
            }
        }
    }
}
pub mod southeast {
    use std::fmt;
    pub enum City {
        Charlotte,
        Norfolk,
        Chattanooga,
        Atlanta,
        Charleston,
        Miami,
        Jacksonville,
        Richmond,
        Knoxville,
        Tampa,
        Mobile,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::Charlotte => "Charlotte",
                    City::Norfolk => "Norfolk",
                    City::Chattanooga => "Chattanooga",
                    City::Atlanta => "Atlanta",
                    City::Charleston => "Charleston",
                    City::Miami => "Miami",
                    City::Jacksonville => "Jacksonville",
                    City::Richmond => "Richmond",
                    City::Knoxville => "Knoxville",
                    City::Tampa => "Tampa",
                    City::Mobile => "Mobile",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charlotte => f.write_fmt(format_args!("charlotte")),
                Self::Norfolk => f.write_fmt(format_args!("norfolk")),
                Self::Chattanooga => f.write_fmt(format_args!("chattanooga")),
                Self::Atlanta => f.write_fmt(format_args!("atlanta")),
                Self::Charleston => f.write_fmt(format_args!("charleston")),
                Self::Miami => f.write_fmt(format_args!("miami")),
                Self::Jacksonville => f.write_fmt(format_args!("jacksonville")),
                Self::Richmond => f.write_fmt(format_args!("richmond")),
                Self::Knoxville => f.write_fmt(format_args!("knoxville")),
                Self::Tampa => f.write_fmt(format_args!("tampa")),
                Self::Mobile => f.write_fmt(format_args!("mobile")),
            }
        }
    }
}
pub mod southcentral {
    use std::fmt;
    pub enum City {
        Birmingham,
        Dallas,
        FortWorth,
        Houston,
        LittleRock,
        Louisville,
        Memphis,
        Nashville,
        NewOrleans,
        SanAntonio,
        Shreveport,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::Birmingham => "Birmingham",
                    City::Dallas => "Dallas",
                    City::FortWorth => "FortWorth",
                    City::Houston => "Houston",
                    City::LittleRock => "LittleRock",
                    City::Louisville => "Louisville",
                    City::Memphis => "Memphis",
                    City::Nashville => "Nashville",
                    City::NewOrleans => "NewOrleans",
                    City::SanAntonio => "SanAntonio",
                    City::Shreveport => "Shreveport",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Birmingham => f.write_fmt(format_args!("birmingham")),
                Self::Dallas => f.write_fmt(format_args!("dallas")),
                Self::FortWorth => f.write_fmt(format_args!("fortworth")),
                Self::Houston => f.write_fmt(format_args!("houston")),
                Self::LittleRock => f.write_fmt(format_args!("littlerock")),
                Self::Louisville => f.write_fmt(format_args!("louisville")),
                Self::Memphis => f.write_fmt(format_args!("memphis")),
                Self::Nashville => f.write_fmt(format_args!("nashville")),
                Self::NewOrleans => f.write_fmt(format_args!("neworleans")),
                Self::SanAntonio => f.write_fmt(format_args!("sanantonio")),
                Self::Shreveport => f.write_fmt(format_args!("shreveport")),
            }
        }
    }
}
pub mod plains {
    use std::fmt;
    pub enum City {
        KansasCity,
        OklahomaCity,
        StPaul,
        Denver,
        Minneapolis,
        DesMoines,
        Omaha,
        Pueblo,
        Fargo,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::KansasCity => "KansasCity",
                    City::OklahomaCity => "OklahomaCity",
                    City::StPaul => "StPaul",
                    City::Denver => "Denver",
                    City::Minneapolis => "Minneapolis",
                    City::DesMoines => "DesMoines",
                    City::Omaha => "Omaha",
                    City::Pueblo => "Pueblo",
                    City::Fargo => "Fargo",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::KansasCity => f.write_fmt(format_args!("kansascity")),
                Self::OklahomaCity => f.write_fmt(format_args!("oklahomacity")),
                Self::StPaul => f.write_fmt(format_args!("stpaul")),
                Self::Denver => f.write_fmt(format_args!("denver")),
                Self::Minneapolis => f.write_fmt(format_args!("minneapolis")),
                Self::DesMoines => f.write_fmt(format_args!("desmoines")),
                Self::Omaha => f.write_fmt(format_args!("omaha")),
                Self::Pueblo => f.write_fmt(format_args!("pueblo")),
                Self::Fargo => f.write_fmt(format_args!("fargo")),
            }
        }
    }
}
pub mod northwest {
    use std::fmt;
    pub enum City {
        Spokane,
        SaltLakeCity,
        Seattle,
        Portland,
        RapidCity,
        Casper,
        Pocatello,
        Billings,
        Butte,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::Spokane => "Spokane",
                    City::SaltLakeCity => "SaltLakeCity",
                    City::Seattle => "Seattle",
                    City::Portland => "Portland",
                    City::RapidCity => "RapidCity",
                    City::Casper => "Casper",
                    City::Pocatello => "Pocatello",
                    City::Billings => "Billings",
                    City::Butte => "Butte",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Spokane => f.write_fmt(format_args!("spokane")),
                Self::SaltLakeCity => f.write_fmt(format_args!("saltlakecity")),
                Self::Seattle => f.write_fmt(format_args!("seattle")),
                Self::Portland => f.write_fmt(format_args!("portland")),
                Self::RapidCity => f.write_fmt(format_args!("rapidcity")),
                Self::Casper => f.write_fmt(format_args!("casper")),
                Self::Pocatello => f.write_fmt(format_args!("pocatello")),
                Self::Billings => f.write_fmt(format_args!("billings")),
                Self::Butte => f.write_fmt(format_args!("butte")),
            }
        }
    }
}
pub mod southwest {
    use std::fmt;
    pub enum City {
        SanDiego,
        LosAngeles,
        Oakland,
        Reno,
        Sacramento,
        LasVegas,
        Phoenix,
        ElPaso,
        SanFrancisco,
        Tucumcari,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for City {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    City::SanDiego => "SanDiego",
                    City::LosAngeles => "LosAngeles",
                    City::Oakland => "Oakland",
                    City::Reno => "Reno",
                    City::Sacramento => "Sacramento",
                    City::LasVegas => "LasVegas",
                    City::Phoenix => "Phoenix",
                    City::ElPaso => "ElPaso",
                    City::SanFrancisco => "SanFrancisco",
                    City::Tucumcari => "Tucumcari",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for City {
        #[inline]
        fn clone(&self) -> City {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for City {}
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::SanDiego => f.write_fmt(format_args!("sandiego")),
                Self::LosAngeles => f.write_fmt(format_args!("losangeles")),
                Self::Oakland => f.write_fmt(format_args!("oakland")),
                Self::Reno => f.write_fmt(format_args!("reno")),
                Self::Sacramento => f.write_fmt(format_args!("sacramento")),
                Self::LasVegas => f.write_fmt(format_args!("lasvegas")),
                Self::Phoenix => f.write_fmt(format_args!("phoenix")),
                Self::ElPaso => f.write_fmt(format_args!("elpaso")),
                Self::SanFrancisco => f.write_fmt(format_args!("sanfrancisco")),
                Self::Tucumcari => f.write_fmt(format_args!("tucumcari")),
            }
        }
    }
}
pub struct Player {
    pub cash: u64,
    pub name: String,
    pub piece: Piece,
}
#[automatically_derived]
impl ::core::fmt::Debug for Player {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Player",
            "cash",
            &self.cash,
            "name",
            &self.name,
            "piece",
            &&self.piece,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Player {
    #[inline]
    fn clone(&self) -> Player {
        Player {
            cash: ::core::clone::Clone::clone(&self.cash),
            name: ::core::clone::Clone::clone(&self.name),
            piece: ::core::clone::Clone::clone(&self.piece),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Player {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Player {
    #[inline]
    fn eq(&self, other: &Player) -> bool {
        self.cash == other.cash && self.name == other.name && self.piece == other.piece
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Player {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Player",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "cash", &self.cash)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "name", &self.name)?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "piece",
                &self.piece,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Player {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "cash" => _serde::__private::Ok(__Field::__field0),
                        "name" => _serde::__private::Ok(__Field::__field1),
                        "piece" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"cash" => _serde::__private::Ok(__Field::__field0),
                        b"name" => _serde::__private::Ok(__Field::__field1),
                        b"piece" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Player>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Player;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Player")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct Player with 3 elements",
                            ));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct Player with 3 elements",
                            ));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<Piece>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct Player with 3 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(Player {
                        cash: __field0,
                        name: __field1,
                        piece: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Piece> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("cash"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        u64,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("piece"),
                                    );
                                }
                                __field2 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Piece,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("cash")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("name")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("piece")?,
                    };
                    _serde::__private::Ok(Player {
                        cash: __field0,
                        name: __field1,
                        piece: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["cash", "name", "piece"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Player",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Player>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    pub player_order: Vec<PlayerId>,
}
#[automatically_derived]
impl ::core::fmt::Debug for State {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "State",
            "stage",
            &self.stage,
            "active_player_id",
            &self.active_player_id,
            "player_order",
            &&self.player_order,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for State {
    #[inline]
    fn clone(&self) -> State {
        State {
            stage: ::core::clone::Clone::clone(&self.stage),
            active_player_id: ::core::clone::Clone::clone(&self.active_player_id),
            player_order: ::core::clone::Clone::clone(&self.player_order),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for State {}
#[automatically_derived]
impl ::core::cmp::PartialEq for State {
    #[inline]
    fn eq(&self, other: &State) -> bool {
        self.stage == other.stage
            && self.active_player_id == other.active_player_id
            && self.player_order == other.player_order
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for State {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "State",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "stage",
                &self.stage,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "active_player_id",
                &self.active_player_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "player_order",
                &self.player_order,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for State {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "stage" => _serde::__private::Ok(__Field::__field0),
                        "active_player_id" => _serde::__private::Ok(__Field::__field1),
                        "player_order" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"stage" => _serde::__private::Ok(__Field::__field0),
                        b"active_player_id" => _serde::__private::Ok(__Field::__field1),
                        b"player_order" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<State>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = State;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct State")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Stage>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct State with 3 elements",
                            ));
                        }
                    };
                    let __field1 =
                        match _serde::de::SeqAccess::next_element::<PlayerId>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct State with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match _serde::de::SeqAccess::next_element::<Vec<PlayerId>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct State with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(State {
                        stage: __field0,
                        active_player_id: __field1,
                        player_order: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Stage> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<PlayerId> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Vec<PlayerId>> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("stage"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Stage,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "active_player_id",
                                        ),
                                    );
                                }
                                __field1 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        PlayerId,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "player_order",
                                        ),
                                    );
                                }
                                __field2 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Vec<PlayerId>,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("stage")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("active_player_id")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("player_order")?
                        }
                    };
                    _serde::__private::Ok(State {
                        stage: __field0,
                        active_player_id: __field1,
                        player_order: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["stage", "active_player_id", "player_order"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "State",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<State>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
