use crate::{uncertain, Element, Element::*, Uncertain};

impl Element {
    /// Standard atomic weight
    ///
    /// Relative atomic mass of the element
    pub const fn standard_atomic_weight(&self) -> Option<Uncertain> {
        match self {
            H => Some(uncertain!(1.007_84..1.008_11)),
            He => Some(uncertain!(4.002_602, 0.000_002)),
            Li => Some(uncertain!(6.938..6.997)),
            Be => Some(uncertain!(9.012_183_1, 0.000_000_5)),
            B => Some(uncertain!(10.806..10.821)),
            C => Some(uncertain!(12.009_6..12.011_6)),
            N => Some(uncertain!(14.006_43..14.007_28)),
            O => Some(uncertain!(15.999_03..15.999_77)),
            #[cfg(feature = "nist")]
            F => Some(uncertain!(18.998_403_163, 0.000_000_006)),
            #[cfg(feature = "iupac")]
            F => Some(uncertain!(18.998_403_162, 0.000_000_005)),
            Ne => Some(uncertain!(20.179_7, 0.000_6)),
            Na => Some(uncertain!(22.989_769_28, 0.000_000_02)),
            Mg => Some(uncertain!(24.304..24.307)),
            #[cfg(feature = "nist")]
            Al => Some(uncertain!(26.981_538_5, 0.000_000_7)),
            #[cfg(feature = "iupac")]
            Al => Some(uncertain!(26.981_538_4, 0.000_000_3)),
            Si => Some(uncertain!(28.084..28.086)),
            P => Some(uncertain!(30.973_761_998, 0.000_000_005)),
            S => Some(uncertain!(32.059..32.076)),
            Cl => Some(uncertain!(35.446..35.457)),
            #[cfg(feature = "nist")]
            Ar => Some(uncertain!(39.948, 0.001)),
            #[cfg(feature = "iupac")]
            Ar => Some(uncertain!(39.792..39.963)),
            K => Some(uncertain!(39.098_3, 0.000_1)),
            Ca => Some(uncertain!(40.078, 0.004)),
            #[cfg(feature = "nist")]
            Sc => Some(uncertain!(44.955_908, 0.000_005)),
            #[cfg(feature = "iupac")]
            Sc => Some(uncertain!(44.955_907, 0.000_004)),
            Ti => Some(uncertain!(47.867, 0.001)),
            V => Some(uncertain!(50.941_5, 0.000_1)),
            Cr => Some(uncertain!(51.996_1, 0.000_6)),
            #[cfg(feature = "nist")]
            Mn => Some(uncertain!(54.938_044, 0.000_003)),
            #[cfg(feature = "iupac")]
            Mn => Some(uncertain!(54.938_043, 0.000_002)),
            Fe => Some(uncertain!(55.845, 0.002)),
            #[cfg(feature = "nist")]
            Co => Some(uncertain!(58.933_194, 0.000_004)),
            #[cfg(feature = "iupac")]
            Co => Some(uncertain!(58.933_194, 0.000_003)),
            Ni => Some(uncertain!(58.693_4, 0.000_4)),
            Cu => Some(uncertain!(63.546, 0.003)),
            Zn => Some(uncertain!(65.38, 0.02)),
            Ga => Some(uncertain!(69.723, 0.001)),
            Ge => Some(uncertain!(72.630, 0.008)),
            As => Some(uncertain!(74.921_595, 0.000_006)),
            Se => Some(uncertain!(78.971, 0.008)),
            Br => Some(uncertain!(79.901..79.907)),
            Kr => Some(uncertain!(83.798, 0.002)),
            Rb => Some(uncertain!(85.467_8, 0.000_3)),
            Sr => Some(uncertain!(87.62, 0.01)),
            #[cfg(feature = "nist")]
            Y => Some(uncertain!(88.905_84, 0.000_02)),
            #[cfg(feature = "iupac")]
            Y => Some(uncertain!(88.905_838, 0.000_002)),
            Zr => Some(uncertain!(91.224, 0.002)),
            #[cfg(feature = "nist")]
            Nb => Some(uncertain!(92.906_37, 0.000_02)),
            #[cfg(feature = "iupac")]
            Nb => Some(uncertain!(92.906_37, 0.000_01)),
            Mo => Some(uncertain!(95.95, 0.01)),
            Ru => Some(uncertain!(101.07, 0.02)),
            #[cfg(feature = "nist")]
            Rh => Some(uncertain!(102.905_50, 0.000_02)),
            #[cfg(feature = "iupac")]
            Rh => Some(uncertain!(102.905_49, 0.000_02)),
            Pd => Some(uncertain!(106.42, 0.01)),
            Ag => Some(uncertain!(107.868_2, 0.000_2)),
            Cd => Some(uncertain!(112.414, 0.004)),
            In => Some(uncertain!(114.818, 0.001)),
            Sn => Some(uncertain!(118.710, 0.007)),
            Sb => Some(uncertain!(121.760, 0.001)),
            Te => Some(uncertain!(127.60, 0.03)),
            I => Some(uncertain!(126.904_47, 0.000_03)),
            Xe => Some(uncertain!(131.293, 0.006)),
            Cs => Some(uncertain!(132.905_451_96, 0.000_000_06)),
            Ba => Some(uncertain!(137.327, 0.007)),
            La => Some(uncertain!(138.905_47, 0.000_07)),
            Ce => Some(uncertain!(140.116, 0.001)),
            #[cfg(feature = "nist")]
            Pr => Some(uncertain!(140.907_66, 0.000_02)),
            #[cfg(feature = "iupac")]
            Pr => Some(uncertain!(140.907_66, 0.000_01)),
            Nd => Some(uncertain!(144.242, 0.003)),
            Sm => Some(uncertain!(150.36, 0.02)),
            Eu => Some(uncertain!(151.964, 0.001)),
            Gd => Some(uncertain!(157.25, 0.03)),
            #[cfg(feature = "nist")]
            Tb => Some(uncertain!(158.925_35, 0.000_02)),
            #[cfg(feature = "iupac")]
            Tb => Some(uncertain!(158.925_354, 0.000_007)),
            Dy => Some(uncertain!(162.500, 0.001)),
            #[cfg(feature = "nist")]
            Ho => Some(uncertain!(164.930_33, 0.000_02)),
            #[cfg(feature = "iupac")]
            Ho => Some(uncertain!(164.930_329, 0.000_005)),
            Er => Some(uncertain!(167.259, 0.003)),
            #[cfg(feature = "nist")]
            Tm => Some(uncertain!(168.934_22, 0.000_02)),
            #[cfg(feature = "iupac")]
            Tm => Some(uncertain!(168.934_219, 0.000_005)),
            #[cfg(feature = "nist")]
            Yb => Some(uncertain!(173.054, 0.005)),
            #[cfg(feature = "iupac")]
            Yb => Some(uncertain!(173.045, 0.010)),
            Lu => Some(uncertain!(174.966_8, 0.000_1)),
            #[cfg(feature = "nist")]
            Hf => Some(uncertain!(178.49, 0.02)),
            #[cfg(feature = "iupac")]
            Hf => Some(uncertain!(178.486, 0.006)),
            Ta => Some(uncertain!(180.947_88, 0.000_02)),
            W => Some(uncertain!(183.84, 0.01)),
            Re => Some(uncertain!(186.207, 0.001)),
            Os => Some(uncertain!(190.23, 0.03)),
            #[cfg(feature = "nist")]
            Ir => Some(uncertain!(192.217, 0.003)),
            #[cfg(feature = "iupac")]
            Ir => Some(uncertain!(192.217, 0.002)),
            Pt => Some(uncertain!(195.084, 0.009)),
            #[cfg(feature = "nist")]
            Au => Some(uncertain!(196.966_569, 0.000_005)),
            #[cfg(feature = "iupac")]
            Au => Some(uncertain!(196.966_570, 0.000_004)),
            Hg => Some(uncertain!(200.592, 0.003)),
            Tl => Some(uncertain!(204.382..204.385)),
            #[cfg(feature = "nist")]
            Pb => Some(uncertain!(207.2, 0.1)),
            #[cfg(feature = "iupac")]
            Pb => Some(uncertain!(206.14..207.94)),
            Bi => Some(uncertain!(208.980_40, 0.000_01)),
            Th => Some(uncertain!(232.037_7, 0.000_4)),
            #[cfg(feature = "nist")]
            Pa => Some(uncertain!(231.035_88, 0.000_02)),
            #[cfg(feature = "iupac")]
            Pa => Some(uncertain!(231.035_88, 0.000_01)),
            U => Some(uncertain!(238.028_91, 0.000_03)),
            _ => None,
        }
    }
}

// Tc None,
// Pm None,
// Po None,
// At None,
// Rn None,
// Fr None,
// Ra None,
// Ac None,
// Np None,
// Pu None,
// /// Mass number of the most stable isotope
// #[cfg(feature = "nist")]
// MostStable(usize),
// #[cfg(feature = "nist")]
// const fn most_stable(value: usize) -> StandardAtomicWeight {
//     StandardAtomicWeight::MostStable(value)
// }
