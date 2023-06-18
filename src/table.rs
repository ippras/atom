// #[rustfmt::skip]
// pub const TABLE: [[Option<Cell>; 18]; 7] = [
//     [Some(Cell::Atom(H)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(Cell::Atom(He))],
//     [Some(Cell::Atom(Li)), Some(Cell::Atom(Be)), None, None, None, None, None, None, None, None, None, None, Some(Cell::Atom(B)), Some(Cell::Atom(C)), Some(Cell::Atom(N)), Some(Cell::Atom(O)), Some(Cell::Atom(F)), Some(Cell::Atom(Ne))],
//     [Some(Cell::Atom(Na)), Some(Cell::Atom(Mg)), None, None, None, None, None, None, None, None, None, None, Some(Cell::Atom(Al)), Some(Cell::Atom(Si)), Some(Cell::Atom(P)), Some(Cell::Atom(S)), Some(Cell::Atom(Cl)), Some(Cell::Atom(Ar))],
//     [Some(Cell::Atom(K)), Some(Cell::Atom(Ca)), Some(Cell::Atom(Sc)), Some(Cell::Atom(Ti)), Some(Cell::Atom(V)), Some(Cell::Atom(Cr)), Some(Cell::Atom(Mn)), Some(Cell::Atom(Fe)), Some(Cell::Atom(Co)), Some(Cell::Atom(Ni)), Some(Cell::Atom(Cu)), Some(Cell::Atom(Zn)), Some(Cell::Atom(Ga)), Some(Cell::Atom(Ge)), Some(Cell::Atom(As)), Some(Cell::Atom(Se)), Some(Cell::Atom(Br)), Some(Cell::Atom(Kr))],
//     [Some(Cell::Atom(Rb)), Some(Cell::Atom(Sr)), Some(Cell::Atom(Y)), Some(Cell::Atom(Zr)), Some(Cell::Atom(Nb)), Some(Cell::Atom(Mo)), Some(Cell::Atom(Tc)), Some(Cell::Atom(Ru)), Some(Cell::Atom(Rh)), Some(Cell::Atom(Pd)), Some(Cell::Atom(Ag)), Some(Cell::Atom(Cd)), Some(Cell::Atom(In)), Some(Cell::Atom(Sn)), Some(Cell::Atom(Sb)), Some(Cell::Atom(Te)), Some(Cell::Atom(I)), Some(Cell::Atom(Xe))],
//     [Some(Cell::Atom(Cs)), Some(Cell::Atom(Ba)), Some(Cell::Atoms([La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu])), Some(Cell::Atom(Hf)), Some(Cell::Atom(Ta)), Some(Cell::Atom(W)), Some(Cell::Atom(Re)), Some(Cell::Atom(Os)), Some(Cell::Atom(Ir)), Some(Cell::Atom(Pt)), Some(Cell::Atom(Au)), Some(Cell::Atom(Hg)), Some(Cell::Atom(Tl)), Some(Cell::Atom(Pb)), Some(Cell::Atom(Bi)), Some(Cell::Atom(Po)), Some(Cell::Atom(At)), Some(Cell::Atom(Rn))],
//     [Some(Cell::Atom(Fr)), Some(Cell::Atom(Ra)), Some(Cell::Atoms([Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr])), Some(Cell::Atom(Rf)), Some(Cell::Atom(Db)), Some(Cell::Atom(Sg)), Some(Cell::Atom(Bh)), Some(Cell::Atom(Hs)), Some(Cell::Atom(Mt)), Some(Cell::Atom(Ds)), Some(Cell::Atom(Rg)), Some(Cell::Atom(Cn)), Some(Cell::Atom(Nh)), Some(Cell::Atom(Fl)), Some(Cell::Atom(Mc)), Some(Cell::Atom(Lv)), Some(Cell::Atom(Ts)), Some(Cell::Atom(Og))],
// ];

// pub enum Cell {
//     Empty,
//     Atom(Atom),
//     Atoms([Atom; 15]),
// }

//         [
//         H => [1, 1], He => [1, 18],
//         Li => [2, 1], Be => [2, 2], B => [2, 13], C => [2, 14], N => [2, 15], O => [2, 16], F => [2, 17], Ne => [2, 18],
//         Na => [3, 1], Mg => [3, 2], Al => [3, 13], Si => [3, 14], P => [3, 15], S => [3, 16], Cl => [3, 17], Ar => [3, 18],
//         K => [4, 0], Ca => [4, 0], Sc => [4, 0], Ti => [4, 0], V => [4, 0], Cr => [4, 0], Mn => [4, 0], Fe => [4, 0], Co => [4, 0], Ni => [4, 0], Cu => [4, 0], Zn => [4, 0], Ga => [4, 0], Ge => [4, 0], As => [4, 0], Se => [4, 0], Br => [4, 0], Kr => [4, 0],
//         Rb => [5, 0], Sr => [5, 0], Y => [5, 0], Zr => [5, 0], Nb => [5, 0], Mo => [5, 0], Tc => [5, 0], Ru => [5, 0], Rh => [5, 0], Pd => [5, 0], Ag => [5, 0], Cd => [5, 0], In => [5, 0], Sn => [5, 0], Sb => [5, 0], Te => [5, 0], I => [5, 0], Xe => [5, 0],
//         Cs => [6, 0], Ba => [6, 0], La => [6, 0], Ce => [6, 0], Pr => [6, 0], Nd => [6, 0], Pm => [6, 0], Sm => [6, 0], Eu => [6, 0], Gd => [6, 0], Tb => [6, 0], Dy => [6, 0], Ho => [6, 0], Er => [6, 0], Tm => [6, 0], Yb => [6, 0], Lu => [6, 0], Hf => [6, 0], Ta => [6, 0], W => [6, 0], Re => [6, 0], Os => [6, 0], Ir => [6, 0], Pt => [6, 0], Au => [6, 0], Hg => [6, 0], Tl => [6, 0], Pb => [6, 0], Bi => [6, 0], Po => [6, 0], At => [6, 0], Rn => [6, 0],
//         Fr => [7, 0], Ra => [7, 0], Ac => [7, 0], Th => [7, 0], Pa => [7, 0], U => [7, 0], Np => [7, 0], Pu => [7, 0], Am => [7, 0], Cm => [7, 0], Bk => [7, 0], Cf => [7, 0], Es => [7, 0], Fm => [7, 0], Md => [7, 0], No => [7, 0], Lr => [7, 0], Rf => [7, 0], Db => [7, 0], Sg => [7, 0], Bh => [7, 0], Hs => [7, 0], Mt => [7, 0], Ds => [7, 0], Rg => [7, 0], Cn => [7, 0], Nh => [7, 0], Fl => [7, 0], Mc => [7, 0], Lv => [7, 0], Ts => [7, 0], Og => [7, 0],
//     ];

// #[rustfmt::skip]
// pub static MAP: HashMap<Atom, usize> = hashmap![
//     H => 0,
// ];

// H => [0, 0], He => [0, 0], Li => [0, 0], Be => [0, 0], B => [0, 0], C => [0, 0], N => [0, 0], O => [0, 0], F => [0, 0], Ne => [0, 0], Na => [0, 0], Mg => [0, 0], Al => [0, 0], Si => [0, 0], P => [0, 0], S => [0, 0], Cl => [0, 0], Ar => [0, 0], K => [0, 0], Ca => [0, 0], Sc => [0, 0], Ti => [0, 0], V => [0, 0], Cr => [0, 0], Mn => [0, 0], Fe => [0, 0],
// Co => [0, 0], Ni => [0, 0], Cu => [0, 0], Zn => [0, 0], Ga => [0, 0], Ge => [0, 0], As => [0, 0], Se => [0, 0], Br => [0, 0], Kr => [0, 0], Rb => [0, 0], Sr => [0, 0], Y => [0, 0], Zr => [0, 0], Nb => [0, 0], Mo => [0, 0], Tc => [0, 0], Ru => [0, 0], Rh => [0, 0], Pd => [0, 0], Ag => [0, 0], Cd => [0, 0], In => [0, 0], Sn => [0, 0],
// Sb => [0, 0], Te => [0, 0], I => [0, 0], Xe => [0, 0], Cs => [0, 0], Ba => [0, 0], La => [0, 0], Ce => [0, 0], Pr => [0, 0], Nd => [0, 0], Pm => [0, 0], Sm => [0, 0], Eu => [0, 0], Gd => [0, 0], Tb => [0, 0], Dy => [0, 0], Ho => [0, 0], Er => [0, 0], Tm => [0, 0], Yb => [0, 0], Lu => [0, 0], Hf => [0, 0], Ta => [0, 0], W => [0, 0],
// Re => [0, 0], Os => [0, 0], Ir => [0, 0], Pt => [0, 0], Au => [0, 0], Hg => [0, 0], Tl => [0, 0], Pb => [0, 0], Bi => [0, 0], Po => [0, 0], At => [0, 0], Rn => [0, 0], Fr => [0, 0], Ra => [0, 0], Ac => [0, 0], Th => [0, 0], Pa => [0, 0], U => [0, 0], Np => [0, 0], Pu => [0, 0], Am => [0, 0], Cm => [0, 0], Bk => [0, 0], Cf => [0, 0],
// Es => [0, 0], Fm => [0, 0], Md => [0, 0], No => [0, 0], Lr => [0, 0], Rf => [0, 0], Db => [0, 0], Sg => [0, 0], Bh => [0, 0], Hs => [0, 0], Mt => [0, 0], Ds => [0, 0], Rg => [0, 0], Cn => [0, 0], Nh => [0, 0], Fl => [0, 0], Mc => [0, 0], Lv => [0, 0], Ts => [0, 0], Og => [0, 0],
