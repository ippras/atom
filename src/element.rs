use self::Element::*;
use crate::{Error, Result};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

pub const ELEMENTS: [Element; COUNT] = [
    H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca, Sc, Ti, V, Cr, Mn, Fe,
    Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn,
    Sb, Te, I, Xe, Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,
    Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf,
    Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
];

pub const COUNT: usize = 118;

/// Element
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Element {
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

impl Element {
    /// Atomic number
    pub const fn atomic_number(&self) -> usize {
        *self as usize
    }

    /// Name
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
}

impl Element {
    pub fn split(&self) -> (&[Self], &[Self]) {
        let split = ELEMENTS.split_at(*self as _);
        (split.0, &split.1[1..])
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            Display::fmt(self.name(), f)
        } else {
            Display::fmt(self.symbol(), f)
        }
    }
}

impl FromStr for Element {
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