pub use crate::Isotope::*;

use crate::{uncertain, Uncertain};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// H isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum H {
    One,
    Two,
    Three,
}

impl H {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::One => Some(uncertain!(0.999_885, 0.000_070)),
            Self::Two => Some(uncertain!(0.000_115, 0.000_070)),
            Self::Three => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::One => uncertain!(1.007_825_032_23, 0.000_000_000_09),
            Self::Two => uncertain!(2.014_101_778_12, 0.000_000_000_12),
            Self::Three => uncertain!(3.016_049_277_9, 0.000_000_002_4),
        }
    }
}

/// He isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum He {
    Three,
    Four,
}

impl He {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Three => Some(uncertain!(0.000_001_34, 0.000_000_03)),
            Self::Four => Some(uncertain!(0.999_998_66, 0.000_000_03)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Three => uncertain!(3.016_029_320_1, 0.000_000_002_5),
            Self::Four => uncertain!(4.002_603_254_13, 0.000_000_000_06),
        }
    }
}

/// Li isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Li {
    Six,
    Seven,
}

impl Li {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Six => Some(uncertain!(0.0759, 0.0004)),
            Self::Seven => Some(uncertain!(0.9241, 0.0004)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Six => 6,
            Self::Seven => 7,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Six => uncertain!(6.015_122_887_4, 0.000_000_001_6),
            Self::Seven => uncertain!(7.016_003_436_6, 0.000_000_004_5),
        }
    }
}

/// B isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum B {
    Ten,
    Eleven,
}

impl B {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Ten => Some(uncertain!(0.199, 0.007)),
            Self::Eleven => Some(uncertain!(0.801, 0.007)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Ten => 10,
            Self::Eleven => 11,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Ten => uncertain!(10.012_936_95, 0.000_000_41),
            Self::Eleven => uncertain!(11.009_305_36, 0.000_000_45),
        }
    }
}

/// C isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum C {
    Twelve,
    Thirteen,
    Fourteen,
}

impl C {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Twelve => Some(uncertain!(0.9893, 0.0008)),
            Self::Thirteen => Some(uncertain!(0.0107, 0.0008)),
            Self::Fourteen => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Twelve => 12,
            Self::Thirteen => 13,
            Self::Fourteen => 14,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Twelve => uncertain!(12.000_000_0, 0.000_000_0),
            Self::Thirteen => uncertain!(13.003_354_835_07, 0.000_000_000_23),
            Self::Fourteen => uncertain!(14.003_241_988_4, 0.000_000_004_0),
        }
    }
}

/// N isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum N {
    Fourteen,
    Fifteen,
}

impl N {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fourteen => Some(uncertain!(0.99636, 0.00020)),
            Self::Fifteen => Some(uncertain!(0.00364, 0.00020)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fourteen => 14,
            Self::Fifteen => 15,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fourteen => uncertain!(14.003_074_004_43, 0.000_000_000_20),
            Self::Fifteen => uncertain!(15.000_108_898_88, 0.000_000_000_64),
        }
    }
}

/// O isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum O {
    Sixteen,
    Seventeen,
    Eighteen,
}

impl O {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Sixteen => Some(uncertain!(0.99757, 0.00016)),
            Self::Seventeen => Some(uncertain!(0.00038, 0.00001)),
            Self::Eighteen => Some(uncertain!(0.00205, 0.00014)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Sixteen => 16,
            Self::Seventeen => 17,
            Self::Eighteen => 18,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Sixteen => uncertain!(15.994_914_619_57, 0.000_000_000_17),
            Self::Seventeen => uncertain!(16.999_131_756_50, 0.000_000_000_69),
            Self::Eighteen => uncertain!(17.999_159_612_86, 0.000_000_000_76),
        }
    }
}

/// Ne isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ne {
    Twenty,
    TwentyOne,
    TwentyTwo,
}

impl Ne {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Twenty => Some(uncertain!(0.9048, 0.0003)),
            Self::TwentyOne => Some(uncertain!(0.0027, 0.0001)),
            Self::TwentyTwo => Some(uncertain!(0.0925, 0.0003)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Twenty => 20,
            Self::TwentyOne => 21,
            Self::TwentyTwo => 22,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Twenty => uncertain!(19.992_440_176_2, 0.000_000_001_7),
            Self::TwentyOne => uncertain!(20.993_846_685, 0.000_000_041),
            Self::TwentyTwo => uncertain!(21.991_385_114, 0.000_000_018),
        }
    }
}

/// Mg isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mg {
    TwentyFour,
    TwentyFive,
    TwentySix,
}

impl Mg {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwentyFour => Some(uncertain!(0.7899, 0.0004)),
            Self::TwentyFive => Some(uncertain!(0.1000, 0.0001)),
            Self::TwentySix => Some(uncertain!(0.1101, 0.0003)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwentyFour => 24,
            Self::TwentyFive => 25,
            Self::TwentySix => 26,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwentyFour => uncertain!(23.985_041_697, 0.000_000_014),
            Self::TwentyFive => uncertain!(24.985_836_976, 0.000_000_050),
            Self::TwentySix => uncertain!(25.982_592_968, 0.000_000_031),
        }
    }
}

/// Si isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Si {
    TwentyEight,
    TwentyNine,
    Thirty,
}

impl Si {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwentyEight => Some(uncertain!(0.92223, 0.00019)),
            Self::TwentyNine => Some(uncertain!(0.04685, 0.00008)),
            Self::Thirty => Some(uncertain!(0.03092, 0.00011)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwentyEight => 28,
            Self::TwentyNine => 29,
            Self::Thirty => 30,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwentyEight => uncertain!(27.976_926_534_65, 0.000_000_000_44),
            Self::TwentyNine => uncertain!(28.976_494_664_90, 0.000_000_000_52),
            Self::Thirty => uncertain!(29.973_770_136, 0.000_000_023),
        }
    }
}

/// S isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum S {
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtySix,
}

impl S {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyTwo => Some(uncertain!(0.9499, 0.0026)),
            Self::ThirtyThree => Some(uncertain!(0.0075, 0.0002)),
            Self::ThirtyFour => Some(uncertain!(0.0425, 0.0024)),
            Self::ThirtySix => Some(uncertain!(0.0001, 0.0001)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyTwo => 32,
            Self::ThirtyThree => 33,
            Self::ThirtyFour => 34,
            Self::ThirtySix => 36,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyTwo => uncertain!(31.972_071_174_4, 0.000_000_001_4),
            Self::ThirtyThree => uncertain!(32.971_458_909_8, 0.000_000_001_5),
            Self::ThirtyFour => uncertain!(33.967_867_004, 0.000_000_047),
            Self::ThirtySix => uncertain!(35.967_080_71, 0.000_000_20),
        }
    }
}

/// Cl isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cl {
    ThirtyFive,
    ThirtySeven,
}

impl Cl {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyFive => Some(uncertain!(0.7576, 0.0010)),
            Self::ThirtySeven => Some(uncertain!(0.2424, 0.0010)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyFive => 35,
            Self::ThirtySeven => 37,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyFive => uncertain!(34.968_852_682, 0.000_000_037),
            Self::ThirtySeven => uncertain!(36.965_902_602, 0.000_000_055),
        }
    }
}

/// Ar isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ar {
    ThirtySix,
    ThirtyEight,
    Forty,
}

impl Ar {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtySix => Some(uncertain!(0.003_336, 0.000_021)),
            Self::ThirtyEight => Some(uncertain!(0.000_629, 0.000_007)),
            Self::Forty => Some(uncertain!(0.996_035, 0.000_025)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtySix => 36,
            Self::ThirtyEight => 38,
            Self::Forty => 40,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtySix => uncertain!(35.967_545_105, 0.000_000_028),
            Self::ThirtyEight => uncertain!(37.962_732_11, 0.000_000_21),
            Self::Forty => uncertain!(39.962_383_123_7, 0.000_000_002_4),
        }
    }
}

/// K isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum K {
    ThirtyNine,
    Forty,
    FortyOne,
}

impl K {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyNine => Some(uncertain!(0.932_581, 0.000_044)),
            Self::Forty => Some(uncertain!(0.000_117, 0.000_001)),
            Self::FortyOne => Some(uncertain!(0.067_302, 0.000_044)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyNine => 39,
            Self::Forty => 40,
            Self::FortyOne => 41,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyNine => uncertain!(38.963_706_486_4, 0.000_000_004_9),
            Self::Forty => uncertain!(39.963_998_166, 0.000_000_060),
            Self::FortyOne => uncertain!(40.961_825_257_9, 0.000_000_004_1),
        }
    }
}

/// Ca isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ca {
    Forty,
    FortyTwo,
    FortyThree,
    FortyFour,
    FortySix,
    FortyEight,
}

impl Ca {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Forty => Some(uncertain!(0.96941, 0.00156)),
            Self::FortyTwo => Some(uncertain!(0.00647, 0.00023)),
            Self::FortyThree => Some(uncertain!(0.00135, 0.00010)),
            Self::FortyFour => Some(uncertain!(0.02086, 0.00110)),
            Self::FortySix => Some(uncertain!(0.00004, 0.00003)),
            Self::FortyEight => Some(uncertain!(0.00187, 0.00021)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Forty => 40,
            Self::FortyTwo => 42,
            Self::FortyThree => 43,
            Self::FortyFour => 44,
            Self::FortySix => 46,
            Self::FortyEight => 48,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Forty => uncertain!(39.962_590_863, 0.000_000_022),
            Self::FortyTwo => uncertain!(41.958_617_83, 0.000_000_16),
            Self::FortyThree => uncertain!(42.958_766_44, 0.000_000_24),
            Self::FortyFour => uncertain!(43.955_481_56, 0.000_000_35),
            Self::FortySix => uncertain!(45.953_689_0, 0.000_002_4),
            Self::FortyEight => uncertain!(47.952_522_76, 0.000_000_13),
        }
    }
}

/// Ti isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ti {
    FortySix,
    FortySeven,
    FortyEight,
    FortyNine,
    Fifty,
}

impl Ti {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::FortySix => Some(uncertain!(0.0825, 0.0003)),
            Self::FortySeven => Some(uncertain!(0.0744, 0.0002)),
            Self::FortyEight => Some(uncertain!(0.7372, 0.0003)),
            Self::FortyNine => Some(uncertain!(0.0541, 0.0002)),
            Self::Fifty => Some(uncertain!(0.0518, 0.0002)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::FortySix => 46,
            Self::FortySeven => 47,
            Self::FortyEight => 48,
            Self::FortyNine => 49,
            Self::Fifty => 50,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FortySix => uncertain!(45.952_627_72, 0.000_000_35),
            Self::FortySeven => uncertain!(46.951_758_79, 0.000_000_38),
            Self::FortyEight => uncertain!(47.947_941_98, 0.000_000_38),
            Self::FortyNine => uncertain!(48.947_865_68, 0.000_000_39),
            Self::Fifty => uncertain!(49.944_786_89, 0.000_000_39),
        }
    }
}

/// V isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum V {
    Fifty,
    FiftyOne,
}

impl V {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fifty => Some(uncertain!(0.00250, 0.00004)),
            Self::FiftyOne => Some(uncertain!(0.99750, 0.00004)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fifty => 50,
            Self::FiftyOne => 51,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fifty => uncertain!(49.947_156_01, 0.000_000_95),
            Self::FiftyOne => uncertain!(50.943_957_04, 0.000_000_94),
        }
    }
}

/// Cr isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cr {
    Fifty,
    FiftyTwo,
    FiftyThree,
    FiftyFour,
}

impl Cr {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fifty => Some(uncertain!(0.04345, 0.00013)),
            Self::FiftyTwo => Some(uncertain!(0.83789, 0.00018)),
            Self::FiftyThree => Some(uncertain!(0.09501, 0.00017)),
            Self::FiftyFour => Some(uncertain!(0.02365, 0.00007)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fifty => 50,
            Self::FiftyTwo => 52,
            Self::FiftyThree => 53,
            Self::FiftyFour => 54,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fifty => uncertain!(49.946_041_83, 0.000_000_94),
            Self::FiftyTwo => uncertain!(51.940_506_23, 0.000_000_63),
            Self::FiftyThree => uncertain!(52.940_648_15, 0.000_000_62),
            Self::FiftyFour => uncertain!(53.938_879_16, 0.000_000_61),
        }
    }
}

/// Fe isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Fe {
    FiftyFour,
    FiftySix,
    FiftySeven,
    FiftyEight,
}

impl Fe {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::FiftyFour => Some(uncertain!(0.05845, 0.00035)),
            Self::FiftySix => Some(uncertain!(0.91754, 0.00036)),
            Self::FiftySeven => Some(uncertain!(0.02119, 0.00010)),
            Self::FiftyEight => Some(uncertain!(0.00282, 0.00004)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::FiftyFour => 54,
            Self::FiftySix => 56,
            Self::FiftySeven => 57,
            Self::FiftyEight => 58,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FiftyFour => uncertain!(53.939_608_99, 0.000_000_53),
            Self::FiftySix => uncertain!(55.934_936_33, 0.000_000_49),
            Self::FiftySeven => uncertain!(56.935_392_84, 0.000_000_49),
            Self::FiftyEight => uncertain!(57.933_274_43, 0.000_000_53),
        }
    }
}

/// Ni isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ni {
    FiftyEight,
    Sixty,
    SixtyOne,
    SixtyTwo,
    SixtyFour,
}

impl Ni {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::FiftyEight => Some(uncertain!(0.68077, 0.00019)),
            Self::Sixty => Some(uncertain!(0.26223, 0.00015)),
            Self::SixtyOne => Some(uncertain!(0.011_399, 0.000_013)),
            Self::SixtyTwo => Some(uncertain!(0.036_346, 0.000_040)),
            Self::SixtyFour => Some(uncertain!(0.009_255, 0.000_019)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::FiftyEight => 58,
            Self::Sixty => 60,
            Self::SixtyOne => 61,
            Self::SixtyTwo => 62,
            Self::SixtyFour => 64,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FiftyEight => uncertain!(57.935_342_41, 0.000_000_52),
            Self::Sixty => uncertain!(59.930_785_88, 0.000_000_52),
            Self::SixtyOne => uncertain!(60.931_055_57, 0.000_000_52),
            Self::SixtyTwo => uncertain!(61.928_345_37, 0.000_000_55),
            Self::SixtyFour => uncertain!(63.927_966_82, 0.000_000_58),
        }
    }
}

/// Cu isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cu {
    SixtyThree,
    SixtyFive,
}

impl Cu {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SixtyThree => Some(uncertain!(0.6915, 0.0015)),
            Self::SixtyFive => Some(uncertain!(0.3085, 0.0015)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SixtyThree => 63,
            Self::SixtyFive => 65,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyThree => uncertain!(62.929_597_72, 0.000_000_56),
            Self::SixtyFive => uncertain!(64.927_789_70, 0.000_000_71),
        }
    }
}

/// Zn isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Zn {
    SixtyFour,
    SixtySix,
    SixtySeven,
    SixtyEight,
    Seventy,
}

impl Zn {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SixtyFour => Some(uncertain!(0.4917, 0.0075)),
            Self::SixtySix => Some(uncertain!(0.2773, 0.0098)),
            Self::SixtySeven => Some(uncertain!(0.0404, 0.0016)),
            Self::SixtyEight => Some(uncertain!(0.1845, 0.0063)),
            Self::Seventy => Some(uncertain!(0.0061, 0.0010)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SixtyFour => 64,
            Self::SixtySix => 66,
            Self::SixtySeven => 67,
            Self::SixtyEight => 68,
            Self::Seventy => 70,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyFour => uncertain!(63.929_142_01, 0.000_000_71),
            Self::SixtySix => uncertain!(65.926_033_81, 0.000_000_94),
            Self::SixtySeven => uncertain!(66.927_127_75, 0.000_000_96),
            Self::SixtyEight => uncertain!(67.924_844_55, 0.000_000_98),
            Self::Seventy => uncertain!(69.925_319_2, 0.000_002_1),
        }
    }
}

/// Ga isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ga {
    SixtyNine,
    SeventyOne,
}

impl Ga {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SixtyNine => Some(uncertain!(0.60108, 0.00009)),
            Self::SeventyOne => Some(uncertain!(0.39892, 0.00009)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SixtyNine => 69,
            Self::SeventyOne => 71,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyNine => uncertain!(68.925_573_5, 0.000_001_3),
            Self::SeventyOne => uncertain!(70.924_702_58, 0.000_000_87),
        }
    }
}

/// Ge isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ge {
    Seventy,
    SeventyTwo,
    SeventyThree,
    SeventyFour,
    SeventySix,
}

impl Ge {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Seventy => Some(uncertain!(0.2057, 0.0027)),
            Self::SeventyTwo => Some(uncertain!(0.2745, 0.0032)),
            Self::SeventyThree => Some(uncertain!(0.0775, 0.0012)),
            Self::SeventyFour => Some(uncertain!(0.3650, 0.0020)),
            Self::SeventySix => Some(uncertain!(0.0773, 0.0012)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Seventy => 70,
            Self::SeventyTwo => 72,
            Self::SeventyThree => 73,
            Self::SeventyFour => 74,
            Self::SeventySix => 76,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Seventy => uncertain!(69.924_248_75, 0.000_000_90),
            Self::SeventyTwo => uncertain!(71.922_075_826, 0.000_000_081),
            Self::SeventyThree => uncertain!(72.923_458_956, 0.000_000_061),
            Self::SeventyFour => uncertain!(73.921_177_761, 0.000_000_013),
            Self::SeventySix => uncertain!(75.921_402_726, 0.000_000_019),
        }
    }
}

/// Se isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Se {
    SeventyFour,
    SeventySix,
    SeventySeven,
    SeventyEight,
    Eighty,
    EightyTwo,
}

impl Se {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SeventyFour => Some(uncertain!(0.0089, 0.0004)),
            Self::SeventySix => Some(uncertain!(0.0937, 0.0029)),
            Self::SeventySeven => Some(uncertain!(0.0763, 0.0016)),
            Self::SeventyEight => Some(uncertain!(0.2377, 0.0028)),
            Self::Eighty => Some(uncertain!(0.4961, 0.0041)),
            Self::EightyTwo => Some(uncertain!(0.0873, 0.0022)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SeventyFour => 74,
            Self::SeventySix => 76,
            Self::SeventySeven => 77,
            Self::SeventyEight => 78,
            Self::Eighty => 80,
            Self::EightyTwo => 82,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyFour => uncertain!(73.922_475_934, 0.000_000_015),
            Self::SeventySix => uncertain!(75.919_213_704, 0.000_000_017),
            Self::SeventySeven => uncertain!(76.919_914_154, 0.000_000_067),
            Self::SeventyEight => uncertain!(77.917_309_28, 0.000_000_20),
            Self::Eighty => uncertain!(79.916_521_8, 0.000_001_3),
            Self::EightyTwo => uncertain!(81.916_699_5, 0.000_001_5),
        }
    }
}

/// Br isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Br {
    SeventyNine,
    EightyOne,
}

impl Br {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SeventyNine => Some(uncertain!(0.5069, 0.0007)),
            Self::EightyOne => Some(uncertain!(0.4931, 0.0007)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SeventyNine => 79,
            Self::EightyOne => 81,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyNine => uncertain!(78.918_337_6, 0.000_001_4),
            Self::EightyOne => uncertain!(80.916_289_7, 0.000_001_4),
        }
    }
}

/// Kr isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Kr {
    SeventyEight,
    Eighty,
    EightyTwo,
    EightyThree,
    EightyFour,
    EightySix,
}

impl Kr {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SeventyEight => Some(uncertain!(0.00355, 0.00003)),
            Self::Eighty => Some(uncertain!(0.02286, 0.00010)),
            Self::EightyTwo => Some(uncertain!(0.11593, 0.00031)),
            Self::EightyThree => Some(uncertain!(0.11500, 0.00019)),
            Self::EightyFour => Some(uncertain!(0.56987, 0.00015)),
            Self::EightySix => Some(uncertain!(0.17279, 0.00041)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SeventyEight => 78,
            Self::Eighty => 80,
            Self::EightyTwo => 82,
            Self::EightyThree => 83,
            Self::EightyFour => 84,
            Self::EightySix => 86,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyEight => uncertain!(77.920_364_94, 0.000_000_76),
            Self::Eighty => uncertain!(79.916_378_08, 0.000_000_75),
            Self::EightyTwo => uncertain!(81.913_482_73, 0.000_000_94),
            Self::EightyThree => uncertain!(82.914_127_16, 0.000_000_32),
            Self::EightyFour => uncertain!(83.911_497_728_2, 0.000_000_004_4),
            Self::EightySix => uncertain!(85.910_610_626_9, 0.000_000_004_1),
        }
    }
}

/// Rb isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Rb {
    EightyFive,
    EightySeven,
}

impl Rb {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::EightyFive => Some(uncertain!(0.7217, 0.0002)),
            Self::EightySeven => Some(uncertain!(0.2783, 0.0002)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::EightyFive => 85,
            Self::EightySeven => 87,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::EightyFive => uncertain!(84.911_789_737_9, 0.000_000_005_4),
            Self::EightySeven => uncertain!(86.909_180_531_0, 0.000_000_006_0),
        }
    }
}

/// Sr isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sr {
    EightyFour,
    EightySix,
    EightySeven,
    EightyEight,
}

impl Sr {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::EightyFour => Some(uncertain!(0.0056, 0.0001)),
            Self::EightySix => Some(uncertain!(0.0986, 0.0001)),
            Self::EightySeven => Some(uncertain!(0.0700, 0.0001)),
            Self::EightyEight => Some(uncertain!(0.8258, 0.0001)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::EightyFour => 84,
            Self::EightySix => 86,
            Self::EightySeven => 87,
            Self::EightyEight => 88,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::EightyFour => uncertain!(83.913_419_1, 0.000_001_3),
            Self::EightySix => uncertain!(85.909_260_6, 0.000_001_2),
            Self::EightySeven => uncertain!(86.908_877_5, 0.000_001_2),
            Self::EightyEight => uncertain!(87.905_612_5, 0.000_001_2),
        }
    }
}

/// Zr isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Zr {
    Ninety,
    NinetyOne,
    NinetyTwo,
    NinetyFour,
    NinetySix,
}

impl Zr {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Ninety => Some(uncertain!(0.5145, 0.0040)),
            Self::NinetyOne => Some(uncertain!(0.1122, 0.0005)),
            Self::NinetyTwo => Some(uncertain!(0.1715, 0.0008)),
            Self::NinetyFour => Some(uncertain!(0.1738, 0.0028)),
            Self::NinetySix => Some(uncertain!(0.0280, 0.0009)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Ninety => 90,
            Self::NinetyOne => 91,
            Self::NinetyTwo => 92,
            Self::NinetyFour => 94,
            Self::NinetySix => 96,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Ninety => uncertain!(89.904_697_7, 0.000_002_0),
            Self::NinetyOne => uncertain!(90.905_639_6, 0.000_002_0),
            Self::NinetyTwo => uncertain!(91.905_034_7, 0.000_002_0),
            Self::NinetyFour => uncertain!(93.906_310_8, 0.000_002_0),
            Self::NinetySix => uncertain!(95.908_271_4, 0.000_002_1),
        }
    }
}

/// Mo isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mo {
    NinetyTwo,
    NinetyFour,
    NinetyFive,
    NinetySix,
    NinetySeven,
    NinetyEight,
    OneHundred,
}

impl Mo {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::NinetyTwo => Some(uncertain!(0.1453, 0.0030)),
            Self::NinetyFour => Some(uncertain!(0.0915, 0.0009)),
            Self::NinetyFive => Some(uncertain!(0.1584, 0.0011)),
            Self::NinetySix => Some(uncertain!(0.1667, 0.0015)),
            Self::NinetySeven => Some(uncertain!(0.0960, 0.0014)),
            Self::NinetyEight => Some(uncertain!(0.2439, 0.0037)),
            Self::OneHundred => Some(uncertain!(0.0982, 0.0031)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::NinetyTwo => 92,
            Self::NinetyFour => 94,
            Self::NinetyFive => 95,
            Self::NinetySix => 96,
            Self::NinetySeven => 97,
            Self::NinetyEight => 98,
            Self::OneHundred => 100,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetyTwo => uncertain!(91.906_807_96, 0.000_000_84),
            Self::NinetyFour => uncertain!(93.905_084_90, 0.000_000_48),
            Self::NinetyFive => uncertain!(94.905_838_77, 0.000_000_47),
            Self::NinetySix => uncertain!(95.904_676_12, 0.000_000_47),
            Self::NinetySeven => uncertain!(96.906_018_12, 0.000_000_49),
            Self::NinetyEight => uncertain!(97.905_404_82, 0.000_000_49),
            Self::OneHundred => uncertain!(99.907_471_8, 0.000_001_1),
        }
    }
}

/// Tc isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tc {
    NinetySeven,
    NinetyEight,
    NinetyNine,
}

impl Tc {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::NinetySeven => None,
            Self::NinetyEight => None,
            Self::NinetyNine => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::NinetySeven => 97,
            Self::NinetyEight => 98,
            Self::NinetyNine => 99,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetySeven => uncertain!(96.906_366_7, 0.000_004_0),
            Self::NinetyEight => uncertain!(97.907_212_4, 0.000_003_6),
            Self::NinetyNine => uncertain!(98.906_250_8, 0.000_001_0),
        }
    }
}

/// Ru isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ru {
    NinetySix,
    NinetyEight,
    NinetyNine,
    OneHundred,
    OneHundredOne,
    OneHundredTwo,
    OneHundredFour,
}

impl Ru {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::NinetySix => Some(uncertain!(0.0554, 0.0014)),
            Self::NinetyEight => Some(uncertain!(0.0187, 0.0003)),
            Self::NinetyNine => Some(uncertain!(0.1276, 0.0014)),
            Self::OneHundred => Some(uncertain!(0.1260, 0.0007)),
            Self::OneHundredOne => Some(uncertain!(0.1706, 0.0002)),
            Self::OneHundredTwo => Some(uncertain!(0.3155, 0.0014)),
            Self::OneHundredFour => Some(uncertain!(0.1862, 0.0027)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::NinetySix => 96,
            Self::NinetyEight => 98,
            Self::NinetyNine => 99,
            Self::OneHundred => 100,
            Self::OneHundredOne => 101,
            Self::OneHundredTwo => 102,
            Self::OneHundredFour => 104,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetySix => uncertain!(95.907_590_25, 0.000_000_49),
            Self::NinetyEight => uncertain!(97.905_286_8, 0.000_006_9),
            Self::NinetyNine => uncertain!(98.905_934_1, 0.000_001_1),
            Self::OneHundred => uncertain!(99.904_214_3, 0.000_001_1),
            Self::OneHundredOne => uncertain!(100.905_576_9, 0.000_001_2),
            Self::OneHundredTwo => uncertain!(101.904_344_1, 0.000_001_2),
            Self::OneHundredFour => uncertain!(103.905_427_5, 0.000_002_8),
        }
    }
}

/// Pd isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pd {
    OneHundredTwo,
    OneHundredFour,
    OneHundredFive,
    OneHundredSix,
    OneHundredEight,
    OneHundredTen,
}

impl Pd {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwo => Some(uncertain!(0.0102, 0.0001)),
            Self::OneHundredFour => Some(uncertain!(0.1114, 0.0008)),
            Self::OneHundredFive => Some(uncertain!(0.2233, 0.0008)),
            Self::OneHundredSix => Some(uncertain!(0.2733, 0.0003)),
            Self::OneHundredEight => Some(uncertain!(0.2646, 0.0009)),
            Self::OneHundredTen => Some(uncertain!(0.1172, 0.0009)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwo => 102,
            Self::OneHundredFour => 104,
            Self::OneHundredFive => 105,
            Self::OneHundredSix => 106,
            Self::OneHundredEight => 108,
            Self::OneHundredTen => 110,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwo => uncertain!(101.905_602_2, 0.000_002_8),
            Self::OneHundredFour => uncertain!(103.904_030_5, 0.000_001_4),
            Self::OneHundredFive => uncertain!(104.905_079_6, 0.000_001_2),
            Self::OneHundredSix => uncertain!(105.903_480_4, 0.000_001_2),
            Self::OneHundredEight => uncertain!(107.903_891_6, 0.000_001_2),
            Self::OneHundredTen => uncertain!(109.905_172_20, 0.000_000_75),
        }
    }
}

/// Ag isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ag {
    OneHundredSeven,
    OneHundredNine,
}

impl Ag {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSeven => Some(uncertain!(0.51839, 0.00008)),
            Self::OneHundredNine => Some(uncertain!(0.48161, 0.00008)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSeven => 107,
            Self::OneHundredNine => 109,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeven => uncertain!(106.905_091_6, 0.000_002_6),
            Self::OneHundredNine => uncertain!(108.904_755_3, 0.000_001_4),
        }
    }
}

/// Cd isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cd {
    OneHundredSix,
    OneHundredEight,
    OneHundredTen,
    OneHundredEleven,
    OneHundredTwelve,
    OneHundredThirteen,
    OneHundredFourteen,
    OneHundredSixteen,
}

impl Cd {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSix => Some(uncertain!(0.0125, 0.0006)),
            Self::OneHundredEight => Some(uncertain!(0.0089, 0.0003)),
            Self::OneHundredTen => Some(uncertain!(0.1249, 0.0018)),
            Self::OneHundredEleven => Some(uncertain!(0.1280, 0.0012)),
            Self::OneHundredTwelve => Some(uncertain!(0.2413, 0.0021)),
            Self::OneHundredThirteen => Some(uncertain!(0.1222, 0.0012)),
            Self::OneHundredFourteen => Some(uncertain!(0.2873, 0.0042)),
            Self::OneHundredSixteen => Some(uncertain!(0.0749, 0.0018)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSix => 106,
            Self::OneHundredEight => 108,
            Self::OneHundredTen => 110,
            Self::OneHundredEleven => 111,
            Self::OneHundredTwelve => 112,
            Self::OneHundredThirteen => 113,
            Self::OneHundredFourteen => 114,
            Self::OneHundredSixteen => 116,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSix => uncertain!(105.906_459_9, 0.000_001_2),
            Self::OneHundredEight => uncertain!(107.904_183_4, 0.000_001_2),
            Self::OneHundredTen => uncertain!(109.903_006_61, 0.000_000_61),
            Self::OneHundredEleven => uncertain!(110.904_182_87, 0.000_000_61),
            Self::OneHundredTwelve => uncertain!(111.902_762_87, 0.000_000_60),
            Self::OneHundredThirteen => uncertain!(112.904_408_13, 0.000_000_45),
            Self::OneHundredFourteen => uncertain!(113.903_365_09, 0.000_000_43),
            Self::OneHundredSixteen => uncertain!(115.904_763_15, 0.000_000_17),
        }
    }
}

/// In isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum In {
    OneHundredThirteen,
    OneHundredFifteen,
}

impl In {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirteen => Some(uncertain!(0.0429, 0.0005)),
            Self::OneHundredFifteen => Some(uncertain!(0.9571, 0.0005)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirteen => 113,
            Self::OneHundredFifteen => 115,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirteen => uncertain!(112.904_061_84, 0.000_000_91),
            Self::OneHundredFifteen => uncertain!(114.903_878_776, 0.000_000_012),
        }
    }
}

/// Sn isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sn {
    OneHundredTwelve,
    OneHundredFourteen,
    OneHundredFifteen,
    OneHundredSixteen,
    OneHundredSeventeen,
    OneHundredEighteen,
    OneHundredNineteen,
    OneHundredTwenty,
    OneHundredTwentyTwo,
    OneHundredTwentyFour,
}

impl Sn {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwelve => Some(uncertain!(0.0097, 0.0001)),
            Self::OneHundredFourteen => Some(uncertain!(0.0066, 0.0001)),
            Self::OneHundredFifteen => Some(uncertain!(0.0034, 0.0001)),
            Self::OneHundredSixteen => Some(uncertain!(0.1454, 0.0009)),
            Self::OneHundredSeventeen => Some(uncertain!(0.0768, 0.0007)),
            Self::OneHundredEighteen => Some(uncertain!(0.2422, 0.0009)),
            Self::OneHundredNineteen => Some(uncertain!(0.0859, 0.0004)),
            Self::OneHundredTwenty => Some(uncertain!(0.3258, 0.0009)),
            Self::OneHundredTwentyTwo => Some(uncertain!(0.0463, 0.0003)),
            Self::OneHundredTwentyFour => Some(uncertain!(0.0579, 0.0005)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwelve => 112,
            Self::OneHundredFourteen => 114,
            Self::OneHundredFifteen => 115,
            Self::OneHundredSixteen => 116,
            Self::OneHundredSeventeen => 117,
            Self::OneHundredEighteen => 118,
            Self::OneHundredNineteen => 119,
            Self::OneHundredTwenty => 120,
            Self::OneHundredTwentyTwo => 122,
            Self::OneHundredTwentyFour => 124,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwelve => uncertain!(111.904_823_87, 0.000_000_61),
            Self::OneHundredFourteen => uncertain!(113.902_782_7, 0.000_001_0),
            Self::OneHundredFifteen => uncertain!(114.903_344_699, 0.000_000_016),
            Self::OneHundredSixteen => uncertain!(115.901_742_80, 0.000_000_10),
            Self::OneHundredSeventeen => uncertain!(116.902_953_98, 0.000_000_52),
            Self::OneHundredEighteen => uncertain!(117.901_606_57, 0.000_000_54),
            Self::OneHundredNineteen => uncertain!(118.903_311_17, 0.000_000_78),
            Self::OneHundredTwenty => uncertain!(119.902_201_63, 0.000_000_97),
            Self::OneHundredTwentyTwo => uncertain!(121.903_443_8, 0.000_002_6),
            Self::OneHundredTwentyFour => uncertain!(123.905_276_6, 0.000_001_1),
        }
    }
}

/// Sb isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sb {
    OneHundredTwentyOne,
    OneHundredTwentyThree,
}

impl Sb {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwentyOne => Some(uncertain!(0.5721, 0.0005)),
            Self::OneHundredTwentyThree => Some(uncertain!(0.4279, 0.0005)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwentyOne => 121,
            Self::OneHundredTwentyThree => 123,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwentyOne => uncertain!(120.903_812_0, 0.000_003_0),
            Self::OneHundredTwentyThree => uncertain!(122.904_213_2, 0.000_002_3),
        }
    }
}

/// Te isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Te {
    OneHundredTwenty,
    OneHundredTwentyTwo,
    OneHundredTwentyThree,
    OneHundredTwentyFour,
    OneHundredTwentyFive,
    OneHundredTwentySix,
    OneHundredTwentyEight,
    OneHundredThirty,
}

impl Te {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwenty => Some(uncertain!(0.0009, 0.0001)),
            Self::OneHundredTwentyTwo => Some(uncertain!(0.0255, 0.0012)),
            Self::OneHundredTwentyThree => Some(uncertain!(0.0089, 0.0003)),
            Self::OneHundredTwentyFour => Some(uncertain!(0.0474, 0.0014)),
            Self::OneHundredTwentyFive => Some(uncertain!(0.0707, 0.0015)),
            Self::OneHundredTwentySix => Some(uncertain!(0.1884, 0.0025)),
            Self::OneHundredTwentyEight => Some(uncertain!(0.3174, 0.0008)),
            Self::OneHundredThirty => Some(uncertain!(0.3408, 0.0062)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwenty => 120,
            Self::OneHundredTwentyTwo => 122,
            Self::OneHundredTwentyThree => 123,
            Self::OneHundredTwentyFour => 124,
            Self::OneHundredTwentyFive => 125,
            Self::OneHundredTwentySix => 126,
            Self::OneHundredTwentyEight => 128,
            Self::OneHundredThirty => 130,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwenty => uncertain!(119.904_059_3, 0.000_003_3),
            Self::OneHundredTwentyTwo => uncertain!(121.903_043_5, 0.000_001_6),
            Self::OneHundredTwentyThree => uncertain!(122.904_269_8, 0.000_001_6),
            Self::OneHundredTwentyFour => uncertain!(123.902_817_1, 0.000_001_6),
            Self::OneHundredTwentyFive => uncertain!(124.904_429_9, 0.000_001_6),
            Self::OneHundredTwentySix => uncertain!(125.903_310_9, 0.000_001_6),
            Self::OneHundredTwentyEight => uncertain!(127.904_461_28, 0.000_000_93),
            Self::OneHundredThirty => uncertain!(129.906_222_748, 0.000_000_012),
        }
    }
}

/// Xe isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Xe {
    OneHundredTwentyFour,
    OneHundredTwentySix,
    OneHundredTwentyEight,
    OneHundredTwentyNine,
    OneHundredThirty,
    OneHundredThirtyOne,
    OneHundredThirtyTwo,
    OneHundredThirtyFour,
    OneHundredThirtySix,
}

impl Xe {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwentyFour => Some(uncertain!(0.000_952, 0.000_003)),
            Self::OneHundredTwentySix => Some(uncertain!(0.000_890, 0.000_002)),
            Self::OneHundredTwentyEight => Some(uncertain!(0.019_102, 0.000_008)),
            Self::OneHundredTwentyNine => Some(uncertain!(0.264_006, 0.000_082)),
            Self::OneHundredThirty => Some(uncertain!(0.040_710, 0.000_013)),
            Self::OneHundredThirtyOne => Some(uncertain!(0.212_324, 0.000_030)),
            Self::OneHundredThirtyTwo => Some(uncertain!(0.269_086, 0.000_033)),
            Self::OneHundredThirtyFour => Some(uncertain!(0.104_357, 0.000_021)),
            Self::OneHundredThirtySix => Some(uncertain!(0.088_573, 0.000_044)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwentyFour => 124,
            Self::OneHundredTwentySix => 126,
            Self::OneHundredTwentyEight => 128,
            Self::OneHundredTwentyNine => 129,
            Self::OneHundredThirty => 130,
            Self::OneHundredThirtyOne => 131,
            Self::OneHundredThirtyTwo => 132,
            Self::OneHundredThirtyFour => 134,
            Self::OneHundredThirtySix => 136,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwentyFour => uncertain!(123.905_892_0, 0.000_001_9),
            Self::OneHundredTwentySix => uncertain!(125.904_298_3, 0.000_003_8),
            Self::OneHundredTwentyEight => uncertain!(127.903_531_0, 0.000_001_1),
            Self::OneHundredTwentyNine => uncertain!(128.904_780_861_1, 0.000_000_006_0),
            Self::OneHundredThirty => uncertain!(129.903_509_349, 0.000_000_010),
            Self::OneHundredThirtyOne => uncertain!(130.905_084_06, 0.000_000_24),
            Self::OneHundredThirtyTwo => uncertain!(131.904_155_085_6, 0.000_000_005_6),
            Self::OneHundredThirtyFour => uncertain!(133.905_394_66, 0.000_000_90),
            Self::OneHundredThirtySix => uncertain!(135.907_214_484, 0.000_000_011),
        }
    }
}

/// Ba isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ba {
    OneHundredThirty,
    OneHundredThirtyTwo,
    OneHundredThirtyFour,
    OneHundredThirtyFive,
    OneHundredThirtySix,
    OneHundredThirtySeven,
    OneHundredThirtyEight,
}

impl Ba {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirty => Some(uncertain!(0.00106, 0.00001)),
            Self::OneHundredThirtyTwo => Some(uncertain!(0.00101, 0.00001)),
            Self::OneHundredThirtyFour => Some(uncertain!(0.02417, 0.00018)),
            Self::OneHundredThirtyFive => Some(uncertain!(0.06592, 0.00012)),
            Self::OneHundredThirtySix => Some(uncertain!(0.07854, 0.00024)),
            Self::OneHundredThirtySeven => Some(uncertain!(0.11232, 0.00024)),
            Self::OneHundredThirtyEight => Some(uncertain!(0.71698, 0.00042)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirty => 130,
            Self::OneHundredThirtyTwo => 132,
            Self::OneHundredThirtyFour => 134,
            Self::OneHundredThirtyFive => 135,
            Self::OneHundredThirtySix => 136,
            Self::OneHundredThirtySeven => 137,
            Self::OneHundredThirtyEight => 138,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirty => uncertain!(129.906_320_7, 0.000_002_8),
            Self::OneHundredThirtyTwo => uncertain!(131.905_061_1, 0.000_001_1),
            Self::OneHundredThirtyFour => uncertain!(133.904_508_18, 0.000_000_30),
            Self::OneHundredThirtyFive => uncertain!(134.905_688_38, 0.000_000_29),
            Self::OneHundredThirtySix => uncertain!(135.904_575_73, 0.000_000_29),
            Self::OneHundredThirtySeven => uncertain!(136.905_827_14, 0.000_000_30),
            Self::OneHundredThirtyEight => uncertain!(137.905_247_00, 0.000_000_31),
        }
    }
}

/// La isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum La {
    OneHundredThirtyEight,
    OneHundredThirtyNine,
}

impl La {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirtyEight => Some(uncertain!(0.000_888_1, 0.000_007_1)),
            Self::OneHundredThirtyNine => Some(uncertain!(0.999_111_9, 0.000_007_1)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirtyEight => 138,
            Self::OneHundredThirtyNine => 139,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirtyEight => uncertain!(137.907_114_9, 0.000_003_7),
            Self::OneHundredThirtyNine => uncertain!(138.906_356_3, 0.000_002_4),
        }
    }
}

/// Ce isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ce {
    OneHundredThirtySix,
    OneHundredThirtyEight,
    OneHundredForty,
    OneHundredFortyTwo,
}

impl Ce {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirtySix => Some(uncertain!(0.00185, 0.00002)),
            Self::OneHundredThirtyEight => Some(uncertain!(0.00251, 0.00002)),
            Self::OneHundredForty => Some(uncertain!(0.88450, 0.00051)),
            Self::OneHundredFortyTwo => Some(uncertain!(0.11114, 0.00051)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirtySix => 136,
            Self::OneHundredThirtyEight => 138,
            Self::OneHundredForty => 140,
            Self::OneHundredFortyTwo => 142,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirtySix => uncertain!(135.907_129_21, 0.000_000_41),
            Self::OneHundredThirtyEight => uncertain!(137.905_991, 0.000_011),
            Self::OneHundredForty => uncertain!(139.905_443_1, 0.000_002_3),
            Self::OneHundredFortyTwo => uncertain!(141.909_250_4, 0.000_002_9),
        }
    }
}

/// Nd isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Nd {
    OneHundredFortyTwo,
    OneHundredFortyThree,
    OneHundredFortyFour,
    OneHundredFortyFive,
    OneHundredFortySix,
    OneHundredFortyEight,
    OneHundredFifty,
}

impl Nd {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFortyTwo => Some(uncertain!(0.27152, 0.00040)),
            Self::OneHundredFortyThree => Some(uncertain!(0.12174, 0.00026)),
            Self::OneHundredFortyFour => Some(uncertain!(0.23798, 0.00019)),
            Self::OneHundredFortyFive => Some(uncertain!(0.08293, 0.00012)),
            Self::OneHundredFortySix => Some(uncertain!(0.17189, 0.00032)),
            Self::OneHundredFortyEight => Some(uncertain!(0.05756, 0.00021)),
            Self::OneHundredFifty => Some(uncertain!(0.05638, 0.00028)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFortyTwo => 142,
            Self::OneHundredFortyThree => 143,
            Self::OneHundredFortyFour => 144,
            Self::OneHundredFortyFive => 145,
            Self::OneHundredFortySix => 146,
            Self::OneHundredFortyEight => 148,
            Self::OneHundredFifty => 150,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyTwo => uncertain!(141.907_729_0, 0.000_002_0),
            Self::OneHundredFortyThree => uncertain!(142.909_820_0, 0.000_002_0),
            Self::OneHundredFortyFour => uncertain!(143.910_093_0, 0.000_002_0),
            Self::OneHundredFortyFive => uncertain!(144.912_579_3, 0.000_002_0),
            Self::OneHundredFortySix => uncertain!(145.913_122_6, 0.000_002_0),
            Self::OneHundredFortyEight => uncertain!(147.916_899_3, 0.000_002_6),
            Self::OneHundredFifty => uncertain!(149.920_902_2, 0.000_001_8),
        }
    }
}

/// Pm isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pm {
    OneHundredFortyFive,
    OneHundredFortySeven,
}

impl Pm {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFortyFive => None,
            Self::OneHundredFortySeven => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFortyFive => 145,
            Self::OneHundredFortySeven => 147,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyFive => uncertain!(144.912_755_9, 0.000_003_3),
            Self::OneHundredFortySeven => uncertain!(146.915_145_0, 0.000_001_9),
        }
    }
}

/// Sm isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sm {
    OneHundredFortyFour,
    OneHundredFortySeven,
    OneHundredFortyEight,
    OneHundredFortyNine,
    OneHundredFifty,
    OneHundredFiftyTwo,
    OneHundredFiftyFour,
}

impl Sm {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFortyFour => Some(uncertain!(0.0307, 0.0007)),
            Self::OneHundredFortySeven => Some(uncertain!(0.1499, 0.0018)),
            Self::OneHundredFortyEight => Some(uncertain!(0.1124, 0.0010)),
            Self::OneHundredFortyNine => Some(uncertain!(0.1382, 0.0007)),
            Self::OneHundredFifty => Some(uncertain!(0.0738, 0.0001)),
            Self::OneHundredFiftyTwo => Some(uncertain!(0.2675, 0.0016)),
            Self::OneHundredFiftyFour => Some(uncertain!(0.2275, 0.0029)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFortyFour => 144,
            Self::OneHundredFortySeven => 147,
            Self::OneHundredFortyEight => 148,
            Self::OneHundredFortyNine => 149,
            Self::OneHundredFifty => 150,
            Self::OneHundredFiftyTwo => 152,
            Self::OneHundredFiftyFour => 154,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyFour => uncertain!(143.912_006_5, 0.000_002_1),
            Self::OneHundredFortySeven => uncertain!(146.914_904_4, 0.000_001_9),
            Self::OneHundredFortyEight => uncertain!(147.914_829_2, 0.000_001_9),
            Self::OneHundredFortyNine => uncertain!(148.917_192_1, 0.000_001_8),
            Self::OneHundredFifty => uncertain!(149.917_282_9, 0.000_001_8),
            Self::OneHundredFiftyTwo => uncertain!(151.919_739_7, 0.000_001_8),
            Self::OneHundredFiftyFour => uncertain!(153.922_216_9, 0.000_002_0),
        }
    }
}

/// Eu isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Eu {
    OneHundredFiftyOne,
    OneHundredFiftyThree,
}

impl Eu {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFiftyOne => Some(uncertain!(0.4781, 0.0006)),
            Self::OneHundredFiftyThree => Some(uncertain!(0.5219, 0.0006)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFiftyOne => 151,
            Self::OneHundredFiftyThree => 153,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftyOne => uncertain!(150.919_857_8, 0.000_001_8),
            Self::OneHundredFiftyThree => uncertain!(152.921_238_0, 0.000_001_8),
        }
    }
}

/// Gd isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Gd {
    OneHundredFiftyTwo,
    OneHundredFiftyFour,
    OneHundredFiftyFive,
    OneHundredFiftySix,
    OneHundredFiftySeven,
    OneHundredFiftyEight,
    OneHundredSixty,
}

impl Gd {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFiftyTwo => Some(uncertain!(0.0020, 0.0001)),
            Self::OneHundredFiftyFour => Some(uncertain!(0.0218, 0.0003)),
            Self::OneHundredFiftyFive => Some(uncertain!(0.1480, 0.0012)),
            Self::OneHundredFiftySix => Some(uncertain!(0.2047, 0.0009)),
            Self::OneHundredFiftySeven => Some(uncertain!(0.1565, 0.0002)),
            Self::OneHundredFiftyEight => Some(uncertain!(0.2484, 0.0007)),
            Self::OneHundredSixty => Some(uncertain!(0.2186, 0.0019)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFiftyTwo => 152,
            Self::OneHundredFiftyFour => 154,
            Self::OneHundredFiftyFive => 155,
            Self::OneHundredFiftySix => 156,
            Self::OneHundredFiftySeven => 157,
            Self::OneHundredFiftyEight => 158,
            Self::OneHundredSixty => 160,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftyTwo => uncertain!(151.919_799_5, 0.000_001_8),
            Self::OneHundredFiftyFour => uncertain!(153.920_874_1, 0.000_001_7),
            Self::OneHundredFiftyFive => uncertain!(154.922_630_5, 0.000_001_7),
            Self::OneHundredFiftySix => uncertain!(155.922_131_2, 0.000_001_7),
            Self::OneHundredFiftySeven => uncertain!(156.923_968_6, 0.000_001_7),
            Self::OneHundredFiftyEight => uncertain!(157.924_112_3, 0.000_001_7),
            Self::OneHundredSixty => uncertain!(159.927_062_4, 0.000_001_8),
        }
    }
}

/// Dy isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dy {
    OneHundredFiftySix,
    OneHundredFiftyEight,
    OneHundredSixty,
    OneHundredSixtyOne,
    OneHundredSixtyTwo,
    OneHundredSixtyThree,
    OneHundredSixtyFour,
}

impl Dy {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFiftySix => Some(uncertain!(0.00056, 0.00003)),
            Self::OneHundredFiftyEight => Some(uncertain!(0.00095, 0.00003)),
            Self::OneHundredSixty => Some(uncertain!(0.02329, 0.00018)),
            Self::OneHundredSixtyOne => Some(uncertain!(0.18889, 0.00042)),
            Self::OneHundredSixtyTwo => Some(uncertain!(0.25475, 0.00036)),
            Self::OneHundredSixtyThree => Some(uncertain!(0.24896, 0.00042)),
            Self::OneHundredSixtyFour => Some(uncertain!(0.28260, 0.00054)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFiftySix => 156,
            Self::OneHundredFiftyEight => 158,
            Self::OneHundredSixty => 160,
            Self::OneHundredSixtyOne => 161,
            Self::OneHundredSixtyTwo => 162,
            Self::OneHundredSixtyThree => 163,
            Self::OneHundredSixtyFour => 164,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftySix => uncertain!(155.924_284_7, 0.000_001_7),
            Self::OneHundredFiftyEight => uncertain!(157.924_415_9, 0.000_003_1),
            Self::OneHundredSixty => uncertain!(159.925_204_6, 0.000_002_0),
            Self::OneHundredSixtyOne => uncertain!(160.926_940_5, 0.000_002_0),
            Self::OneHundredSixtyTwo => uncertain!(161.926_805_6, 0.000_002_0),
            Self::OneHundredSixtyThree => uncertain!(162.928_738_3, 0.000_002_0),
            Self::OneHundredSixtyFour => uncertain!(163.929_181_9, 0.000_002_0),
        }
    }
}

/// Er isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Er {
    OneHundredSixtyTwo,
    OneHundredSixtyFour,
    OneHundredSixtySix,
    OneHundredSixtySeven,
    OneHundredSixtyEight,
    OneHundredSeventy,
}

impl Er {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSixtyTwo => Some(uncertain!(0.00139, 0.00005)),
            Self::OneHundredSixtyFour => Some(uncertain!(0.01601, 0.00003)),
            Self::OneHundredSixtySix => Some(uncertain!(0.33503, 0.00036)),
            Self::OneHundredSixtySeven => Some(uncertain!(0.22869, 0.00009)),
            Self::OneHundredSixtyEight => Some(uncertain!(0.26978, 0.00018)),
            Self::OneHundredSeventy => Some(uncertain!(0.14910, 0.00036)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSixtyTwo => 162,
            Self::OneHundredSixtyFour => 164,
            Self::OneHundredSixtySix => 166,
            Self::OneHundredSixtySeven => 167,
            Self::OneHundredSixtyEight => 168,
            Self::OneHundredSeventy => 170,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSixtyTwo => uncertain!(161.928_788_4, 0.000_002_0),
            Self::OneHundredSixtyFour => uncertain!(163.929_208_8, 0.000_002_0),
            Self::OneHundredSixtySix => uncertain!(165.930_299_5, 0.000_002_2),
            Self::OneHundredSixtySeven => uncertain!(166.932_054_6, 0.000_002_2),
            Self::OneHundredSixtyEight => uncertain!(167.932_376_7, 0.000_002_2),
            Self::OneHundredSeventy => uncertain!(169.935_470_2, 0.000_002_6),
        }
    }
}

/// Yb isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Yb {
    OneHundredSixtyEight,
    OneHundredSeventy,
    OneHundredSeventyOne,
    OneHundredSeventyTwo,
    OneHundredSeventyThree,
    OneHundredSeventyFour,
    OneHundredSeventySix,
}

impl Yb {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSixtyEight => Some(uncertain!(0.00123, 0.00003)),
            Self::OneHundredSeventy => Some(uncertain!(0.02982, 0.00039)),
            Self::OneHundredSeventyOne => Some(uncertain!(0.1409, 0.0014)),
            Self::OneHundredSeventyTwo => Some(uncertain!(0.2168, 0.0013)),
            Self::OneHundredSeventyThree => Some(uncertain!(0.16103, 0.00063)),
            Self::OneHundredSeventyFour => Some(uncertain!(0.32026, 0.00080)),
            Self::OneHundredSeventySix => Some(uncertain!(0.12996, 0.00083)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSixtyEight => 168,
            Self::OneHundredSeventy => 170,
            Self::OneHundredSeventyOne => 171,
            Self::OneHundredSeventyTwo => 172,
            Self::OneHundredSeventyThree => 173,
            Self::OneHundredSeventyFour => 174,
            Self::OneHundredSeventySix => 176,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSixtyEight => uncertain!(167.933_889_6, 0.000_002_2),
            Self::OneHundredSeventy => uncertain!(169.934_766_4, 0.000_002_2),
            Self::OneHundredSeventyOne => uncertain!(170.936_330_2, 0.000_002_2),
            Self::OneHundredSeventyTwo => uncertain!(171.936_385_9, 0.000_002_2),
            Self::OneHundredSeventyThree => uncertain!(172.938_215_1, 0.000_002_2),
            Self::OneHundredSeventyFour => uncertain!(173.938_866_4, 0.000_002_2),
            Self::OneHundredSeventySix => uncertain!(175.942_576_4, 0.000_002_4),
        }
    }
}

/// Lu isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Lu {
    OneHundredSeventyFive,
    OneHundredSeventySix,
}

impl Lu {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSeventyFive => Some(uncertain!(0.97401, 0.00013)),
            Self::OneHundredSeventySix => Some(uncertain!(0.02599, 0.00013)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSeventyFive => 175,
            Self::OneHundredSeventySix => 176,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeventyFive => uncertain!(174.940_775_2, 0.000_002_0),
            Self::OneHundredSeventySix => uncertain!(175.942_689_7, 0.000_002_0),
        }
    }
}

/// Hf isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Hf {
    OneHundredSeventyFour,
    OneHundredSeventySix,
    OneHundredSeventySeven,
    OneHundredSeventyEight,
    OneHundredSeventyNine,
    OneHundredEighty,
}

impl Hf {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSeventyFour => Some(uncertain!(0.0016, 0.0001)),
            Self::OneHundredSeventySix => Some(uncertain!(0.0526, 0.0007)),
            Self::OneHundredSeventySeven => Some(uncertain!(0.1860, 0.0009)),
            Self::OneHundredSeventyEight => Some(uncertain!(0.2728, 0.0007)),
            Self::OneHundredSeventyNine => Some(uncertain!(0.1362, 0.0002)),
            Self::OneHundredEighty => Some(uncertain!(0.3508, 0.0016)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSeventyFour => 174,
            Self::OneHundredSeventySix => 176,
            Self::OneHundredSeventySeven => 177,
            Self::OneHundredSeventyEight => 178,
            Self::OneHundredSeventyNine => 179,
            Self::OneHundredEighty => 180,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeventyFour => uncertain!(173.940_046_1, 0.000_002_8),
            Self::OneHundredSeventySix => uncertain!(175.941_407_6, 0.000_002_2),
            Self::OneHundredSeventySeven => uncertain!(176.943_227_7, 0.000_002_0),
            Self::OneHundredSeventyEight => uncertain!(177.943_705_8, 0.000_002_0),
            Self::OneHundredSeventyNine => uncertain!(178.945_823_2, 0.000_002_0),
            Self::OneHundredEighty => uncertain!(179.946_557_0, 0.000_002_0),
        }
    }
}

/// Ta isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ta {
    OneHundredEighty,
    OneHundredEightyOne,
}

impl Ta {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEighty => Some(uncertain!(0.000_120_1, 0.000_003_2)),
            Self::OneHundredEightyOne => Some(uncertain!(0.999_879_9, 0.000_003_2)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEighty => 180,
            Self::OneHundredEightyOne => 181,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEighty => uncertain!(179.947_464_8, 0.000_002_4),
            Self::OneHundredEightyOne => uncertain!(180.947_995_8, 0.000_002_0),
        }
    }
}

/// W isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum W {
    OneHundredEighty,
    OneHundredEightyTwo,
    OneHundredEightyThree,
    OneHundredEightyFour,
    OneHundredEightySix,
}

impl W {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEighty => Some(uncertain!(0.0012, 0.0001)),
            Self::OneHundredEightyTwo => Some(uncertain!(0.2650, 0.0016)),
            Self::OneHundredEightyThree => Some(uncertain!(0.1431, 0.0004)),
            Self::OneHundredEightyFour => Some(uncertain!(0.3064, 0.0002)),
            Self::OneHundredEightySix => Some(uncertain!(0.2843, 0.0019)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEighty => 180,
            Self::OneHundredEightyTwo => 182,
            Self::OneHundredEightyThree => 183,
            Self::OneHundredEightyFour => 184,
            Self::OneHundredEightySix => 186,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEighty => uncertain!(179.946_710_8, 0.000_002_0),
            Self::OneHundredEightyTwo => uncertain!(181.948_203_94, 0.000_000_91),
            Self::OneHundredEightyThree => uncertain!(182.950_222_75, 0.000_000_90),
            Self::OneHundredEightyFour => uncertain!(183.950_930_92, 0.000_000_94),
            Self::OneHundredEightySix => uncertain!(185.954_362_8, 0.000_001_7),
        }
    }
}

/// Re isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Re {
    OneHundredEightyFive,
    OneHundredEightySeven,
}

impl Re {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEightyFive => Some(uncertain!(0.3740, 0.0002)),
            Self::OneHundredEightySeven => Some(uncertain!(0.6260, 0.0002)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEightyFive => 185,
            Self::OneHundredEightySeven => 187,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEightyFive => uncertain!(184.952_954_5, 0.000_001_3),
            Self::OneHundredEightySeven => uncertain!(186.955_750_1, 0.000_001_6),
        }
    }
}

/// Os isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Os {
    OneHundredEightyFour,
    OneHundredEightySix,
    OneHundredEightySeven,
    OneHundredEightyEight,
    OneHundredEightyNine,
    OneHundredNinety,
    OneHundredNinetyTwo,
}

impl Os {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEightyFour => Some(uncertain!(0.0002, 0.0001)),
            Self::OneHundredEightySix => Some(uncertain!(0.0159, 0.0003)),
            Self::OneHundredEightySeven => Some(uncertain!(0.0196, 0.0002)),
            Self::OneHundredEightyEight => Some(uncertain!(0.1324, 0.0008)),
            Self::OneHundredEightyNine => Some(uncertain!(0.1615, 0.0005)),
            Self::OneHundredNinety => Some(uncertain!(0.2626, 0.0002)),
            Self::OneHundredNinetyTwo => Some(uncertain!(0.4078, 0.0019)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEightyFour => 184,
            Self::OneHundredEightySix => 186,
            Self::OneHundredEightySeven => 187,
            Self::OneHundredEightyEight => 188,
            Self::OneHundredEightyNine => 189,
            Self::OneHundredNinety => 190,
            Self::OneHundredNinetyTwo => 192,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEightyFour => uncertain!(183.952_488_5, 0.000_001_4),
            Self::OneHundredEightySix => uncertain!(185.953_835_0, 0.000_001_6),
            Self::OneHundredEightySeven => uncertain!(186.955_747_4, 0.000_001_6),
            Self::OneHundredEightyEight => uncertain!(187.955_835_2, 0.000_001_6),
            Self::OneHundredEightyNine => uncertain!(188.958_144_2, 0.000_001_7),
            Self::OneHundredNinety => uncertain!(189.958_443_7, 0.000_001_7),
            Self::OneHundredNinetyTwo => uncertain!(191.961_477_0, 0.000_002_9),
        }
    }
}

/// Ir isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ir {
    OneHundredNinetyOne,
    OneHundredNinetyThree,
}

impl Ir {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredNinetyOne => Some(uncertain!(0.373, 0.002)),
            Self::OneHundredNinetyThree => Some(uncertain!(0.627, 0.002)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredNinetyOne => 191,
            Self::OneHundredNinetyThree => 193,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinetyOne => uncertain!(190.960_589_3, 0.000_002_1),
            Self::OneHundredNinetyThree => uncertain!(192.962_921_6, 0.000_002_1),
        }
    }
}

/// Pt isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pt {
    OneHundredNinety,
    OneHundredNinetyTwo,
    OneHundredNinetyFour,
    OneHundredNinetyFive,
    OneHundredNinetySix,
    OneHundredNinetyEight,
}

impl Pt {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredNinety => Some(uncertain!(0.00012, 0.00002)),
            Self::OneHundredNinetyTwo => Some(uncertain!(0.00782, 0.00024)),
            Self::OneHundredNinetyFour => Some(uncertain!(0.3286, 0.0040)),
            Self::OneHundredNinetyFive => Some(uncertain!(0.3378, 0.0024)),
            Self::OneHundredNinetySix => Some(uncertain!(0.2521, 0.0034)),
            Self::OneHundredNinetyEight => Some(uncertain!(0.07356, 0.00130)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredNinety => 190,
            Self::OneHundredNinetyTwo => 192,
            Self::OneHundredNinetyFour => 194,
            Self::OneHundredNinetyFive => 195,
            Self::OneHundredNinetySix => 196,
            Self::OneHundredNinetyEight => 198,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinety => uncertain!(189.959_929_7, 0.000_006_3),
            Self::OneHundredNinetyTwo => uncertain!(191.961_038_7, 0.000_003_2),
            Self::OneHundredNinetyFour => uncertain!(193.962_680_9, 0.000_001_0),
            Self::OneHundredNinetyFive => uncertain!(194.964_791_7, 0.000_001_0),
            Self::OneHundredNinetySix => uncertain!(195.964_952_09, 0.000_000_99),
            Self::OneHundredNinetyEight => uncertain!(197.967_894_9, 0.000_002_3),
        }
    }
}

/// Hg isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Hg {
    OneHundredNinetySix,
    OneHundredNinetyEight,
    OneHundredNinetyNine,
    TwoHundred,
    TwoHundredOne,
    TwoHundredTwo,
    TwoHundredFour,
}

impl Hg {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredNinetySix => Some(uncertain!(0.0015, 0.0001)),
            Self::OneHundredNinetyEight => Some(uncertain!(0.0997, 0.0020)),
            Self::OneHundredNinetyNine => Some(uncertain!(0.1687, 0.0022)),
            Self::TwoHundred => Some(uncertain!(0.2310, 0.0019)),
            Self::TwoHundredOne => Some(uncertain!(0.1318, 0.0009)),
            Self::TwoHundredTwo => Some(uncertain!(0.2986, 0.0026)),
            Self::TwoHundredFour => Some(uncertain!(0.0687, 0.0015)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredNinetySix => 196,
            Self::OneHundredNinetyEight => 198,
            Self::OneHundredNinetyNine => 199,
            Self::TwoHundred => 200,
            Self::TwoHundredOne => 201,
            Self::TwoHundredTwo => 202,
            Self::TwoHundredFour => 204,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinetySix => uncertain!(195.965_832_6, 0.000_003_2),
            Self::OneHundredNinetyEight => uncertain!(197.966_768_60, 0.000_000_52),
            Self::OneHundredNinetyNine => uncertain!(198.968_280_64, 0.000_000_46),
            Self::TwoHundred => uncertain!(199.968_326_59, 0.000_000_47),
            Self::TwoHundredOne => uncertain!(200.970_302_84, 0.000_000_69),
            Self::TwoHundredTwo => uncertain!(201.970_643_40, 0.000_000_69),
            Self::TwoHundredFour => uncertain!(203.973_493_98, 0.000_000_53),
        }
    }
}

/// Tl isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tl {
    TwoHundredThree,
    TwoHundredFive,
}

impl Tl {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThree => Some(uncertain!(0.2952, 0.0001)),
            Self::TwoHundredFive => Some(uncertain!(0.7048, 0.0001)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThree => 203,
            Self::TwoHundredFive => 205,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThree => uncertain!(202.972_344_6, 0.000_001_4),
            Self::TwoHundredFive => uncertain!(204.974_427_8, 0.000_001_4),
        }
    }
}

/// Pb isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pb {
    TwoHundredFour,
    TwoHundredSix,
    TwoHundredSeven,
    TwoHundredEight,
}

impl Pb {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFour => Some(uncertain!(0.014, 0.001)),
            Self::TwoHundredSix => Some(uncertain!(0.241, 0.001)),
            Self::TwoHundredSeven => Some(uncertain!(0.221, 0.001)),
            Self::TwoHundredEight => Some(uncertain!(0.524, 0.001)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFour => 204,
            Self::TwoHundredSix => 206,
            Self::TwoHundredSeven => 207,
            Self::TwoHundredEight => 208,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFour => uncertain!(203.973_044_0, 0.000_001_3),
            Self::TwoHundredSix => uncertain!(205.974_465_7, 0.000_001_3),
            Self::TwoHundredSeven => uncertain!(206.975_897_3, 0.000_001_3),
            Self::TwoHundredEight => uncertain!(207.976_652_5, 0.000_001_3),
        }
    }
}

/// Po isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Po {
    TwoHundredNine,
    TwoHundredTen,
}

impl Po {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredNine => None,
            Self::TwoHundredTen => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredNine => 209,
            Self::TwoHundredTen => 210,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredNine => uncertain!(208.982_430_8, 0.000_002_0),
            Self::TwoHundredTen => uncertain!(209.982_874_1, 0.000_001_3),
        }
    }
}

/// At isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum At {
    TwoHundredTen,
    TwoHundredEleven,
}

impl At {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredTen => None,
            Self::TwoHundredEleven => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredTen => 210,
            Self::TwoHundredEleven => 211,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredTen => uncertain!(209.987_147_9, 0.000_008_3),
            Self::TwoHundredEleven => uncertain!(210.987_496_6, 0.000_003_0),
        }
    }
}

/// Rn isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Rn {
    TwoHundredEleven,
    TwoHundredTwenty,
    TwoHundredTwentyTwo,
}

impl Rn {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredEleven => None,
            Self::TwoHundredTwenty => None,
            Self::TwoHundredTwentyTwo => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredEleven => 211,
            Self::TwoHundredTwenty => 220,
            Self::TwoHundredTwentyTwo => 222,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredEleven => uncertain!(210.990_601_1, 0.000_007_3),
            Self::TwoHundredTwenty => uncertain!(220.011_394_1, 0.000_002_3),
            Self::TwoHundredTwentyTwo => uncertain!(222.017_578_2, 0.000_002_5),
        }
    }
}

/// Ra isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ra {
    TwoHundredTwentyThree,
    TwoHundredTwentyFour,
    TwoHundredTwentySix,
    TwoHundredTwentyEight,
}

impl Ra {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredTwentyThree => None,
            Self::TwoHundredTwentyFour => None,
            Self::TwoHundredTwentySix => None,
            Self::TwoHundredTwentyEight => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredTwentyThree => 223,
            Self::TwoHundredTwentyFour => 224,
            Self::TwoHundredTwentySix => 226,
            Self::TwoHundredTwentyEight => 228,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredTwentyThree => uncertain!(223.018_502_3, 0.000_002_7),
            Self::TwoHundredTwentyFour => uncertain!(224.020_212_0, 0.000_002_3),
            Self::TwoHundredTwentySix => uncertain!(226.025_410_3, 0.000_002_5),
            Self::TwoHundredTwentyEight => uncertain!(228.031_070_7, 0.000_002_6),
        }
    }
}

/// Th isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Th {
    TwoHundredThirty,
    TwoHundredThirtyTwo,
}

impl Th {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirty => None,
            Self::TwoHundredThirtyTwo => Some(uncertain!(1.0)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirty => 230,
            Self::TwoHundredThirtyTwo => 232,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirty => uncertain!(230.033_134_1, 0.000_001_9),
            Self::TwoHundredThirtyTwo => uncertain!(232.038_055_8, 0.000_002_1),
        }
    }
}

/// U isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum U {
    TwoHundredThirtyThree,
    TwoHundredThirtyFour,
    TwoHundredThirtyFive,
    TwoHundredThirtySix,
    TwoHundredThirtyEight,
}

impl U {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirtyThree => None,
            Self::TwoHundredThirtyFour => Some(uncertain!(0.000_054, 0.000_005)),
            Self::TwoHundredThirtyFive => Some(uncertain!(0.007_204, 0.000_006)),
            Self::TwoHundredThirtySix => None,
            Self::TwoHundredThirtyEight => Some(uncertain!(0.992_742, 0.000_010)),
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirtyThree => 233,
            Self::TwoHundredThirtyFour => 234,
            Self::TwoHundredThirtyFive => 235,
            Self::TwoHundredThirtySix => 236,
            Self::TwoHundredThirtyEight => 238,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtyThree => uncertain!(233.039_635_5, 0.000_002_9),
            Self::TwoHundredThirtyFour => uncertain!(234.040_952_3, 0.000_001_9),
            Self::TwoHundredThirtyFive => uncertain!(235.043_930_1, 0.000_001_9),
            Self::TwoHundredThirtySix => uncertain!(236.045_568_2, 0.000_001_9),
            Self::TwoHundredThirtyEight => uncertain!(238.050_788_4, 0.000_002_0),
        }
    }
}

/// Np isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Np {
    TwoHundredThirtySix,
    TwoHundredThirtySeven,
}

impl Np {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirtySix => None,
            Self::TwoHundredThirtySeven => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirtySix => 236,
            Self::TwoHundredThirtySeven => 237,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtySix => uncertain!(236.046_570, 0.000_054),
            Self::TwoHundredThirtySeven => uncertain!(237.048_173_6, 0.000_001_9),
        }
    }
}

/// Pu isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pu {
    TwoHundredThirtyEight,
    TwoHundredThirtyNine,
    TwoHundredForty,
    TwoHundredFortyOne,
    TwoHundredFortyTwo,
    TwoHundredFortyFour,
}

impl Pu {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirtyEight => None,
            Self::TwoHundredThirtyNine => None,
            Self::TwoHundredForty => None,
            Self::TwoHundredFortyOne => None,
            Self::TwoHundredFortyTwo => None,
            Self::TwoHundredFortyFour => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirtyEight => 238,
            Self::TwoHundredThirtyNine => 239,
            Self::TwoHundredForty => 240,
            Self::TwoHundredFortyOne => 241,
            Self::TwoHundredFortyTwo => 242,
            Self::TwoHundredFortyFour => 244,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtyEight => uncertain!(238.049_560_1, 0.000_001_9),
            Self::TwoHundredThirtyNine => uncertain!(239.052_163_6, 0.000_001_9),
            Self::TwoHundredForty => uncertain!(240.053_813_8, 0.000_001_9),
            Self::TwoHundredFortyOne => uncertain!(241.056_851_7, 0.000_001_9),
            Self::TwoHundredFortyTwo => uncertain!(242.058_742_8, 0.000_002_0),
            Self::TwoHundredFortyFour => uncertain!(244.064_205_3, 0.000_005_6),
        }
    }
}

/// Am isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Am {
    TwoHundredFortyOne,
    TwoHundredFortyThree,
}

impl Am {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortyOne => None,
            Self::TwoHundredFortyThree => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortyOne => 241,
            Self::TwoHundredFortyThree => 243,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyOne => uncertain!(241.056_829_3, 0.000_001_9),
            Self::TwoHundredFortyThree => uncertain!(243.061_381_3, 0.000_002_4),
        }
    }
}

/// Cm isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cm {
    TwoHundredFortyThree,
    TwoHundredFortyFour,
    TwoHundredFortyFive,
    TwoHundredFortySix,
    TwoHundredFortySeven,
    TwoHundredFortyEight,
}

impl Cm {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortyThree => None,
            Self::TwoHundredFortyFour => None,
            Self::TwoHundredFortyFive => None,
            Self::TwoHundredFortySix => None,
            Self::TwoHundredFortySeven => None,
            Self::TwoHundredFortyEight => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortyThree => 243,
            Self::TwoHundredFortyFour => 244,
            Self::TwoHundredFortyFive => 245,
            Self::TwoHundredFortySix => 246,
            Self::TwoHundredFortySeven => 247,
            Self::TwoHundredFortyEight => 248,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyThree => uncertain!(243.061_389_3, 0.000_002_2),
            Self::TwoHundredFortyFour => uncertain!(244.062_752_8, 0.000_001_9),
            Self::TwoHundredFortyFive => uncertain!(245.065_491_5, 0.000_002_2),
            Self::TwoHundredFortySix => uncertain!(246.067_223_8, 0.000_002_2),
            Self::TwoHundredFortySeven => uncertain!(247.070_354_1, 0.000_004_7),
            Self::TwoHundredFortyEight => uncertain!(248.072_349_9, 0.000_005_6),
        }
    }
}

/// Bk isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Bk {
    TwoHundredFortySeven,
    TwoHundredFortyNine,
}

impl Bk {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortySeven => None,
            Self::TwoHundredFortyNine => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortySeven => 247,
            Self::TwoHundredFortyNine => 249,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortySeven => uncertain!(247.070_307_3, 0.000_005_9),
            Self::TwoHundredFortyNine => uncertain!(249.074_987_7, 0.000_002_7),
        }
    }
}

/// Cf isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cf {
    TwoHundredFortyNine,
    TwoHundredFifty,
    TwoHundredFiftyOne,
    TwoHundredFiftyTwo,
}

impl Cf {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortyNine => None,
            Self::TwoHundredFifty => None,
            Self::TwoHundredFiftyOne => None,
            Self::TwoHundredFiftyTwo => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortyNine => 249,
            Self::TwoHundredFifty => 250,
            Self::TwoHundredFiftyOne => 251,
            Self::TwoHundredFiftyTwo => 252,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyNine => uncertain!(249.074_853_9, 0.000_002_3),
            Self::TwoHundredFifty => uncertain!(250.076_406_2, 0.000_002_2),
            Self::TwoHundredFiftyOne => uncertain!(251.079_588_6, 0.000_004_8),
            Self::TwoHundredFiftyTwo => uncertain!(252.081_627_2, 0.000_005_6),
        }
    }
}

/// Md isotopes
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Md {
    TwoHundredFiftyEight,
    TwoHundredSixty,
}

impl Md {
    /// Isotopic composition
    #[must_use]
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFiftyEight => None,
            Self::TwoHundredSixty => None,
        }
    }

    /// Mass number
    #[must_use]
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFiftyEight => 258,
            Self::TwoHundredSixty => 260,
        }
    }

    /// Relative Atomic Mass
    #[must_use]
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFiftyEight => uncertain!(258.098_431_5, 0.000_005_0),
            Self::TwoHundredSixty => uncertain!(260.10365, 0.00034),
        }
    }
}
