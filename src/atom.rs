use self::Atom::*;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::Range,
    str::FromStr,
};

pub const ABRIDGED: bool = true;

pub const UNABRIDGED: bool = false;

pub const ATOMS: [Atom; COUNT] = [
    H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca, Sc, Ti, V, Cr, Mn, Fe,
    Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn,
    Sb, Te, I, Xe, Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,
    Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf,
    Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
];

pub const COUNT: usize = 118;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Atom {
    H = 1,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}

impl Atom {
    pub const fn group(&self) -> usize {
        match self {
            H | Li | Na | K | Rb | Cs | Fr => 1,
            Be | Mg | Ca | Sr | Ba | Ra => 2,
            Sc | Y | La | Ce | Pr | Nd | Pm | Sm | Eu | Gd | Tb | Dy | Ho | Er | Tm | Yb | Lu
            | Ac | Th | Pa | U | Np | Pu | Am | Cm | Bk | Cf | Es | Fm | Md | No | Lr => 3,
            Ti | Zr | Hf | Rf => 4,
            V | Nb | Ta | Db => 5,
            Cr | Mo | W | Sg => 6,
            Mn | Tc | Re | Bh => 7,
            Fe | Ru | Os | Hs => 8,
            Co | Rh | Ir | Mt => 9,
            Ni | Pd | Pt | Ds => 10,
            Cu | Ag | Au | Rg => 11,
            Zn | Cd | Hg | Cn => 12,
            B | Al | Ga | In | Tl | Nh => 13,
            C | Si | Ge | Sn | Pb | Fl => 14,
            N | P | As | Sb | Bi | Mc => 15,
            O | S | Se | Te | Po | Lv => 16,
            F | Cl | Br | I | At | Ts => 17,
            He | Ne | Ar | Kr | Xe | Rn | Og => 18,
        }
    }

    /// Standard atomic weight `Ar°(E)`
    pub const fn weight<const ABRIDGED: bool>(&self) -> Option<StandardAtomicWeight<ABRIDGED>> {
        match self {
            // Abridged
            H if ABRIDGED => Some(uncertain(1.0080, 0.0002)),
            He if ABRIDGED => Some(uncertain(4.0026, 0.0001)),
            Li if ABRIDGED => Some(uncertain(6.94, 0.06)),
            Be if ABRIDGED => Some(uncertain(9.0122, 0.0001)),
            B if ABRIDGED => Some(uncertain(10.81, 0.02)),
            C if ABRIDGED => Some(uncertain(12.011, 0.002)),
            N if ABRIDGED => Some(uncertain(14.007, 0.001)),
            O if ABRIDGED => Some(uncertain(15.999, 0.001)),
            F if ABRIDGED => Some(uncertain(18.998, 0.001)),
            Ne if ABRIDGED => Some(uncertain(20.180, 0.001)),
            Na if ABRIDGED => Some(uncertain(22.990, 0.001)),
            Mg if ABRIDGED => Some(uncertain(24.305, 0.002)),
            Al if ABRIDGED => Some(uncertain(26.982, 0.001)),
            Si if ABRIDGED => Some(uncertain(28.085, 0.001)),
            P if ABRIDGED => Some(uncertain(30.974, 0.001)),
            S if ABRIDGED => Some(uncertain(32.06, 0.02)),
            Cl if ABRIDGED => Some(uncertain(35.45, 0.01)),
            Ar if ABRIDGED => Some(uncertain(39.95, 0.16)),
            K if ABRIDGED => Some(uncertain(39.098, 0.001)),
            Ca if ABRIDGED => Some(uncertain(40.078, 0.004)),
            Sc if ABRIDGED => Some(uncertain(44.956, 0.001)),
            Ti if ABRIDGED => Some(uncertain(47.867, 0.001)),
            V if ABRIDGED => Some(uncertain(50.942, 0.001)),
            Cr if ABRIDGED => Some(uncertain(51.996, 0.001)),
            Mn if ABRIDGED => Some(uncertain(54.938, 0.001)),
            Fe if ABRIDGED => Some(uncertain(55.845, 0.002)),
            Co if ABRIDGED => Some(uncertain(58.933, 0.001)),
            Ni if ABRIDGED => Some(uncertain(58.693, 0.001)),
            Cu if ABRIDGED => Some(uncertain(63.546, 0.003)),
            Zn if ABRIDGED => Some(uncertain(65.38, 0.02)),
            Ga if ABRIDGED => Some(uncertain(69.723, 0.001)),
            Ge if ABRIDGED => Some(uncertain(72.630, 0.008)),
            As if ABRIDGED => Some(uncertain(74.922, 0.001)),
            Se if ABRIDGED => Some(uncertain(78.971, 0.008)),
            Br if ABRIDGED => Some(uncertain(79.904, 0.003)),
            Kr if ABRIDGED => Some(uncertain(83.798, 0.002)),
            Rb if ABRIDGED => Some(uncertain(85.468, 0.001)),
            Sr if ABRIDGED => Some(uncertain(87.62, 0.01)),
            Y if ABRIDGED => Some(uncertain(88.906, 0.001)),
            Zr if ABRIDGED => Some(uncertain(91.224, 0.002)),
            Nb if ABRIDGED => Some(uncertain(92.906, 0.001)),
            Mo if ABRIDGED => Some(uncertain(95.95, 0.01)),
            Ru if ABRIDGED => Some(uncertain(101.07, 0.02)),
            Rh if ABRIDGED => Some(uncertain(102.91, 0.01)),
            Pd if ABRIDGED => Some(uncertain(106.42, 0.01)),
            Ag if ABRIDGED => Some(uncertain(107.87, 0.01)),
            Cd if ABRIDGED => Some(uncertain(112.41, 0.01)),
            In if ABRIDGED => Some(uncertain(114.82, 0.01)),
            Sn if ABRIDGED => Some(uncertain(118.71, 0.01)),
            Sb if ABRIDGED => Some(uncertain(121.76, 0.01)),
            Te if ABRIDGED => Some(uncertain(127.60, 0.03)),
            I if ABRIDGED => Some(uncertain(126.90, 0.01)),
            Xe if ABRIDGED => Some(uncertain(131.29, 0.01)),
            Cs if ABRIDGED => Some(uncertain(132.91, 0.01)),
            Ba if ABRIDGED => Some(uncertain(137.33, 0.01)),
            La if ABRIDGED => Some(uncertain(138.91, 0.01)),
            Ce if ABRIDGED => Some(uncertain(140.12, 0.01)),
            Pr if ABRIDGED => Some(uncertain(140.91, 0.01)),
            Nd if ABRIDGED => Some(uncertain(144.24, 0.01)),
            Sm if ABRIDGED => Some(uncertain(150.36, 0.02)),
            Eu if ABRIDGED => Some(uncertain(151.96, 0.01)),
            Gd if ABRIDGED => Some(uncertain(157.25, 0.03)),
            Tb if ABRIDGED => Some(uncertain(158.93, 0.01)),
            Dy if ABRIDGED => Some(uncertain(162.50, 0.01)),
            Ho if ABRIDGED => Some(uncertain(164.93, 0.01)),
            Er if ABRIDGED => Some(uncertain(167.26, 0.01)),
            Tm if ABRIDGED => Some(uncertain(168.93, 0.01)),
            Yb if ABRIDGED => Some(uncertain(173.05, 0.02)),
            Lu if ABRIDGED => Some(uncertain(174.97, 0.01)),
            Hf if ABRIDGED => Some(uncertain(178.49, 0.01)),
            Ta if ABRIDGED => Some(uncertain(180.95, 0.01)),
            W if ABRIDGED => Some(uncertain(183.84, 0.01)),
            Re if ABRIDGED => Some(uncertain(186.21, 0.01)),
            Os if ABRIDGED => Some(uncertain(190.23, 0.03)),
            Ir if ABRIDGED => Some(uncertain(192.22, 0.01)),
            Pt if ABRIDGED => Some(uncertain(195.08, 0.02)),
            Au if ABRIDGED => Some(uncertain(196.97, 0.01)),
            Hg if ABRIDGED => Some(uncertain(200.59, 0.01)),
            Tl if ABRIDGED => Some(uncertain(204.38, 0.01)),
            Pb if ABRIDGED => Some(uncertain(207.2, 1.1)),
            Bi if ABRIDGED => Some(uncertain(208.98, 0.01)),
            Th if ABRIDGED => Some(uncertain(232.04, 0.01)),
            Pa if ABRIDGED => Some(uncertain(231.04, 0.01)),
            U if ABRIDGED => Some(uncertain(238.03, 0.01)),
            H => Some(interval(1.00784..1.00811)),
            He => Some(uncertain(4.002602, 0.000002)),
            Li => Some(interval(6.938..6.997)),
            Be => Some(uncertain(9.0121831, 0.0000005)),
            B => Some(interval(10.806..10.821)),
            C => Some(interval(12.0096..12.0116)),
            N => Some(interval(14.00643..14.00728)),
            O => Some(interval(15.99903..15.99977)),
            F => Some(uncertain(18.998403162, 0.000000005)),
            Ne => Some(uncertain(20.1797, 0.0006)),
            Na => Some(uncertain(22.98976928, 0.00000002)),
            Mg => Some(interval(24.304..24.307)),
            Al => Some(uncertain(26.9815384, 0.0000003)),
            Si => Some(interval(28.084..28.086)),
            P => Some(uncertain(30.973761998, 0.000000005)),
            S => Some(interval(32.059..32.076)),
            Cl => Some(interval(35.446..35.457)),
            Ar => Some(interval(39.792..39.963)),
            K => Some(uncertain(39.0983, 0.0001)),
            Ca => Some(uncertain(40.078, 0.004)),
            Sc => Some(uncertain(44.955907, 0.000004)),
            Ti => Some(uncertain(47.867, 0.001)),
            V => Some(uncertain(50.9415, 0.0001)),
            Cr => Some(uncertain(51.9961, 0.0006)),
            Mn => Some(uncertain(54.938043, 0.000002)),
            Fe => Some(uncertain(55.845, 0.002)),
            Co => Some(uncertain(58.933194, 0.000003)),
            Ni => Some(uncertain(58.6934, 0.0004)),
            Cu => Some(uncertain(63.546, 0.003)),
            Zn => Some(uncertain(65.38, 0.02)),
            Ga => Some(uncertain(69.723, 0.001)),
            Ge => Some(uncertain(72.630, 0.008)),
            As => Some(uncertain(74.921595, 0.000006)),
            Se => Some(uncertain(78.971, 0.008)),
            Br => Some(interval(79.901..79.907)),
            Kr => Some(uncertain(83.798, 0.002)),
            Rb => Some(uncertain(85.4678, 0.0003)),
            Sr => Some(uncertain(87.62, 0.01)),
            Y => Some(uncertain(88.905838, 0.000002)),
            Zr => Some(uncertain(91.224, 0.002)),
            Nb => Some(uncertain(92.90637, 0.00001)),
            Mo => Some(uncertain(95.95, 0.01)),
            Ru => Some(uncertain(101.07, 0.02)),
            Rh => Some(uncertain(102.90549, 0.00002)),
            Pd => Some(uncertain(106.42, 0.01)),
            Ag => Some(uncertain(107.8682, 0.0002)),
            Cd => Some(uncertain(112.414, 0.004)),
            In => Some(uncertain(114.818, 0.001)),
            Sn => Some(uncertain(118.710, 0.007)),
            Sb => Some(uncertain(121.760, 0.001)),
            Te => Some(uncertain(127.60, 0.03)),
            I => Some(uncertain(126.90447, 0.00003)),
            Xe => Some(uncertain(131.293, 0.006)),
            Cs => Some(uncertain(132.90545196, 0.00000006)),
            Ba => Some(uncertain(137.327, 0.007)),
            La => Some(uncertain(138.90547, 0.00007)),
            Ce => Some(uncertain(140.116, 0.001)),
            Pr => Some(uncertain(140.90766, 0.00001)),
            Nd => Some(uncertain(144.242, 0.003)),
            Sm => Some(uncertain(150.36, 0.02)),
            Eu => Some(uncertain(151.964, 0.001)),
            Gd => Some(uncertain(157.25, 0.03)),
            Tb => Some(uncertain(158.925354, 0.000007)),
            Dy => Some(uncertain(162.500, 0.001)),
            Ho => Some(uncertain(164.930329, 0.000005)),
            Er => Some(uncertain(167.259, 0.003)),
            Tm => Some(uncertain(168.934219, 0.000005)),
            Yb => Some(uncertain(173.045, 0.010)),
            Lu => Some(uncertain(174.9668, 0.0001)),
            Hf => Some(uncertain(178.486, 0.006)),
            Ta => Some(uncertain(180.94788, 0.00002)),
            W => Some(uncertain(183.84, 0.01)),
            Re => Some(uncertain(186.207, 0.001)),
            Os => Some(uncertain(190.23, 0.03)),
            Ir => Some(uncertain(192.217, 0.002)),
            Pt => Some(uncertain(195.084, 0.009)),
            Au => Some(uncertain(196.966570, 0.000004)),
            Hg => Some(uncertain(200.592, 0.003)),
            Tl => Some(interval(204.382..204.385)),
            Pb => Some(interval(206.14..207.94)),
            Bi => Some(uncertain(208.98040, 0.00001)),
            Th => Some(uncertain(232.0377, 0.0004)),
            Pa => Some(uncertain(231.03588, 0.00001)),
            U => Some(uncertain(238.02891, 0.00003)),
            _ => None,
        }
    }

    /// Element
    pub const fn name(&self) -> &'static str {
        match self {
            H => "Hydrogen",
            He => "Helium",
            Li => "Lithium",
            Be => "Beryllium",
            B => "Boron",
            C => "Carbon",
            N => "Nitrogen",
            O => "Oxygen",
            F => "Fluorine",
            Ne => "Neon",
            Na => "Sodium",
            Mg => "Magnesium",
            Al => "Aluminium",
            Si => "Silicon",
            P => "Phosphorus",
            S => "Sulfur",
            Cl => "Chlorine",
            Ar => "Argon",
            K => "Potassium",
            Ca => "Calcium",
            Sc => "Scandium",
            Ti => "Titanium",
            V => "Vanadium",
            Cr => "Chromium",
            Mn => "Manganese",
            Fe => "Iron",
            Co => "Cobalt",
            Ni => "Nickel",
            Cu => "Copper",
            Zn => "Zinc",
            Ga => "Gallium",
            Ge => "Germanium",
            As => "Arsenic",
            Se => "Selenium",
            Br => "Bromine",
            Kr => "Krypton",
            Rb => "Rubidium",
            Sr => "Strontium",
            Y => "Yttrium",
            Zr => "Zirconium",
            Nb => "Niobium",
            Mo => "Molybdenum",
            Tc => "Technetium",
            Ru => "Ruthenium",
            Rh => "Rhodium",
            Pd => "Palladium",
            Ag => "Silver",
            Cd => "Cadmium",
            In => "Indium",
            Sn => "Tin",
            Sb => "Antimony",
            Te => "Tellurium",
            I => "Iodine",
            Xe => "Xenon",
            Cs => "Caesium",
            Ba => "Barium",
            La => "Lanthanum",
            Ce => "Cerium",
            Pr => "Praseodymium",
            Nd => "Neodymium",
            Pm => "Promethium",
            Sm => "Samarium",
            Eu => "Europium",
            Gd => "Gadolinium",
            Tb => "Terbium",
            Dy => "Dysprosium",
            Ho => "Holmium",
            Er => "Erbium",
            Tm => "Thulium",
            Yb => "Ytterbium",
            Lu => "Lutetium",
            Hf => "Hafnium",
            Ta => "Tantalum",
            W => "Tungsten",
            Re => "Rhenium",
            Os => "Osmium",
            Ir => "Iridium",
            Pt => "Platinum",
            Au => "Gold",
            Hg => "Mercury",
            Tl => "Thallium",
            Pb => "Lead",
            Bi => "Bismuth",
            Po => "Polonium",
            At => "Astatine",
            Rn => "Radon",
            Fr => "Francium",
            Ra => "Radium",
            Ac => "Actinium",
            Th => "Thorium",
            Pa => "Protactinium",
            U => "Uranium",
            Np => "Neptunium",
            Pu => "Plutonium",
            Am => "Americium",
            Cm => "Curium",
            Bk => "Berkelium",
            Cf => "Californium",
            Es => "Einsteinium",
            Fm => "Fermium",
            Md => "Mendelevium",
            No => "Nobelium",
            Lr => "Lawrencium",
            Rf => "Rutherfordium",
            Db => "Dubnium",
            Sg => "Seaborgium",
            Bh => "Bohrium",
            Hs => "Hassium",
            Mt => "Meitnerium",
            Ds => "Darmstadtium",
            Rg => "Roentgenium",
            Cn => "Copernicium",
            Nh => "Nihonium",
            Fl => "Flerovium",
            Mc => "Moscovium",
            Lv => "Livermorium",
            Ts => "Tennessine",
            Og => "Oganesson ",
        }
    }

    /// Atomic number
    pub const fn number(&self) -> usize {
        *self as usize
    }

    pub const fn period(&self) -> usize {
        match self {
            H | He => 1,
            Li | Be | B | C | N | O | F | Ne => 2,
            Na | Mg | Al | Si | P | S | Cl | Ar => 3,
            K | Ca | Sc | Ti | V | Cr | Mn | Fe | Co | Ni | Cu | Zn | Ga | Ge | As | Se | Br
            | Kr => 4,
            Rb | Sr | Y | Zr | Nb | Mo | Tc | Ru | Rh | Pd | Ag | Cd | In | Sn | Sb | Te | I
            | Xe => 5,
            Cs | Ba | La | Ce | Pr | Nd | Pm | Sm | Eu | Gd | Tb | Dy | Ho | Er | Tm | Yb | Lu
            | Hf | Ta | W | Re | Os | Ir | Pt | Au | Hg | Tl | Pb | Bi | Po | At | Rn => 6,
            Fr | Ra | Ac | Th | Pa | U | Np | Pu | Am | Cm | Bk | Cf | Es | Fm | Md | No | Lr
            | Rf | Db | Sg | Bh | Hs | Mt | Ds | Rg | Cn | Nh | Fl | Mc | Lv | Ts | Og => 7,
        }
    }

    pub fn split(&self) -> (&[Self], &[Self]) {
        let split = ATOMS.split_at(*self as _);
        (split.0, &split.1[1..])
    }

    /// Symbol
    pub const fn symbol(&self) -> &'static str {
        match self {
            H => "H",
            He => "He",
            Li => "Li",
            Be => "Be",
            B => "B",
            C => "C",
            N => "N",
            O => "O",
            F => "F",
            Ne => "Ne",
            Na => "Na",
            Mg => "Mg",
            Al => "Al",
            Si => "Si",
            P => "P",
            S => "S",
            Cl => "Cl",
            Ar => "Ar",
            K => "K",
            Ca => "Ca",
            Sc => "Sc",
            Ti => "Ti",
            V => "V",
            Cr => "Cr",
            Mn => "Mn",
            Fe => "Fe",
            Co => "Co",
            Ni => "Ni",
            Cu => "Cu",
            Zn => "Zn",
            Ga => "Ga",
            Ge => "Ge",
            As => "As",
            Se => "Se",
            Br => "Br",
            Kr => "Kr",
            Rb => "Rb",
            Sr => "Sr",
            Y => "Y",
            Zr => "Zr",
            Nb => "Nb",
            Mo => "Mo",
            Tc => "Tc",
            Ru => "Ru",
            Rh => "Rh",
            Pd => "Pd",
            Ag => "Ag",
            Cd => "Cd",
            In => "In",
            Sn => "Sn",
            Sb => "Sb",
            Te => "Te",
            I => "I",
            Xe => "Xe",
            Cs => "Cs",
            Ba => "Ba",
            La => "La",
            Ce => "Ce",
            Pr => "Pr",
            Nd => "Nd",
            Pm => "Pm",
            Sm => "Sm",
            Eu => "Eu",
            Gd => "Gd",
            Tb => "Tb",
            Dy => "Dy",
            Ho => "Ho",
            Er => "Er",
            Tm => "Tm",
            Yb => "Yb",
            Lu => "Lu",
            Hf => "Hf",
            Ta => "Ta",
            W => "W",
            Re => "Re",
            Os => "Os",
            Ir => "Ir",
            Pt => "Pt",
            Au => "Au",
            Hg => "Hg",
            Tl => "Tl",
            Pb => "Pb",
            Bi => "Bi",
            Po => "Po",
            At => "At",
            Rn => "Rn",
            Fr => "Fr",
            Ra => "Ra",
            Ac => "Ac",
            Th => "Th",
            Pa => "Pa",
            U => "U",
            Np => "Np",
            Pu => "Pu",
            Am => "Am",
            Cm => "Cm",
            Bk => "Bk",
            Cf => "Cf",
            Es => "Es",
            Fm => "Fm",
            Md => "Md",
            No => "No",
            Lr => "Lr",
            Rf => "Rf",
            Db => "Db",
            Sg => "Sg",
            Bh => "Bh",
            Hs => "Hs",
            Mt => "Mt",
            Ds => "Ds",
            Rg => "Rg",
            Cn => "Cn",
            Nh => "Nh",
            Fl => "Fl",
            Mc => "Mc",
            Lv => "Lv",
            Ts => "Ts",
            Og => "Og",
        }
    }

    pub const fn valency(&self) -> &[usize] {
        match self {
            H => &[1],
            He => &[0],
            Li => &[1],
            Be => &[2],
            B => &[3],
            C => &[4, 2],
            N => &[1, 2, 3, 4],
            O => &[2],
            F => &[1],
            Ne => &[0],
            Na => &[1],
            Mg => &[2],
            Al => &[3],
            Si => &[4],
            P => &[3, 5],
            S => &[6, 4, 2],
            Cl => &[1, 3, 5, 7],
            Ar => &[0],
            K => &[1],
            Ca => &[2],
            Sc => &[3],
            Ti => &[2, 3, 4],
            V => &[2, 3, 4, 5],
            Cr => &[2, 3, 6],
            Mn => &[2, 3, 4, 6, 7],
            Fe => &[2, 3],
            Co => &[2, 3],
            Ni => &[2, 3, 4],
            Cu => &[1, 2],
            Zn => &[2],
            Ga => &[3],
            Ge => &[2, 4],
            As => &[3, 5],
            Se => &[2, 4, 6],
            Br => &[1, 3, 5, 7],
            Kr => &[6, 4, 2],
            Rb => &[1],
            Sr => &[2],
            Y => &[3],
            Zr => &[2, 3, 4],
            Nb => &[1, 2, 3, 4, 5],
            Mo => &[2, 3, 4, 5, 6],
            Tc => &[1, 2, 3, 4, 5, 6, 7],
            Ru => &[2, 3, 4, 5, 6, 7, 8],
            Rh => &[1, 2, 3, 4, 5],
            Pd => &[1, 2, 3, 4],
            Ag => &[1, 2, 3],
            Cd => &[2],
            In => &[3],
            Sn => &[2, 4],
            Sb => &[3, 5],
            Te => &[6, 4, 2],
            I => &[1, 3, 5, 7],
            Xe => &[2, 4, 6, 8],
            Cs => &[1],
            Ba => &[2],
            La => &[3],
            Ce => &[3, 4],
            Pr => &[3, 4],
            Nd => &[3],
            Pm => &[3],
            Sm => &[2, 3],
            Eu => &[2, 3],
            Gd => &[3],
            Tb => &[3, 4],
            Dy => &[3],
            Ho => &[3],
            Er => &[3],
            Tm => &[2, 3],
            Yb => &[2, 3],
            Lu => &[3],
            Hf => &[2, 3, 4],
            Ta => &[1, 2, 3, 4, 5],
            W => &[2, 3, 4, 5, 6],
            Re => &[1, 2, 3, 4, 5, 6, 7],
            Os => &[2, 3, 4, 5, 6, 8],
            Ir => &[1, 2, 3, 4, 5, 6],
            Pt => &[1, 2, 3, 4, 5],
            Au => &[1, 2, 3],
            Hg => &[2],
            Tl => &[1, 3],
            Pb => &[2, 4],
            Bi => &[3, 5],
            Po => &[6, 4, 2],
            At => &[],
            Rn => &[0],
            Fr => &[1],
            Ra => &[2],
            Ac => &[3],
            Th => &[2, 3, 4],
            Pa => &[4, 5],
            U => &[3, 4],
            Np => &[3, 4, 5, 6],
            Pu => &[2, 3, 4],
            Am => &[3, 4, 5, 6],
            Cm => &[3, 4],
            Bk => &[3, 4],
            Cf => &[2, 3, 4],
            Es => &[2, 3],
            Fm => &[2, 3],
            Md => &[2, 3],
            No => &[2, 3],
            Lr => &[3],
            Rf => &[],
            Db => &[],
            Sg => &[],
            Bh => &[],
            Hs => &[],
            Mt => &[],
            Ds => &[],
            Rg => &[],
            Cn => &[],
            Nh => &[],
            Fl => &[],
            Mc => &[],
            Lv => &[],
            Ts => &[],
            Og => &[],
        }
    }
}

impl AsRef<str> for Atom {
    fn as_ref(&self) -> &'static str {
        self.symbol()
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Atom {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "H" => Ok(H),
            "He" => Ok(He),
            "Li" => Ok(Li),
            "Be" => Ok(Be),
            "B" => Ok(B),
            "C" => Ok(C),
            "N" => Ok(N),
            "O" => Ok(O),
            "F" => Ok(F),
            "Ne" => Ok(Ne),
            "Na" => Ok(Na),
            "Mg" => Ok(Mg),
            "Al" => Ok(Al),
            "Si" => Ok(Si),
            "P" => Ok(P),
            "S" => Ok(S),
            "Cl" => Ok(Cl),
            "Ar" => Ok(Ar),
            "K" => Ok(K),
            "Ca" => Ok(Ca),
            "Sc" => Ok(Sc),
            "Ti" => Ok(Ti),
            "V" => Ok(V),
            "Cr" => Ok(Cr),
            "Mn" => Ok(Mn),
            "Fe" => Ok(Fe),
            "Co" => Ok(Co),
            "Ni" => Ok(Ni),
            "Cu" => Ok(Cu),
            "Zn" => Ok(Zn),
            "Ga" => Ok(Ga),
            "Ge" => Ok(Ge),
            "As" => Ok(As),
            "Se" => Ok(Se),
            "Br" => Ok(Br),
            "Kr" => Ok(Kr),
            "Rb" => Ok(Rb),
            "Sr" => Ok(Sr),
            "Y" => Ok(Y),
            "Zr" => Ok(Zr),
            "Nb" => Ok(Nb),
            "Mo" => Ok(Mo),
            "Tc" => Ok(Tc),
            "Ru" => Ok(Ru),
            "Rh" => Ok(Rh),
            "Pd" => Ok(Pd),
            "Ag" => Ok(Ag),
            "Cd" => Ok(Cd),
            "In" => Ok(In),
            "Sn" => Ok(Sn),
            "Sb" => Ok(Sb),
            "Te" => Ok(Te),
            "I" => Ok(I),
            "Xe" => Ok(Xe),
            "Cs" => Ok(Cs),
            "Ba" => Ok(Ba),
            "La" => Ok(La),
            "Ce" => Ok(Ce),
            "Pr" => Ok(Pr),
            "Nd" => Ok(Nd),
            "Pm" => Ok(Pm),
            "Sm" => Ok(Sm),
            "Eu" => Ok(Eu),
            "Gd" => Ok(Gd),
            "Tb" => Ok(Tb),
            "Dy" => Ok(Dy),
            "Ho" => Ok(Ho),
            "Er" => Ok(Er),
            "Tm" => Ok(Tm),
            "Yb" => Ok(Yb),
            "Lu" => Ok(Lu),
            "Hf" => Ok(Hf),
            "Ta" => Ok(Ta),
            "W" => Ok(W),
            "Re" => Ok(Re),
            "Os" => Ok(Os),
            "Ir" => Ok(Ir),
            "Pt" => Ok(Pt),
            "Au" => Ok(Au),
            "Hg" => Ok(Hg),
            "Tl" => Ok(Tl),
            "Pb" => Ok(Pb),
            "Bi" => Ok(Bi),
            "Po" => Ok(Po),
            "At" => Ok(At),
            "Rn" => Ok(Rn),
            "Fr" => Ok(Fr),
            "Ra" => Ok(Ra),
            "Ac" => Ok(Ac),
            "Th" => Ok(Th),
            "Pa" => Ok(Pa),
            "U" => Ok(U),
            "Np" => Ok(Np),
            "Pu" => Ok(Pu),
            "Am" => Ok(Am),
            "Cm" => Ok(Cm),
            "Bk" => Ok(Bk),
            "Cf" => Ok(Cf),
            "Es" => Ok(Es),
            "Fm" => Ok(Fm),
            "Md" => Ok(Md),
            "No" => Ok(No),
            "Lr" => Ok(Lr),
            "Rf" => Ok(Rf),
            "Db" => Ok(Db),
            "Sg" => Ok(Sg),
            "Bh" => Ok(Bh),
            "Hs" => Ok(Hs),
            "Mt" => Ok(Mt),
            "Ds" => Ok(Ds),
            "Rg" => Ok(Rg),
            "Cn" => Ok(Cn),
            "Nh" => Ok(Nh),
            "Fl" => Ok(Fl),
            "Mc" => Ok(Mc),
            "Lv" => Ok(Lv),
            "Ts" => Ok(Ts),
            "Og" => Ok(Og),
            _ => Err(Error::Parse),
        }
    }
}

/// Standard atomic weight
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StandardAtomicWeight<const ABRIDGED: bool = false> {
    Interval(Range<f64>),
    Uncertain { value: f64, uncertainty: f64 },
}

impl<const ABRIDGED: bool> StandardAtomicWeight<ABRIDGED> {
    pub const fn minimum(&self) -> f64 {
        match self {
            Self::Interval(interval) => interval.start,
            Self::Uncertain { value, uncertainty } => *value - *uncertainty,
        }
    }

    pub const fn average(&self) -> f64 {
        match self {
            Self::Interval(interval) => (interval.start + interval.end) / 2.0,
            Self::Uncertain { value, .. } => *value,
        }
    }

    pub const fn maximum(&self) -> f64 {
        match self {
            Self::Interval(interval) => interval.end,
            Self::Uncertain { value, uncertainty } => *value + *uncertainty,
        }
    }
}

// impl<const ABRIDGED: bool> From<StandardAtomicWeight<ABRIDGED>> for Range<f64> {
//     fn from(value: StandardAtomicWeight<ABRIDGED>) -> Self {
//         value.minimum()..value.maximum()
//     }
// }

impl<const ABRIDGED: bool> Display for StandardAtomicWeight<ABRIDGED> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Interval(Range { start, end }) => match f.precision() {
                Some(precision) => write!(f, "[{start:.0$}, {end:.0$}]", precision),
                None => write!(f, "[{start}, {end}]"),
            },
            Self::Uncertain { value, uncertainty } => match f.precision() {
                Some(precision) => write!(f, "{value:.0$}±{uncertainty:.0$}", precision),
                None => write!(f, "{value}±{uncertainty}"),
            },
        }
    }
}

impl<const ABRIDGED: bool> PartialEq for StandardAtomicWeight<ABRIDGED> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Interval(Range {
                    start: start1,
                    end: end1,
                }),
                Self::Interval(Range {
                    start: start2,
                    end: end2,
                }),
            ) => (start1 - start2).abs() < f64::EPSILON && (end1 - end2).abs() < f64::EPSILON,
            (
                Self::Uncertain {
                    value: value1,
                    uncertainty: uncertainty1,
                },
                Self::Uncertain {
                    value: value2,
                    uncertainty: uncertainty2,
                },
            ) => {
                (value1 - value2).abs() < f64::EPSILON
                    && (uncertainty1 - uncertainty2).abs() < f64::EPSILON
            }
            _ => false,
        }
    }
}

const fn interval<const ABRIDGED: bool>(range: Range<f64>) -> StandardAtomicWeight<ABRIDGED> {
    StandardAtomicWeight::Interval(range)
}

const fn uncertain<const ABRIDGED: bool>(
    value: f64,
    uncertainty: f64,
) -> StandardAtomicWeight<ABRIDGED> {
    StandardAtomicWeight::Uncertain { value, uncertainty }
}

#[test]
fn abridged() {
    assert_eq!(H.weight::<ABRIDGED>(), Some(uncertain(1.0080, 0.0002)));
    assert_eq!(He.weight::<ABRIDGED>(), Some(uncertain(4.0026, 0.0001)));
    assert_eq!(Li.weight::<ABRIDGED>(), Some(uncertain(6.94, 0.06)));
    assert_eq!(Be.weight::<ABRIDGED>(), Some(uncertain(9.0122, 0.0001)));
    assert_eq!(B.weight::<ABRIDGED>(), Some(uncertain(10.81, 0.02)));
    assert_eq!(C.weight::<ABRIDGED>(), Some(uncertain(12.011, 0.002)));
    assert_eq!(N.weight::<ABRIDGED>(), Some(uncertain(14.007, 0.001)));
    assert_eq!(O.weight::<ABRIDGED>(), Some(uncertain(15.999, 0.001)));
    assert_eq!(F.weight::<ABRIDGED>(), Some(uncertain(18.998, 0.001)));
    assert_eq!(Ne.weight::<ABRIDGED>(), Some(uncertain(20.180, 0.001)));
    assert_eq!(Na.weight::<ABRIDGED>(), Some(uncertain(22.990, 0.001)));
    assert_eq!(Mg.weight::<ABRIDGED>(), Some(uncertain(24.305, 0.002)));
    assert_eq!(Al.weight::<ABRIDGED>(), Some(uncertain(26.982, 0.001)));
    assert_eq!(Si.weight::<ABRIDGED>(), Some(uncertain(28.085, 0.001)));
    assert_eq!(P.weight::<ABRIDGED>(), Some(uncertain(30.974, 0.001)));
    assert_eq!(S.weight::<ABRIDGED>(), Some(uncertain(32.06, 0.02)));
    assert_eq!(Cl.weight::<ABRIDGED>(), Some(uncertain(35.45, 0.01)));
    assert_eq!(Ar.weight::<ABRIDGED>(), Some(uncertain(39.95, 0.16)));
    assert_eq!(K.weight::<ABRIDGED>(), Some(uncertain(39.098, 0.001)));
    assert_eq!(Ca.weight::<ABRIDGED>(), Some(uncertain(40.078, 0.004)));
    assert_eq!(Sc.weight::<ABRIDGED>(), Some(uncertain(44.956, 0.001)));
    assert_eq!(Ti.weight::<ABRIDGED>(), Some(uncertain(47.867, 0.001)));
    assert_eq!(V.weight::<ABRIDGED>(), Some(uncertain(50.942, 0.001)));
    assert_eq!(Cr.weight::<ABRIDGED>(), Some(uncertain(51.996, 0.001)));
    assert_eq!(Mn.weight::<ABRIDGED>(), Some(uncertain(54.938, 0.001)));
    assert_eq!(Fe.weight::<ABRIDGED>(), Some(uncertain(55.845, 0.002)));
    assert_eq!(Co.weight::<ABRIDGED>(), Some(uncertain(58.933, 0.001)));
    assert_eq!(Ni.weight::<ABRIDGED>(), Some(uncertain(58.693, 0.001)));
    assert_eq!(Cu.weight::<ABRIDGED>(), Some(uncertain(63.546, 0.003)));
    assert_eq!(Zn.weight::<ABRIDGED>(), Some(uncertain(65.38, 0.02)));
    assert_eq!(Ga.weight::<ABRIDGED>(), Some(uncertain(69.723, 0.001)));
    assert_eq!(Ge.weight::<ABRIDGED>(), Some(uncertain(72.630, 0.008)));
    assert_eq!(As.weight::<ABRIDGED>(), Some(uncertain(74.922, 0.001)));
    assert_eq!(Se.weight::<ABRIDGED>(), Some(uncertain(78.971, 0.008)));
    assert_eq!(Br.weight::<ABRIDGED>(), Some(uncertain(79.904, 0.003)));
    assert_eq!(Kr.weight::<ABRIDGED>(), Some(uncertain(83.798, 0.002)));
    assert_eq!(Rb.weight::<ABRIDGED>(), Some(uncertain(85.468, 0.001)));
    assert_eq!(Sr.weight::<ABRIDGED>(), Some(uncertain(87.62, 0.01)));
    assert_eq!(Y.weight::<ABRIDGED>(), Some(uncertain(88.906, 0.001)));
    assert_eq!(Zr.weight::<ABRIDGED>(), Some(uncertain(91.224, 0.002)));
    assert_eq!(Nb.weight::<ABRIDGED>(), Some(uncertain(92.906, 0.001)));
    assert_eq!(Mo.weight::<ABRIDGED>(), Some(uncertain(95.95, 0.01)));
    assert_eq!(Tc.weight::<ABRIDGED>(), None);
    assert_eq!(Ru.weight::<ABRIDGED>(), Some(uncertain(101.07, 0.02)));
    assert_eq!(Rh.weight::<ABRIDGED>(), Some(uncertain(102.91, 0.01)));
    assert_eq!(Pd.weight::<ABRIDGED>(), Some(uncertain(106.42, 0.01)));
    assert_eq!(Ag.weight::<ABRIDGED>(), Some(uncertain(107.87, 0.01)));
    assert_eq!(Cd.weight::<ABRIDGED>(), Some(uncertain(112.41, 0.01)));
    assert_eq!(In.weight::<ABRIDGED>(), Some(uncertain(114.82, 0.01)));
    assert_eq!(Sn.weight::<ABRIDGED>(), Some(uncertain(118.71, 0.01)));
    assert_eq!(Sb.weight::<ABRIDGED>(), Some(uncertain(121.76, 0.01)));
    assert_eq!(Te.weight::<ABRIDGED>(), Some(uncertain(127.60, 0.03)));
    assert_eq!(I.weight::<ABRIDGED>(), Some(uncertain(126.90, 0.01)));
    assert_eq!(Xe.weight::<ABRIDGED>(), Some(uncertain(131.29, 0.01)));
    assert_eq!(Cs.weight::<ABRIDGED>(), Some(uncertain(132.91, 0.01)));
    assert_eq!(Ba.weight::<ABRIDGED>(), Some(uncertain(137.33, 0.01)));
    assert_eq!(La.weight::<ABRIDGED>(), Some(uncertain(138.91, 0.01)));
    assert_eq!(Ce.weight::<ABRIDGED>(), Some(uncertain(140.12, 0.01)));
    assert_eq!(Pr.weight::<ABRIDGED>(), Some(uncertain(140.91, 0.01)));
    assert_eq!(Nd.weight::<ABRIDGED>(), Some(uncertain(144.24, 0.01)));
    assert_eq!(Pm.weight::<ABRIDGED>(), None);
    assert_eq!(Sm.weight::<ABRIDGED>(), Some(uncertain(150.36, 0.02)));
    assert_eq!(Eu.weight::<ABRIDGED>(), Some(uncertain(151.96, 0.01)));
    assert_eq!(Gd.weight::<ABRIDGED>(), Some(uncertain(157.25, 0.03)));
    assert_eq!(Tb.weight::<ABRIDGED>(), Some(uncertain(158.93, 0.01)));
    assert_eq!(Dy.weight::<ABRIDGED>(), Some(uncertain(162.50, 0.01)));
    assert_eq!(Ho.weight::<ABRIDGED>(), Some(uncertain(164.93, 0.01)));
    assert_eq!(Er.weight::<ABRIDGED>(), Some(uncertain(167.26, 0.01)));
    assert_eq!(Tm.weight::<ABRIDGED>(), Some(uncertain(168.93, 0.01)));
    assert_eq!(Yb.weight::<ABRIDGED>(), Some(uncertain(173.05, 0.02)));
    assert_eq!(Lu.weight::<ABRIDGED>(), Some(uncertain(174.97, 0.01)));
    assert_eq!(Hf.weight::<ABRIDGED>(), Some(uncertain(178.49, 0.01)));
    assert_eq!(Ta.weight::<ABRIDGED>(), Some(uncertain(180.95, 0.01)));
    assert_eq!(W.weight::<ABRIDGED>(), Some(uncertain(183.84, 0.01)));
    assert_eq!(Re.weight::<ABRIDGED>(), Some(uncertain(186.21, 0.01)));
    assert_eq!(Os.weight::<ABRIDGED>(), Some(uncertain(190.23, 0.03)));
    assert_eq!(Ir.weight::<ABRIDGED>(), Some(uncertain(192.22, 0.01)));
    assert_eq!(Pt.weight::<ABRIDGED>(), Some(uncertain(195.08, 0.02)));
    assert_eq!(Au.weight::<ABRIDGED>(), Some(uncertain(196.97, 0.01)));
    assert_eq!(Hg.weight::<ABRIDGED>(), Some(uncertain(200.59, 0.01)));
    assert_eq!(Tl.weight::<ABRIDGED>(), Some(uncertain(204.38, 0.01)));
    assert_eq!(Pb.weight::<ABRIDGED>(), Some(uncertain(207.2, 1.1)));
    assert_eq!(Bi.weight::<ABRIDGED>(), Some(uncertain(208.98, 0.01)));
    assert_eq!(Po.weight::<ABRIDGED>(), None);
    assert_eq!(At.weight::<ABRIDGED>(), None);
    assert_eq!(Rn.weight::<ABRIDGED>(), None);
    assert_eq!(Fr.weight::<ABRIDGED>(), None);
    assert_eq!(Ra.weight::<ABRIDGED>(), None);
    assert_eq!(Ac.weight::<ABRIDGED>(), None);
    assert_eq!(Th.weight::<ABRIDGED>(), Some(uncertain(232.04, 0.01)));
    assert_eq!(Pa.weight::<ABRIDGED>(), Some(uncertain(231.04, 0.01)));
    assert_eq!(U.weight::<ABRIDGED>(), Some(uncertain(238.03, 0.01)));
    assert_eq!(Np.weight::<ABRIDGED>(), None);
    assert_eq!(Pu.weight::<ABRIDGED>(), None);
    assert_eq!(Am.weight::<ABRIDGED>(), None);
    assert_eq!(Cm.weight::<ABRIDGED>(), None);
    assert_eq!(Bk.weight::<ABRIDGED>(), None);
    assert_eq!(Cf.weight::<ABRIDGED>(), None);
    assert_eq!(Es.weight::<ABRIDGED>(), None);
    assert_eq!(Fm.weight::<ABRIDGED>(), None);
    assert_eq!(Md.weight::<ABRIDGED>(), None);
    assert_eq!(No.weight::<ABRIDGED>(), None);
    assert_eq!(Lr.weight::<ABRIDGED>(), None);
    assert_eq!(Rf.weight::<ABRIDGED>(), None);
    assert_eq!(Db.weight::<ABRIDGED>(), None);
    assert_eq!(Sg.weight::<ABRIDGED>(), None);
    assert_eq!(Bh.weight::<ABRIDGED>(), None);
    assert_eq!(Hs.weight::<ABRIDGED>(), None);
    assert_eq!(Mt.weight::<ABRIDGED>(), None);
    assert_eq!(Ds.weight::<ABRIDGED>(), None);
    assert_eq!(Rg.weight::<ABRIDGED>(), None);
    assert_eq!(Cn.weight::<ABRIDGED>(), None);
    assert_eq!(Nh.weight::<ABRIDGED>(), None);
    assert_eq!(Fl.weight::<ABRIDGED>(), None);
    assert_eq!(Mc.weight::<ABRIDGED>(), None);
    assert_eq!(Lv.weight::<ABRIDGED>(), None);
    assert_eq!(Ts.weight::<ABRIDGED>(), None);
    assert_eq!(Og.weight::<ABRIDGED>(), None);
}

#[test]
fn unabridged() {
    assert_eq!(H.weight::<UNABRIDGED>(), Some(interval(1.00784..1.00811)));
    assert_eq!(
        He.weight::<UNABRIDGED>(),
        Some(uncertain(4.002602, 0.000002))
    );
    assert_eq!(Li.weight::<UNABRIDGED>(), Some(interval(6.938..6.997)));
    assert_eq!(
        Be.weight::<UNABRIDGED>(),
        Some(uncertain(9.0121831, 0.0000005))
    );
    assert_eq!(B.weight::<UNABRIDGED>(), Some(interval(10.806..10.821)));
    assert_eq!(C.weight::<UNABRIDGED>(), Some(interval(12.0096..12.0116)));
    assert_eq!(N.weight::<UNABRIDGED>(), Some(interval(14.00643..14.00728)));
    assert_eq!(O.weight::<UNABRIDGED>(), Some(interval(15.99903..15.99977)));
    assert_eq!(
        F.weight::<UNABRIDGED>(),
        Some(uncertain(18.998403162, 0.000000005))
    );
    assert_eq!(Ne.weight::<UNABRIDGED>(), Some(uncertain(20.1797, 0.0006)));
    assert_eq!(
        Na.weight::<UNABRIDGED>(),
        Some(uncertain(22.98976928, 0.00000002))
    );
    assert_eq!(Mg.weight::<UNABRIDGED>(), Some(interval(24.304..24.307)));
    assert_eq!(
        Al.weight::<UNABRIDGED>(),
        Some(uncertain(26.9815384, 0.0000003))
    );
    assert_eq!(Si.weight::<UNABRIDGED>(), Some(interval(28.084..28.086)));
    assert_eq!(
        P.weight::<UNABRIDGED>(),
        Some(uncertain(30.973761998, 0.000000005))
    );
    assert_eq!(S.weight::<UNABRIDGED>(), Some(interval(32.059..32.076)));
    assert_eq!(Cl.weight::<UNABRIDGED>(), Some(interval(35.446..35.457)));
    assert_eq!(Ar.weight::<UNABRIDGED>(), Some(interval(39.792..39.963)));
    assert_eq!(K.weight::<UNABRIDGED>(), Some(uncertain(39.0983, 0.0001)));
    assert_eq!(Ca.weight::<UNABRIDGED>(), Some(uncertain(40.078, 0.004)));
    assert_eq!(
        Sc.weight::<UNABRIDGED>(),
        Some(uncertain(44.955907, 0.000004))
    );
    assert_eq!(Ti.weight::<UNABRIDGED>(), Some(uncertain(47.867, 0.001)));
    assert_eq!(V.weight::<UNABRIDGED>(), Some(uncertain(50.9415, 0.0001)));
    assert_eq!(Cr.weight::<UNABRIDGED>(), Some(uncertain(51.9961, 0.0006)));
    assert_eq!(
        Mn.weight::<UNABRIDGED>(),
        Some(uncertain(54.938043, 0.000002))
    );
    assert_eq!(Fe.weight::<UNABRIDGED>(), Some(uncertain(55.845, 0.002)));
    assert_eq!(
        Co.weight::<UNABRIDGED>(),
        Some(uncertain(58.933194, 0.000003))
    );
    assert_eq!(Ni.weight::<UNABRIDGED>(), Some(uncertain(58.6934, 0.0004)));
    assert_eq!(Cu.weight::<UNABRIDGED>(), Some(uncertain(63.546, 0.003)));
    assert_eq!(Zn.weight::<UNABRIDGED>(), Some(uncertain(65.38, 0.02)));
    assert_eq!(Ga.weight::<UNABRIDGED>(), Some(uncertain(69.723, 0.001)));
    assert_eq!(Ge.weight::<UNABRIDGED>(), Some(uncertain(72.630, 0.008)));
    assert_eq!(
        As.weight::<UNABRIDGED>(),
        Some(uncertain(74.921595, 0.000006))
    );
    assert_eq!(Se.weight::<UNABRIDGED>(), Some(uncertain(78.971, 0.008)));
    assert_eq!(Br.weight::<UNABRIDGED>(), Some(interval(79.901..79.907)));
    assert_eq!(Kr.weight::<UNABRIDGED>(), Some(uncertain(83.798, 0.002)));
    assert_eq!(Rb.weight::<UNABRIDGED>(), Some(uncertain(85.4678, 0.0003)));
    assert_eq!(Sr.weight::<UNABRIDGED>(), Some(uncertain(87.62, 0.01)));
    assert_eq!(
        Y.weight::<UNABRIDGED>(),
        Some(uncertain(88.905838, 0.000002))
    );
    assert_eq!(Zr.weight::<UNABRIDGED>(), Some(uncertain(91.224, 0.002)));
    assert_eq!(
        Nb.weight::<UNABRIDGED>(),
        Some(uncertain(92.90637, 0.00001))
    );
    assert_eq!(Mo.weight::<UNABRIDGED>(), Some(uncertain(95.95, 0.01)));
    assert_eq!(Tc.weight::<UNABRIDGED>(), None);
    assert_eq!(Ru.weight::<UNABRIDGED>(), Some(uncertain(101.07, 0.02)));
    assert_eq!(
        Rh.weight::<UNABRIDGED>(),
        Some(uncertain(102.90549, 0.00002))
    );
    assert_eq!(Pd.weight::<UNABRIDGED>(), Some(uncertain(106.42, 0.01)));
    assert_eq!(Ag.weight::<UNABRIDGED>(), Some(uncertain(107.8682, 0.0002)));
    assert_eq!(Cd.weight::<UNABRIDGED>(), Some(uncertain(112.414, 0.004)));
    assert_eq!(In.weight::<UNABRIDGED>(), Some(uncertain(114.818, 0.001)));
    assert_eq!(Sn.weight::<UNABRIDGED>(), Some(uncertain(118.710, 0.007)));
    assert_eq!(Sb.weight::<UNABRIDGED>(), Some(uncertain(121.760, 0.001)));
    assert_eq!(Te.weight::<UNABRIDGED>(), Some(uncertain(127.60, 0.03)));
    assert_eq!(
        I.weight::<UNABRIDGED>(),
        Some(uncertain(126.90447, 0.00003))
    );
    assert_eq!(Xe.weight::<UNABRIDGED>(), Some(uncertain(131.293, 0.006)));
    assert_eq!(
        Cs.weight::<UNABRIDGED>(),
        Some(uncertain(132.90545196, 0.00000006))
    );
    assert_eq!(Ba.weight::<UNABRIDGED>(), Some(uncertain(137.327, 0.007)));
    assert_eq!(
        La.weight::<UNABRIDGED>(),
        Some(uncertain(138.90547, 0.00007))
    );
    assert_eq!(Ce.weight::<UNABRIDGED>(), Some(uncertain(140.116, 0.001)));
    assert_eq!(
        Pr.weight::<UNABRIDGED>(),
        Some(uncertain(140.90766, 0.00001))
    );
    assert_eq!(Nd.weight::<UNABRIDGED>(), Some(uncertain(144.242, 0.003)));
    assert_eq!(Pm.weight::<UNABRIDGED>(), None);
    assert_eq!(Sm.weight::<UNABRIDGED>(), Some(uncertain(150.36, 0.02)));
    assert_eq!(Eu.weight::<UNABRIDGED>(), Some(uncertain(151.964, 0.001)));
    assert_eq!(Gd.weight::<UNABRIDGED>(), Some(uncertain(157.25, 0.03)));
    assert_eq!(
        Tb.weight::<UNABRIDGED>(),
        Some(uncertain(158.925354, 0.000007))
    );
    assert_eq!(Dy.weight::<UNABRIDGED>(), Some(uncertain(162.500, 0.001)));
    assert_eq!(
        Ho.weight::<UNABRIDGED>(),
        Some(uncertain(164.930329, 0.000005))
    );
    assert_eq!(Er.weight::<UNABRIDGED>(), Some(uncertain(167.259, 0.003)));
    assert_eq!(
        Tm.weight::<UNABRIDGED>(),
        Some(uncertain(168.934219, 0.000005))
    );
    assert_eq!(Yb.weight::<UNABRIDGED>(), Some(uncertain(173.045, 0.010)));
    assert_eq!(Lu.weight::<UNABRIDGED>(), Some(uncertain(174.9668, 0.0001)));
    assert_eq!(Hf.weight::<UNABRIDGED>(), Some(uncertain(178.486, 0.006)));
    assert_eq!(
        Ta.weight::<UNABRIDGED>(),
        Some(uncertain(180.94788, 0.00002))
    );
    assert_eq!(W.weight::<UNABRIDGED>(), Some(uncertain(183.84, 0.01)));
    assert_eq!(Re.weight::<UNABRIDGED>(), Some(uncertain(186.207, 0.001)));
    assert_eq!(Os.weight::<UNABRIDGED>(), Some(uncertain(190.23, 0.03)));
    assert_eq!(Ir.weight::<UNABRIDGED>(), Some(uncertain(192.217, 0.002)));
    assert_eq!(Pt.weight::<UNABRIDGED>(), Some(uncertain(195.084, 0.009)));
    assert_eq!(
        Au.weight::<UNABRIDGED>(),
        Some(uncertain(196.966570, 0.000004))
    );
    assert_eq!(Hg.weight::<UNABRIDGED>(), Some(uncertain(200.592, 0.003)));
    assert_eq!(Tl.weight::<UNABRIDGED>(), Some(interval(204.382..204.385)));
    assert_eq!(Pb.weight::<UNABRIDGED>(), Some(interval(206.14..207.94)));
    assert_eq!(
        Bi.weight::<UNABRIDGED>(),
        Some(uncertain(208.98040, 0.00001))
    );
    assert_eq!(Po.weight::<UNABRIDGED>(), None);
    assert_eq!(At.weight::<UNABRIDGED>(), None);
    assert_eq!(Rn.weight::<UNABRIDGED>(), None);
    assert_eq!(Fr.weight::<UNABRIDGED>(), None);
    assert_eq!(Ra.weight::<UNABRIDGED>(), None);
    assert_eq!(Ac.weight::<UNABRIDGED>(), None);
    assert_eq!(Th.weight::<UNABRIDGED>(), Some(uncertain(232.0377, 0.0004)));
    assert_eq!(
        Pa.weight::<UNABRIDGED>(),
        Some(uncertain(231.03588, 0.00001))
    );
    assert_eq!(
        U.weight::<UNABRIDGED>(),
        Some(uncertain(238.02891, 0.00003))
    );
    assert_eq!(Np.weight::<UNABRIDGED>(), None);
    assert_eq!(Pu.weight::<UNABRIDGED>(), None);
    assert_eq!(Am.weight::<UNABRIDGED>(), None);
    assert_eq!(Cm.weight::<UNABRIDGED>(), None);
    assert_eq!(Bk.weight::<UNABRIDGED>(), None);
    assert_eq!(Cf.weight::<UNABRIDGED>(), None);
    assert_eq!(Es.weight::<UNABRIDGED>(), None);
    assert_eq!(Fm.weight::<UNABRIDGED>(), None);
    assert_eq!(Md.weight::<UNABRIDGED>(), None);
    assert_eq!(No.weight::<UNABRIDGED>(), None);
    assert_eq!(Lr.weight::<UNABRIDGED>(), None);
    assert_eq!(Rf.weight::<UNABRIDGED>(), None);
    assert_eq!(Db.weight::<UNABRIDGED>(), None);
    assert_eq!(Sg.weight::<UNABRIDGED>(), None);
    assert_eq!(Bh.weight::<UNABRIDGED>(), None);
    assert_eq!(Hs.weight::<UNABRIDGED>(), None);
    assert_eq!(Mt.weight::<UNABRIDGED>(), None);
    assert_eq!(Ds.weight::<UNABRIDGED>(), None);
    assert_eq!(Rg.weight::<UNABRIDGED>(), None);
    assert_eq!(Cn.weight::<UNABRIDGED>(), None);
    assert_eq!(Nh.weight::<UNABRIDGED>(), None);
    assert_eq!(Fl.weight::<UNABRIDGED>(), None);
    assert_eq!(Mc.weight::<UNABRIDGED>(), None);
    assert_eq!(Lv.weight::<UNABRIDGED>(), None);
    assert_eq!(Ts.weight::<UNABRIDGED>(), None);
    assert_eq!(Og.weight::<UNABRIDGED>(), None);
}
