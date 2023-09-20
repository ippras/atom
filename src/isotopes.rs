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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::One => Some(uncertain!(0.999885, 0.000070)),
            Self::Two => Some(uncertain!(0.000115, 0.000070)),
            Self::Three => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::One => uncertain!(1.00782503223, 0.00000000009),
            Self::Two => uncertain!(2.01410177812, 0.00000000012),
            Self::Three => uncertain!(3.0160492779, 0.0000000024),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Three => Some(uncertain!(0.00000134, 0.00000003)),
            Self::Four => Some(uncertain!(0.99999866, 0.00000003)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Three => uncertain!(3.0160293201, 0.0000000025),
            Self::Four => uncertain!(4.00260325413, 0.00000000006),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Six => Some(uncertain!(0.0759, 0.0004)),
            Self::Seven => Some(uncertain!(0.9241, 0.0004)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Six => 6,
            Self::Seven => 7,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Six => uncertain!(6.0151228874, 0.0000000016),
            Self::Seven => uncertain!(7.0160034366, 0.0000000045),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Ten => Some(uncertain!(0.199, 0.007)),
            Self::Eleven => Some(uncertain!(0.801, 0.007)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Ten => 10,
            Self::Eleven => 11,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Ten => uncertain!(10.01293695, 0.00000041),
            Self::Eleven => uncertain!(11.00930536, 0.00000045),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Twelve => Some(uncertain!(0.9893, 0.0008)),
            Self::Thirteen => Some(uncertain!(0.0107, 0.0008)),
            Self::Fourteen => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Twelve => 12,
            Self::Thirteen => 13,
            Self::Fourteen => 14,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Twelve => uncertain!(12.0000000, 0.0000000),
            Self::Thirteen => uncertain!(13.00335483507, 0.00000000023),
            Self::Fourteen => uncertain!(14.0032419884, 0.0000000040),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fourteen => Some(uncertain!(0.99636, 0.00020)),
            Self::Fifteen => Some(uncertain!(0.00364, 0.00020)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fourteen => 14,
            Self::Fifteen => 15,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fourteen => uncertain!(14.00307400443, 0.00000000020),
            Self::Fifteen => uncertain!(15.00010889888, 0.00000000064),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Sixteen => Some(uncertain!(0.99757, 0.00016)),
            Self::Seventeen => Some(uncertain!(0.00038, 0.00001)),
            Self::Eighteen => Some(uncertain!(0.00205, 0.00014)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Sixteen => 16,
            Self::Seventeen => 17,
            Self::Eighteen => 18,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Sixteen => uncertain!(15.99491461957, 0.00000000017),
            Self::Seventeen => uncertain!(16.99913175650, 0.00000000069),
            Self::Eighteen => uncertain!(17.99915961286, 0.00000000076),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Twenty => Some(uncertain!(0.9048, 0.0003)),
            Self::TwentyOne => Some(uncertain!(0.0027, 0.0001)),
            Self::TwentyTwo => Some(uncertain!(0.0925, 0.0003)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Twenty => 20,
            Self::TwentyOne => 21,
            Self::TwentyTwo => 22,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Twenty => uncertain!(19.9924401762, 0.0000000017),
            Self::TwentyOne => uncertain!(20.993846685, 0.000000041),
            Self::TwentyTwo => uncertain!(21.991385114, 0.000000018),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwentyFour => Some(uncertain!(0.7899, 0.0004)),
            Self::TwentyFive => Some(uncertain!(0.1000, 0.0001)),
            Self::TwentySix => Some(uncertain!(0.1101, 0.0003)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwentyFour => 24,
            Self::TwentyFive => 25,
            Self::TwentySix => 26,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwentyFour => uncertain!(23.985041697, 0.000000014),
            Self::TwentyFive => uncertain!(24.985836976, 0.000000050),
            Self::TwentySix => uncertain!(25.982592968, 0.000000031),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwentyEight => Some(uncertain!(0.92223, 0.00019)),
            Self::TwentyNine => Some(uncertain!(0.04685, 0.00008)),
            Self::Thirty => Some(uncertain!(0.03092, 0.00011)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwentyEight => 28,
            Self::TwentyNine => 29,
            Self::Thirty => 30,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwentyEight => uncertain!(27.97692653465, 0.00000000044),
            Self::TwentyNine => uncertain!(28.97649466490, 0.00000000052),
            Self::Thirty => uncertain!(29.973770136, 0.000000023),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyTwo => Some(uncertain!(0.9499, 0.0026)),
            Self::ThirtyThree => Some(uncertain!(0.0075, 0.0002)),
            Self::ThirtyFour => Some(uncertain!(0.0425, 0.0024)),
            Self::ThirtySix => Some(uncertain!(0.0001, 0.0001)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyTwo => 32,
            Self::ThirtyThree => 33,
            Self::ThirtyFour => 34,
            Self::ThirtySix => 36,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyTwo => uncertain!(31.9720711744, 0.0000000014),
            Self::ThirtyThree => uncertain!(32.9714589098, 0.0000000015),
            Self::ThirtyFour => uncertain!(33.967867004, 0.000000047),
            Self::ThirtySix => uncertain!(35.96708071, 0.00000020),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyFive => Some(uncertain!(0.7576, 0.0010)),
            Self::ThirtySeven => Some(uncertain!(0.2424, 0.0010)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyFive => 35,
            Self::ThirtySeven => 37,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyFive => uncertain!(34.968852682, 0.000000037),
            Self::ThirtySeven => uncertain!(36.965902602, 0.000000055),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtySix => Some(uncertain!(0.003336, 0.000021)),
            Self::ThirtyEight => Some(uncertain!(0.000629, 0.000007)),
            Self::Forty => Some(uncertain!(0.996035, 0.000025)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtySix => 36,
            Self::ThirtyEight => 38,
            Self::Forty => 40,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtySix => uncertain!(35.967545105, 0.000000028),
            Self::ThirtyEight => uncertain!(37.96273211, 0.00000021),
            Self::Forty => uncertain!(39.9623831237, 0.0000000024),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::ThirtyNine => Some(uncertain!(0.932581, 0.000044)),
            Self::Forty => Some(uncertain!(0.000117, 0.000001)),
            Self::FortyOne => Some(uncertain!(0.067302, 0.000044)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::ThirtyNine => 39,
            Self::Forty => 40,
            Self::FortyOne => 41,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::ThirtyNine => uncertain!(38.9637064864, 0.0000000049),
            Self::Forty => uncertain!(39.963998166, 0.000000060),
            Self::FortyOne => uncertain!(40.9618252579, 0.0000000041),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Forty => uncertain!(39.962590863, 0.000000022),
            Self::FortyTwo => uncertain!(41.95861783, 0.00000016),
            Self::FortyThree => uncertain!(42.95876644, 0.00000024),
            Self::FortyFour => uncertain!(43.95548156, 0.00000035),
            Self::FortySix => uncertain!(45.9536890, 0.0000024),
            Self::FortyEight => uncertain!(47.95252276, 0.00000013),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FortySix => uncertain!(45.95262772, 0.00000035),
            Self::FortySeven => uncertain!(46.95175879, 0.00000038),
            Self::FortyEight => uncertain!(47.94794198, 0.00000038),
            Self::FortyNine => uncertain!(48.94786568, 0.00000039),
            Self::Fifty => uncertain!(49.94478689, 0.00000039),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fifty => Some(uncertain!(0.00250, 0.00004)),
            Self::FiftyOne => Some(uncertain!(0.99750, 0.00004)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fifty => 50,
            Self::FiftyOne => 51,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fifty => uncertain!(49.94715601, 0.00000095),
            Self::FiftyOne => uncertain!(50.94395704, 0.00000094),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::Fifty => Some(uncertain!(0.04345, 0.00013)),
            Self::FiftyTwo => Some(uncertain!(0.83789, 0.00018)),
            Self::FiftyThree => Some(uncertain!(0.09501, 0.00017)),
            Self::FiftyFour => Some(uncertain!(0.02365, 0.00007)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::Fifty => 50,
            Self::FiftyTwo => 52,
            Self::FiftyThree => 53,
            Self::FiftyFour => 54,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Fifty => uncertain!(49.94604183, 0.00000094),
            Self::FiftyTwo => uncertain!(51.94050623, 0.00000063),
            Self::FiftyThree => uncertain!(52.94064815, 0.00000062),
            Self::FiftyFour => uncertain!(53.93887916, 0.00000061),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::FiftyFour => Some(uncertain!(0.05845, 0.00035)),
            Self::FiftySix => Some(uncertain!(0.91754, 0.00036)),
            Self::FiftySeven => Some(uncertain!(0.02119, 0.00010)),
            Self::FiftyEight => Some(uncertain!(0.00282, 0.00004)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::FiftyFour => 54,
            Self::FiftySix => 56,
            Self::FiftySeven => 57,
            Self::FiftyEight => 58,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FiftyFour => uncertain!(53.93960899, 0.00000053),
            Self::FiftySix => uncertain!(55.93493633, 0.00000049),
            Self::FiftySeven => uncertain!(56.93539284, 0.00000049),
            Self::FiftyEight => uncertain!(57.93327443, 0.00000053),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::FiftyEight => Some(uncertain!(0.68077, 0.00019)),
            Self::Sixty => Some(uncertain!(0.26223, 0.00015)),
            Self::SixtyOne => Some(uncertain!(0.011399, 0.000013)),
            Self::SixtyTwo => Some(uncertain!(0.036346, 0.000040)),
            Self::SixtyFour => Some(uncertain!(0.009255, 0.000019)),
        }
    }

    /// Mass number
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::FiftyEight => uncertain!(57.93534241, 0.00000052),
            Self::Sixty => uncertain!(59.93078588, 0.00000052),
            Self::SixtyOne => uncertain!(60.93105557, 0.00000052),
            Self::SixtyTwo => uncertain!(61.92834537, 0.00000055),
            Self::SixtyFour => uncertain!(63.92796682, 0.00000058),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SixtyThree => Some(uncertain!(0.6915, 0.0015)),
            Self::SixtyFive => Some(uncertain!(0.3085, 0.0015)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SixtyThree => 63,
            Self::SixtyFive => 65,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyThree => uncertain!(62.92959772, 0.00000056),
            Self::SixtyFive => uncertain!(64.92778970, 0.00000071),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyFour => uncertain!(63.92914201, 0.00000071),
            Self::SixtySix => uncertain!(65.92603381, 0.00000094),
            Self::SixtySeven => uncertain!(66.92712775, 0.00000096),
            Self::SixtyEight => uncertain!(67.92484455, 0.00000098),
            Self::Seventy => uncertain!(69.9253192, 0.0000021),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SixtyNine => Some(uncertain!(0.60108, 0.00009)),
            Self::SeventyOne => Some(uncertain!(0.39892, 0.00009)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SixtyNine => 69,
            Self::SeventyOne => 71,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SixtyNine => uncertain!(68.9255735, 0.0000013),
            Self::SeventyOne => uncertain!(70.92470258, 0.00000087),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Seventy => uncertain!(69.92424875, 0.00000090),
            Self::SeventyTwo => uncertain!(71.922075826, 0.000000081),
            Self::SeventyThree => uncertain!(72.923458956, 0.000000061),
            Self::SeventyFour => uncertain!(73.921177761, 0.000000013),
            Self::SeventySix => uncertain!(75.921402726, 0.000000019),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyFour => uncertain!(73.922475934, 0.000000015),
            Self::SeventySix => uncertain!(75.919213704, 0.000000017),
            Self::SeventySeven => uncertain!(76.919914154, 0.000000067),
            Self::SeventyEight => uncertain!(77.91730928, 0.00000020),
            Self::Eighty => uncertain!(79.9165218, 0.0000013),
            Self::EightyTwo => uncertain!(81.9166995, 0.0000015),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::SeventyNine => Some(uncertain!(0.5069, 0.0007)),
            Self::EightyOne => Some(uncertain!(0.4931, 0.0007)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::SeventyNine => 79,
            Self::EightyOne => 81,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyNine => uncertain!(78.9183376, 0.0000014),
            Self::EightyOne => uncertain!(80.9162897, 0.0000014),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::SeventyEight => uncertain!(77.92036494, 0.00000076),
            Self::Eighty => uncertain!(79.91637808, 0.00000075),
            Self::EightyTwo => uncertain!(81.91348273, 0.00000094),
            Self::EightyThree => uncertain!(82.91412716, 0.00000032),
            Self::EightyFour => uncertain!(83.9114977282, 0.0000000044),
            Self::EightySix => uncertain!(85.9106106269, 0.0000000041),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::EightyFive => Some(uncertain!(0.7217, 0.0002)),
            Self::EightySeven => Some(uncertain!(0.2783, 0.0002)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::EightyFive => 85,
            Self::EightySeven => 87,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::EightyFive => uncertain!(84.9117897379, 0.0000000054),
            Self::EightySeven => uncertain!(86.9091805310, 0.0000000060),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::EightyFour => Some(uncertain!(0.0056, 0.0001)),
            Self::EightySix => Some(uncertain!(0.0986, 0.0001)),
            Self::EightySeven => Some(uncertain!(0.0700, 0.0001)),
            Self::EightyEight => Some(uncertain!(0.8258, 0.0001)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::EightyFour => 84,
            Self::EightySix => 86,
            Self::EightySeven => 87,
            Self::EightyEight => 88,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::EightyFour => uncertain!(83.9134191, 0.0000013),
            Self::EightySix => uncertain!(85.9092606, 0.0000012),
            Self::EightySeven => uncertain!(86.9088775, 0.0000012),
            Self::EightyEight => uncertain!(87.9056125, 0.0000012),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::Ninety => uncertain!(89.9046977, 0.0000020),
            Self::NinetyOne => uncertain!(90.9056396, 0.0000020),
            Self::NinetyTwo => uncertain!(91.9050347, 0.0000020),
            Self::NinetyFour => uncertain!(93.9063108, 0.0000020),
            Self::NinetySix => uncertain!(95.9082714, 0.0000021),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetyTwo => uncertain!(91.90680796, 0.00000084),
            Self::NinetyFour => uncertain!(93.90508490, 0.00000048),
            Self::NinetyFive => uncertain!(94.90583877, 0.00000047),
            Self::NinetySix => uncertain!(95.90467612, 0.00000047),
            Self::NinetySeven => uncertain!(96.90601812, 0.00000049),
            Self::NinetyEight => uncertain!(97.90540482, 0.00000049),
            Self::OneHundred => uncertain!(99.9074718, 0.0000011),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::NinetySeven => None,
            Self::NinetyEight => None,
            Self::NinetyNine => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::NinetySeven => 97,
            Self::NinetyEight => 98,
            Self::NinetyNine => 99,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetySeven => uncertain!(96.9063667, 0.0000040),
            Self::NinetyEight => uncertain!(97.9072124, 0.0000036),
            Self::NinetyNine => uncertain!(98.9062508, 0.0000010),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::NinetySix => uncertain!(95.90759025, 0.00000049),
            Self::NinetyEight => uncertain!(97.9052868, 0.0000069),
            Self::NinetyNine => uncertain!(98.9059341, 0.0000011),
            Self::OneHundred => uncertain!(99.9042143, 0.0000011),
            Self::OneHundredOne => uncertain!(100.9055769, 0.0000012),
            Self::OneHundredTwo => uncertain!(101.9043441, 0.0000012),
            Self::OneHundredFour => uncertain!(103.9054275, 0.0000028),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwo => uncertain!(101.9056022, 0.0000028),
            Self::OneHundredFour => uncertain!(103.9040305, 0.0000014),
            Self::OneHundredFive => uncertain!(104.9050796, 0.0000012),
            Self::OneHundredSix => uncertain!(105.9034804, 0.0000012),
            Self::OneHundredEight => uncertain!(107.9038916, 0.0000012),
            Self::OneHundredTen => uncertain!(109.90517220, 0.00000075),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSeven => Some(uncertain!(0.51839, 0.00008)),
            Self::OneHundredNine => Some(uncertain!(0.48161, 0.00008)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSeven => 107,
            Self::OneHundredNine => 109,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeven => uncertain!(106.9050916, 0.0000026),
            Self::OneHundredNine => uncertain!(108.9047553, 0.0000014),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSix => uncertain!(105.9064599, 0.0000012),
            Self::OneHundredEight => uncertain!(107.9041834, 0.0000012),
            Self::OneHundredTen => uncertain!(109.90300661, 0.00000061),
            Self::OneHundredEleven => uncertain!(110.90418287, 0.00000061),
            Self::OneHundredTwelve => uncertain!(111.90276287, 0.00000060),
            Self::OneHundredThirteen => uncertain!(112.90440813, 0.00000045),
            Self::OneHundredFourteen => uncertain!(113.90336509, 0.00000043),
            Self::OneHundredSixteen => uncertain!(115.90476315, 0.00000017),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirteen => Some(uncertain!(0.0429, 0.0005)),
            Self::OneHundredFifteen => Some(uncertain!(0.9571, 0.0005)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirteen => 113,
            Self::OneHundredFifteen => 115,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirteen => uncertain!(112.90406184, 0.00000091),
            Self::OneHundredFifteen => uncertain!(114.903878776, 0.000000012),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwelve => uncertain!(111.90482387, 0.00000061),
            Self::OneHundredFourteen => uncertain!(113.9027827, 0.0000010),
            Self::OneHundredFifteen => uncertain!(114.903344699, 0.000000016),
            Self::OneHundredSixteen => uncertain!(115.90174280, 0.00000010),
            Self::OneHundredSeventeen => uncertain!(116.90295398, 0.00000052),
            Self::OneHundredEighteen => uncertain!(117.90160657, 0.00000054),
            Self::OneHundredNineteen => uncertain!(118.90331117, 0.00000078),
            Self::OneHundredTwenty => uncertain!(119.90220163, 0.00000097),
            Self::OneHundredTwentyTwo => uncertain!(121.9034438, 0.0000026),
            Self::OneHundredTwentyFour => uncertain!(123.9052766, 0.0000011),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwentyOne => Some(uncertain!(0.5721, 0.0005)),
            Self::OneHundredTwentyThree => Some(uncertain!(0.4279, 0.0005)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredTwentyOne => 121,
            Self::OneHundredTwentyThree => 123,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwentyOne => uncertain!(120.9038120, 0.0000030),
            Self::OneHundredTwentyThree => uncertain!(122.9042132, 0.0000023),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwenty => uncertain!(119.9040593, 0.0000033),
            Self::OneHundredTwentyTwo => uncertain!(121.9030435, 0.0000016),
            Self::OneHundredTwentyThree => uncertain!(122.9042698, 0.0000016),
            Self::OneHundredTwentyFour => uncertain!(123.9028171, 0.0000016),
            Self::OneHundredTwentyFive => uncertain!(124.9044299, 0.0000016),
            Self::OneHundredTwentySix => uncertain!(125.9033109, 0.0000016),
            Self::OneHundredTwentyEight => uncertain!(127.90446128, 0.00000093),
            Self::OneHundredThirty => uncertain!(129.906222748, 0.000000012),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredTwentyFour => Some(uncertain!(0.000952, 0.000003)),
            Self::OneHundredTwentySix => Some(uncertain!(0.000890, 0.000002)),
            Self::OneHundredTwentyEight => Some(uncertain!(0.019102, 0.000008)),
            Self::OneHundredTwentyNine => Some(uncertain!(0.264006, 0.000082)),
            Self::OneHundredThirty => Some(uncertain!(0.040710, 0.000013)),
            Self::OneHundredThirtyOne => Some(uncertain!(0.212324, 0.000030)),
            Self::OneHundredThirtyTwo => Some(uncertain!(0.269086, 0.000033)),
            Self::OneHundredThirtyFour => Some(uncertain!(0.104357, 0.000021)),
            Self::OneHundredThirtySix => Some(uncertain!(0.088573, 0.000044)),
        }
    }

    /// Mass number
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredTwentyFour => uncertain!(123.9058920, 0.0000019),
            Self::OneHundredTwentySix => uncertain!(125.9042983, 0.0000038),
            Self::OneHundredTwentyEight => uncertain!(127.9035310, 0.0000011),
            Self::OneHundredTwentyNine => uncertain!(128.9047808611, 0.0000000060),
            Self::OneHundredThirty => uncertain!(129.903509349, 0.000000010),
            Self::OneHundredThirtyOne => uncertain!(130.90508406, 0.00000024),
            Self::OneHundredThirtyTwo => uncertain!(131.9041550856, 0.0000000056),
            Self::OneHundredThirtyFour => uncertain!(133.90539466, 0.00000090),
            Self::OneHundredThirtySix => uncertain!(135.907214484, 0.000000011),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirty => uncertain!(129.9063207, 0.0000028),
            Self::OneHundredThirtyTwo => uncertain!(131.9050611, 0.0000011),
            Self::OneHundredThirtyFour => uncertain!(133.90450818, 0.00000030),
            Self::OneHundredThirtyFive => uncertain!(134.90568838, 0.00000029),
            Self::OneHundredThirtySix => uncertain!(135.90457573, 0.00000029),
            Self::OneHundredThirtySeven => uncertain!(136.90582714, 0.00000030),
            Self::OneHundredThirtyEight => uncertain!(137.90524700, 0.00000031),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirtyEight => Some(uncertain!(0.0008881, 0.0000071)),
            Self::OneHundredThirtyNine => Some(uncertain!(0.9991119, 0.0000071)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirtyEight => 138,
            Self::OneHundredThirtyNine => 139,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirtyEight => uncertain!(137.9071149, 0.0000037),
            Self::OneHundredThirtyNine => uncertain!(138.9063563, 0.0000024),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredThirtySix => Some(uncertain!(0.00185, 0.00002)),
            Self::OneHundredThirtyEight => Some(uncertain!(0.00251, 0.00002)),
            Self::OneHundredForty => Some(uncertain!(0.88450, 0.00051)),
            Self::OneHundredFortyTwo => Some(uncertain!(0.11114, 0.00051)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredThirtySix => 136,
            Self::OneHundredThirtyEight => 138,
            Self::OneHundredForty => 140,
            Self::OneHundredFortyTwo => 142,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredThirtySix => uncertain!(135.90712921, 0.00000041),
            Self::OneHundredThirtyEight => uncertain!(137.905991, 0.000011),
            Self::OneHundredForty => uncertain!(139.9054431, 0.0000023),
            Self::OneHundredFortyTwo => uncertain!(141.9092504, 0.0000029),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyTwo => uncertain!(141.9077290, 0.0000020),
            Self::OneHundredFortyThree => uncertain!(142.9098200, 0.0000020),
            Self::OneHundredFortyFour => uncertain!(143.9100930, 0.0000020),
            Self::OneHundredFortyFive => uncertain!(144.9125793, 0.0000020),
            Self::OneHundredFortySix => uncertain!(145.9131226, 0.0000020),
            Self::OneHundredFortyEight => uncertain!(147.9168993, 0.0000026),
            Self::OneHundredFifty => uncertain!(149.9209022, 0.0000018),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFortyFive => None,
            Self::OneHundredFortySeven => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFortyFive => 145,
            Self::OneHundredFortySeven => 147,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyFive => uncertain!(144.9127559, 0.0000033),
            Self::OneHundredFortySeven => uncertain!(146.9151450, 0.0000019),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFortyFour => uncertain!(143.9120065, 0.0000021),
            Self::OneHundredFortySeven => uncertain!(146.9149044, 0.0000019),
            Self::OneHundredFortyEight => uncertain!(147.9148292, 0.0000019),
            Self::OneHundredFortyNine => uncertain!(148.9171921, 0.0000018),
            Self::OneHundredFifty => uncertain!(149.9172829, 0.0000018),
            Self::OneHundredFiftyTwo => uncertain!(151.9197397, 0.0000018),
            Self::OneHundredFiftyFour => uncertain!(153.9222169, 0.0000020),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredFiftyOne => Some(uncertain!(0.4781, 0.0006)),
            Self::OneHundredFiftyThree => Some(uncertain!(0.5219, 0.0006)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredFiftyOne => 151,
            Self::OneHundredFiftyThree => 153,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftyOne => uncertain!(150.9198578, 0.0000018),
            Self::OneHundredFiftyThree => uncertain!(152.9212380, 0.0000018),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftyTwo => uncertain!(151.9197995, 0.0000018),
            Self::OneHundredFiftyFour => uncertain!(153.9208741, 0.0000017),
            Self::OneHundredFiftyFive => uncertain!(154.9226305, 0.0000017),
            Self::OneHundredFiftySix => uncertain!(155.9221312, 0.0000017),
            Self::OneHundredFiftySeven => uncertain!(156.9239686, 0.0000017),
            Self::OneHundredFiftyEight => uncertain!(157.9241123, 0.0000017),
            Self::OneHundredSixty => uncertain!(159.9270624, 0.0000018),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredFiftySix => uncertain!(155.9242847, 0.0000017),
            Self::OneHundredFiftyEight => uncertain!(157.9244159, 0.0000031),
            Self::OneHundredSixty => uncertain!(159.9252046, 0.0000020),
            Self::OneHundredSixtyOne => uncertain!(160.9269405, 0.0000020),
            Self::OneHundredSixtyTwo => uncertain!(161.9268056, 0.0000020),
            Self::OneHundredSixtyThree => uncertain!(162.9287383, 0.0000020),
            Self::OneHundredSixtyFour => uncertain!(163.9291819, 0.0000020),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSixtyTwo => uncertain!(161.9287884, 0.0000020),
            Self::OneHundredSixtyFour => uncertain!(163.9292088, 0.0000020),
            Self::OneHundredSixtySix => uncertain!(165.9302995, 0.0000022),
            Self::OneHundredSixtySeven => uncertain!(166.9320546, 0.0000022),
            Self::OneHundredSixtyEight => uncertain!(167.9323767, 0.0000022),
            Self::OneHundredSeventy => uncertain!(169.9354702, 0.0000026),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSixtyEight => uncertain!(167.9338896, 0.0000022),
            Self::OneHundredSeventy => uncertain!(169.9347664, 0.0000022),
            Self::OneHundredSeventyOne => uncertain!(170.9363302, 0.0000022),
            Self::OneHundredSeventyTwo => uncertain!(171.9363859, 0.0000022),
            Self::OneHundredSeventyThree => uncertain!(172.9382151, 0.0000022),
            Self::OneHundredSeventyFour => uncertain!(173.9388664, 0.0000022),
            Self::OneHundredSeventySix => uncertain!(175.9425764, 0.0000024),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredSeventyFive => Some(uncertain!(0.97401, 0.00013)),
            Self::OneHundredSeventySix => Some(uncertain!(0.02599, 0.00013)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredSeventyFive => 175,
            Self::OneHundredSeventySix => 176,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeventyFive => uncertain!(174.9407752, 0.0000020),
            Self::OneHundredSeventySix => uncertain!(175.9426897, 0.0000020),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredSeventyFour => uncertain!(173.9400461, 0.0000028),
            Self::OneHundredSeventySix => uncertain!(175.9414076, 0.0000022),
            Self::OneHundredSeventySeven => uncertain!(176.9432277, 0.0000020),
            Self::OneHundredSeventyEight => uncertain!(177.9437058, 0.0000020),
            Self::OneHundredSeventyNine => uncertain!(178.9458232, 0.0000020),
            Self::OneHundredEighty => uncertain!(179.9465570, 0.0000020),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEighty => Some(uncertain!(0.0001201, 0.0000032)),
            Self::OneHundredEightyOne => Some(uncertain!(0.9998799, 0.0000032)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEighty => 180,
            Self::OneHundredEightyOne => 181,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEighty => uncertain!(179.9474648, 0.0000024),
            Self::OneHundredEightyOne => uncertain!(180.9479958, 0.0000020),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEighty => uncertain!(179.9467108, 0.0000020),
            Self::OneHundredEightyTwo => uncertain!(181.94820394, 0.00000091),
            Self::OneHundredEightyThree => uncertain!(182.95022275, 0.00000090),
            Self::OneHundredEightyFour => uncertain!(183.95093092, 0.00000094),
            Self::OneHundredEightySix => uncertain!(185.9543628, 0.0000017),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredEightyFive => Some(uncertain!(0.3740, 0.0002)),
            Self::OneHundredEightySeven => Some(uncertain!(0.6260, 0.0002)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredEightyFive => 185,
            Self::OneHundredEightySeven => 187,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEightyFive => uncertain!(184.9529545, 0.0000013),
            Self::OneHundredEightySeven => uncertain!(186.9557501, 0.0000016),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredEightyFour => uncertain!(183.9524885, 0.0000014),
            Self::OneHundredEightySix => uncertain!(185.9538350, 0.0000016),
            Self::OneHundredEightySeven => uncertain!(186.9557474, 0.0000016),
            Self::OneHundredEightyEight => uncertain!(187.9558352, 0.0000016),
            Self::OneHundredEightyNine => uncertain!(188.9581442, 0.0000017),
            Self::OneHundredNinety => uncertain!(189.9584437, 0.0000017),
            Self::OneHundredNinetyTwo => uncertain!(191.9614770, 0.0000029),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::OneHundredNinetyOne => Some(uncertain!(0.373, 0.002)),
            Self::OneHundredNinetyThree => Some(uncertain!(0.627, 0.002)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::OneHundredNinetyOne => 191,
            Self::OneHundredNinetyThree => 193,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinetyOne => uncertain!(190.9605893, 0.0000021),
            Self::OneHundredNinetyThree => uncertain!(192.9629216, 0.0000021),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinety => uncertain!(189.9599297, 0.0000063),
            Self::OneHundredNinetyTwo => uncertain!(191.9610387, 0.0000032),
            Self::OneHundredNinetyFour => uncertain!(193.9626809, 0.0000010),
            Self::OneHundredNinetyFive => uncertain!(194.9647917, 0.0000010),
            Self::OneHundredNinetySix => uncertain!(195.96495209, 0.00000099),
            Self::OneHundredNinetyEight => uncertain!(197.9678949, 0.0000023),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::OneHundredNinetySix => uncertain!(195.9658326, 0.0000032),
            Self::OneHundredNinetyEight => uncertain!(197.96676860, 0.00000052),
            Self::OneHundredNinetyNine => uncertain!(198.96828064, 0.00000046),
            Self::TwoHundred => uncertain!(199.96832659, 0.00000047),
            Self::TwoHundredOne => uncertain!(200.97030284, 0.00000069),
            Self::TwoHundredTwo => uncertain!(201.97064340, 0.00000069),
            Self::TwoHundredFour => uncertain!(203.97349398, 0.00000053),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThree => Some(uncertain!(0.2952, 0.0001)),
            Self::TwoHundredFive => Some(uncertain!(0.7048, 0.0001)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThree => 203,
            Self::TwoHundredFive => 205,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThree => uncertain!(202.9723446, 0.0000014),
            Self::TwoHundredFive => uncertain!(204.9744278, 0.0000014),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFour => Some(uncertain!(0.014, 0.001)),
            Self::TwoHundredSix => Some(uncertain!(0.241, 0.001)),
            Self::TwoHundredSeven => Some(uncertain!(0.221, 0.001)),
            Self::TwoHundredEight => Some(uncertain!(0.524, 0.001)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFour => 204,
            Self::TwoHundredSix => 206,
            Self::TwoHundredSeven => 207,
            Self::TwoHundredEight => 208,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFour => uncertain!(203.9730440, 0.0000013),
            Self::TwoHundredSix => uncertain!(205.9744657, 0.0000013),
            Self::TwoHundredSeven => uncertain!(206.9758973, 0.0000013),
            Self::TwoHundredEight => uncertain!(207.9766525, 0.0000013),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredNine => None,
            Self::TwoHundredTen => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredNine => 209,
            Self::TwoHundredTen => 210,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredNine => uncertain!(208.9824308, 0.0000020),
            Self::TwoHundredTen => uncertain!(209.9828741, 0.0000013),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredTen => None,
            Self::TwoHundredEleven => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredTen => 210,
            Self::TwoHundredEleven => 211,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredTen => uncertain!(209.9871479, 0.0000083),
            Self::TwoHundredEleven => uncertain!(210.9874966, 0.0000030),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredEleven => None,
            Self::TwoHundredTwenty => None,
            Self::TwoHundredTwentyTwo => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredEleven => 211,
            Self::TwoHundredTwenty => 220,
            Self::TwoHundredTwentyTwo => 222,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredEleven => uncertain!(210.9906011, 0.0000073),
            Self::TwoHundredTwenty => uncertain!(220.0113941, 0.0000023),
            Self::TwoHundredTwentyTwo => uncertain!(222.0175782, 0.0000025),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredTwentyThree => None,
            Self::TwoHundredTwentyFour => None,
            Self::TwoHundredTwentySix => None,
            Self::TwoHundredTwentyEight => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredTwentyThree => 223,
            Self::TwoHundredTwentyFour => 224,
            Self::TwoHundredTwentySix => 226,
            Self::TwoHundredTwentyEight => 228,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredTwentyThree => uncertain!(223.0185023, 0.0000027),
            Self::TwoHundredTwentyFour => uncertain!(224.0202120, 0.0000023),
            Self::TwoHundredTwentySix => uncertain!(226.0254103, 0.0000025),
            Self::TwoHundredTwentyEight => uncertain!(228.0310707, 0.0000026),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirty => None,
            Self::TwoHundredThirtyTwo => Some(uncertain!(1.0)),
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirty => 230,
            Self::TwoHundredThirtyTwo => 232,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirty => uncertain!(230.0331341, 0.0000019),
            Self::TwoHundredThirtyTwo => uncertain!(232.0380558, 0.0000021),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirtyThree => None,
            Self::TwoHundredThirtyFour => Some(uncertain!(0.000054, 0.000005)),
            Self::TwoHundredThirtyFive => Some(uncertain!(0.007204, 0.000006)),
            Self::TwoHundredThirtySix => None,
            Self::TwoHundredThirtyEight => Some(uncertain!(0.992742, 0.000010)),
        }
    }

    /// Mass number
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtyThree => uncertain!(233.0396355, 0.0000029),
            Self::TwoHundredThirtyFour => uncertain!(234.0409523, 0.0000019),
            Self::TwoHundredThirtyFive => uncertain!(235.0439301, 0.0000019),
            Self::TwoHundredThirtySix => uncertain!(236.0455682, 0.0000019),
            Self::TwoHundredThirtyEight => uncertain!(238.0507884, 0.0000020),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredThirtySix => None,
            Self::TwoHundredThirtySeven => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredThirtySix => 236,
            Self::TwoHundredThirtySeven => 237,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtySix => uncertain!(236.046570, 0.000054),
            Self::TwoHundredThirtySeven => uncertain!(237.0481736, 0.0000019),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredThirtyEight => uncertain!(238.0495601, 0.0000019),
            Self::TwoHundredThirtyNine => uncertain!(239.0521636, 0.0000019),
            Self::TwoHundredForty => uncertain!(240.0538138, 0.0000019),
            Self::TwoHundredFortyOne => uncertain!(241.0568517, 0.0000019),
            Self::TwoHundredFortyTwo => uncertain!(242.0587428, 0.0000020),
            Self::TwoHundredFortyFour => uncertain!(244.0642053, 0.0000056),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortyOne => None,
            Self::TwoHundredFortyThree => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortyOne => 241,
            Self::TwoHundredFortyThree => 243,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyOne => uncertain!(241.0568293, 0.0000019),
            Self::TwoHundredFortyThree => uncertain!(243.0613813, 0.0000024),
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
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyThree => uncertain!(243.0613893, 0.0000022),
            Self::TwoHundredFortyFour => uncertain!(244.0627528, 0.0000019),
            Self::TwoHundredFortyFive => uncertain!(245.0654915, 0.0000022),
            Self::TwoHundredFortySix => uncertain!(246.0672238, 0.0000022),
            Self::TwoHundredFortySeven => uncertain!(247.0703541, 0.0000047),
            Self::TwoHundredFortyEight => uncertain!(248.0723499, 0.0000056),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortySeven => None,
            Self::TwoHundredFortyNine => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortySeven => 247,
            Self::TwoHundredFortyNine => 249,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortySeven => uncertain!(247.0703073, 0.0000059),
            Self::TwoHundredFortyNine => uncertain!(249.0749877, 0.0000027),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFortyNine => None,
            Self::TwoHundredFifty => None,
            Self::TwoHundredFiftyOne => None,
            Self::TwoHundredFiftyTwo => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFortyNine => 249,
            Self::TwoHundredFifty => 250,
            Self::TwoHundredFiftyOne => 251,
            Self::TwoHundredFiftyTwo => 252,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFortyNine => uncertain!(249.0748539, 0.0000023),
            Self::TwoHundredFifty => uncertain!(250.0764062, 0.0000022),
            Self::TwoHundredFiftyOne => uncertain!(251.0795886, 0.0000048),
            Self::TwoHundredFiftyTwo => uncertain!(252.0816272, 0.0000056),
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
    pub const fn isotopic_composition(&self) -> Option<Uncertain> {
        match self {
            Self::TwoHundredFiftyEight => None,
            Self::TwoHundredSixty => None,
        }
    }

    /// Mass number
    pub const fn mass_number(&self) -> usize {
        match self {
            Self::TwoHundredFiftyEight => 258,
            Self::TwoHundredSixty => 260,
        }
    }

    /// Relative Atomic Mass
    pub const fn relative_atomic_mass(&self) -> Uncertain {
        match self {
            Self::TwoHundredFiftyEight => uncertain!(258.0984315, 0.0000050),
            Self::TwoHundredSixty => uncertain!(260.10365, 0.00034),
        }
    }
}
