use crate::{
    Element,
    Element::{
        Ac, Ag, Al, Am, Ar, As, At, Au, Ba, Be, Bh, Bi, Bk, Br, Ca, Cd, Ce, Cf, Cl, Cm, Cn, Co, Cr,
        Cs, Cu, Db, Ds, Dy, Er, Es, Eu, Fe, Fl, Fm, Fr, Ga, Gd, Ge, He, Hf, Hg, Ho, Hs, In, Ir, Kr,
        La, Li, Lr, Lu, Lv, Mc, Md, Mg, Mn, Mo, Mt, Na, Nb, Nd, Ne, Nh, Ni, No, Np, Og, Os, Pa, Pb,
        Pd, Pm, Po, Pr, Pt, Pu, Ra, Rb, Re, Rf, Rg, Rh, Rn, Ru, Sb, Sc, Se, Sg, Si, Sm, Sn, Sr, Ta,
        Tb, Tc, Te, Th, Ti, Tl, Tm, Ts, Xe, Yb, Zn, Zr, B, C, F, H, I, K, N, O, P, S, U, V, W, Y,
    },
};

pub const PERIODS: usize = 7;

pub const GROUPS: usize = 18;

#[rustfmt::skip]
pub const PERIODIC_TABLE: [[Option<Element>; 32]; 7] = [
    [Some(H),  None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(He)],
    [Some(Li), Some(Be), None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(B),  Some(C),  Some(N),  Some(O),  Some(F),  Some(Ne)],
    [Some(Na), Some(Mg), None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Al), Some(Si), Some(P),  Some(S),  Some(Cl), Some(Ar)],
    [Some(K),  Some(Ca), None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Sc), Some(Ti), Some(V),  Some(Cr), Some(Mn), Some(Fe), Some(Co), Some(Ni), Some(Cu), Some(Zn), Some(Ga), Some(Ge), Some(As), Some(Se), Some(Br), Some(Kr)],
    [Some(Rb), Some(Sr), None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Y),  Some(Zr), Some(Nb), Some(Mo), Some(Tc), Some(Ru), Some(Rh), Some(Pd), Some(Ag), Some(Cd), Some(In), Some(Sn), Some(Sb), Some(Te), Some(I),  Some(Xe)],
    [Some(Cs), Some(Ba), Some(La), Some(Ce), Some(Pr), Some(Nd), Some(Pm), Some(Sm), Some(Eu), Some(Gd), Some(Tb), Some(Dy), Some(Ho), Some(Er), Some(Tm), Some(Yb), Some(Lu), Some(Hf), Some(Ta), Some(W),  Some(Re), Some(Os), Some(Ir), Some(Pt), Some(Au), Some(Hg), Some(Tl), Some(Pb), Some(Bi), Some(Po), Some(At), Some(Rn)],
    [Some(Fr), Some(Ra), Some(Ac), Some(Th), Some(Pa), Some(U),  Some(Np), Some(Pu), Some(Am), Some(Cm), Some(Bk), Some(Cf), Some(Es), Some(Fm), Some(Md), Some(No), Some(Lr), Some(Rf), Some(Db), Some(Sg), Some(Bh), Some(Hs), Some(Mt), Some(Ds), Some(Rg), Some(Cn), Some(Nh), Some(Fl), Some(Mc), Some(Lv), Some(Ts), Some(Og)],
];

//          1s                                                                                           H  He
//          2s                                                                                           Li Be
//       2p 3s                                                                         B  C  N  O  F  Ne Na Mg
//       3p 4s                                                                         Al Si P  S  Cl Ar K  Ca
//    3d 4p 5s                                           Sc Ti V  Cr Mn Fe Co Ni Cu Zn Ga Ge As Se Br Kr Rb Sr
//    4d 5p 6s                                           Y  Zr Nb Mo Tc Ru Rh Pd Ag Cd In Sn Sb Te I  Xe Cs Ba
// 4f 5d 6p 7s La Ce Pr Nd Pm Sm Eu Gd Tb Dy Ho Er Tm Yb Lu Hf Ta W  Re Os Ir Pt Au Hg Tl Pb Bi Po At Rn Fr Ra
// 5f 6d 7p 8s Ac Th Pa U  Np Pu Am Cm Bk Cf Es Fm Md No Lr Rf Db Sg Bh Hs Mt Ds Rg Cn Nh Fl Mc Lv Ts Og
#[rustfmt::skip]
pub const LEFT_STEP_PERIODIC_TABLE: [[Option<Element>; 32]; 8] = [
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(H),  Some(He)],
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Li), Some(Be)],
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(B),  Some(C),  Some(N),  Some(O),  Some(F),  Some(Ne), Some(Na), Some(Mg)],
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Al), Some(Si), Some(P),  Some(S),  Some(Cl), Some(Ar), Some(K),  Some(Ca)],
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Sc), Some(Ti), Some(V),  Some(Cr), Some(Mn), Some(Fe), Some(Co), Some(Ni), Some(Cu), Some(Zn), Some(Ga), Some(Ge), Some(As), Some(Se), Some(Br), Some(Kr), Some(Rb), Some(Sr)],
    [None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     Some(Y),  Some(Zr), Some(Nb), Some(Mo), Some(Tc), Some(Ru), Some(Rh), Some(Pd), Some(Ag), Some(Cd), Some(In), Some(Sn), Some(Sb), Some(Te), Some(I),  Some(Xe), Some(Cs), Some(Ba)],
    [Some(La), Some(Ce), Some(Pr), Some(Nd), Some(Pm), Some(Sm), Some(Eu), Some(Gd), Some(Tb), Some(Dy), Some(Ho), Some(Er), Some(Tm), Some(Yb), Some(Lu), Some(Hf), Some(Ta), Some(W),  Some(Re), Some(Os), Some(Ir), Some(Pt), Some(Au), Some(Hg), Some(Tl), Some(Pb), Some(Bi), Some(Po), Some(At), Some(Rn), Some(Fr), Some(Ra)],
    [Some(Ac), Some(Th), Some(Pa), Some(U),  Some(Np), Some(Pu), Some(Am), Some(Cm), Some(Bk), Some(Cf), Some(Es), Some(Fm), Some(Md), Some(No), Some(Lr), Some(Rf), Some(Db), Some(Sg), Some(Bh), Some(Hs), Some(Mt), Some(Ds), Some(Rg), Some(Cn), Some(Nh), Some(Fl), Some(Mc), Some(Lv), Some(Ts), Some(Og), None,     None],
];

// #[rustfmt::skip]
// pub const TABLE: [[Option<Cell>; 18]; 7] = [
//     [Some(Atom(H)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(Atom(He))],
//     [Some(Atom(Li)), Some(Atom(Be)), None, None, None, None, None, None, None, None, None, None, Some(Atom(B)), Some(Atom(C)), Some(Atom(N)), Some(Atom(O)), Some(Atom(F)), Some(Atom(Ne))],
//     [Some(Atom(Na)), Some(Atom(Mg)), None, None, None, None, None, None, None, None, None, None, Some(Atom(Al)), Some(Atom(Si)), Some(Atom(P)), Some(Atom(S)), Some(Atom(Cl)), Some(Atom(Ar))],
//     [Some(Atom(K)), Some(Atom(Ca)), Some(Atom(Sc)), Some(Atom(Ti)), Some(Atom(V)), Some(Atom(Cr)), Some(Atom(Mn)), Some(Atom(Fe)), Some(Atom(Co)), Some(Atom(Ni)), Some(Atom(Cu)), Some(Atom(Zn)), Some(Atom(Ga)), Some(Atom(Ge)), Some(Atom(As)), Some(Atom(Se)), Some(Atom(Br)), Some(Atom(Kr))],
//     [Some(Atom(Rb)), Some(Atom(Sr)), Some(Atom(Y)), Some(Atom(Zr)), Some(Atom(Nb)), Some(Atom(Mo)), Some(Atom(Tc)), Some(Atom(Ru)), Some(Atom(Rh)), Some(Atom(Pd)), Some(Atom(Ag)), Some(Atom(Cd)), Some(Atom(In)), Some(Atom(Sn)), Some(Atom(Sb)), Some(Atom(Te)), Some(Atom(I)), Some(Atom(Xe))],
//     [Some(Atom(Cs)), Some(Atom(Ba)), Some(Atoms([La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu])), Some(Atom(Hf)), Some(Atom(Ta)), Some(Atom(W)), Some(Atom(Re)), Some(Atom(Os)), Some(Atom(Ir)), Some(Atom(Pt)), Some(Atom(Au)), Some(Atom(Hg)), Some(Atom(Tl)), Some(Atom(Pb)), Some(Atom(Bi)), Some(Atom(Po)), Some(Atom(At)), Some(Atom(Rn))],
//     [Some(Atom(Fr)), Some(Atom(Ra)), Some(Atoms([Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr])), Some(Atom(Rf)), Some(Atom(Db)), Some(Atom(Sg)), Some(Atom(Bh)), Some(Atom(Hs)), Some(Atom(Mt)), Some(Atom(Ds)), Some(Atom(Rg)), Some(Atom(Cn)), Some(Atom(Nh)), Some(Atom(Fl)), Some(Atom(Mc)), Some(Atom(Lv)), Some(Atom(Ts)), Some(Atom(Og))],
// ];

// s, p, d, f, g
// 2, 2, 8, 8, 18, 18, 32, 32.

impl Element {
    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

#[test]
fn test() {
    for row in LEFT_STEP_PERIODIC_TABLE {
        for cell in row {
            match cell {
                None => print!("   "),
                Some(element) => print!("{element:2} "),
            }
        }
        println!();
    }
}
