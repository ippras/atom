use atom::{uncertain, Element::*};

#[test]
#[rustfmt::skip]
fn standard_atomic_weight() {
    assert_eq!(H.standard_atomic_weight(), Some(uncertain!(1.007_84..1.008_11)));
    assert_eq!(He.standard_atomic_weight(), Some(uncertain!(4.002_602, 0.000_002)));
    assert_eq!(Li.standard_atomic_weight(), Some(uncertain!(6.938..6.997)));
    assert_eq!(Be.standard_atomic_weight(), Some(uncertain!(9.012_183_1, 0.000_000_5)));
    assert_eq!(B.standard_atomic_weight(), Some(uncertain!(10.806..10.821)));
    assert_eq!(C.standard_atomic_weight(), Some(uncertain!(12.009_6..12.011_6)));
    assert_eq!(N.standard_atomic_weight(), Some(uncertain!(14.006_43..14.007_28)));
    assert_eq!(O.standard_atomic_weight(), Some(uncertain!(15.999_03..15.999_77)));
    #[cfg(feature = "nist")]
    assert_eq!(F.standard_atomic_weight(), Some(uncertain!(18.998_403_163, 0.000_000_006)));
    #[cfg(feature = "iupac")]
    assert_eq!(F.standard_atomic_weight(), Some(uncertain!(18.998_403_162, 0.000_000_005)));
    assert_eq!(Ne.standard_atomic_weight(), Some(uncertain!(20.179_7, 0.000_6)));
    assert_eq!(Na.standard_atomic_weight(), Some(uncertain!(22.989_769_28, 0.000_000_02)));
    assert_eq!(Mg.standard_atomic_weight(), Some(uncertain!(24.304..24.307)));
    #[cfg(feature = "nist")]
    assert_eq!(Al.standard_atomic_weight(), Some(uncertain!(26.981_538_5, 0.000_000_7)));
    #[cfg(feature = "iupac")]
    assert_eq!(Al.standard_atomic_weight(), Some(uncertain!(26.981_538_4, 0.000_000_3)));
    assert_eq!(Si.standard_atomic_weight(), Some(uncertain!(28.084..28.086)));
    assert_eq!(P.standard_atomic_weight(), Some(uncertain!(30.973_761_998, 0.000_000_005)));
    assert_eq!(S.standard_atomic_weight(), Some(uncertain!(32.059..32.076)));
    assert_eq!(Cl.standard_atomic_weight(), Some(uncertain!(35.446..35.457)));
    #[cfg(feature = "nist")]
    assert_eq!(Ar.standard_atomic_weight(), Some(uncertain!(39.948, 0.001)));
    #[cfg(feature = "iupac")]
    assert_eq!(Ar.standard_atomic_weight(), Some(uncertain!(39.792..39.963)));
    assert_eq!(K.standard_atomic_weight(), Some(uncertain!(39.098_3, 0.000_1)));
    assert_eq!(Ca.standard_atomic_weight(), Some(uncertain!(40.078, 0.004)));
    #[cfg(feature = "nist")]
    assert_eq!(Sc.standard_atomic_weight(), Some(uncertain!(44.955_908, 0.000_005)));
    #[cfg(feature = "iupac")]
    assert_eq!(Sc.standard_atomic_weight(), Some(uncertain!(44.955_907, 0.000_004)));
    assert_eq!(Ti.standard_atomic_weight(), Some(uncertain!(47.867, 0.001)));
    assert_eq!(V.standard_atomic_weight(), Some(uncertain!(50.941_5, 0.000_1)));
    assert_eq!(Cr.standard_atomic_weight(), Some(uncertain!(51.996_1, 0.000_6)));
    #[cfg(feature = "nist")]
    assert_eq!(Mn.standard_atomic_weight(), Some(uncertain!(54.938_044, 0.000_003)));
    #[cfg(feature = "iupac")]
    assert_eq!(Mn.standard_atomic_weight(), Some(uncertain!(54.938_043, 0.000_002)));
    assert_eq!(Fe.standard_atomic_weight(), Some(uncertain!(55.845, 0.002)));
    #[cfg(feature = "nist")]
    assert_eq!(Co.standard_atomic_weight(), Some(uncertain!(58.933_194, 0.000_004)));
    #[cfg(feature = "iupac")]
    assert_eq!(Co.standard_atomic_weight(), Some(uncertain!(58.933_194, 0.000_003)));
    assert_eq!(Ni.standard_atomic_weight(), Some(uncertain!(58.693_4, 0.000_4)));
    assert_eq!(Cu.standard_atomic_weight(), Some(uncertain!(63.546, 0.003)));
    assert_eq!(Zn.standard_atomic_weight(), Some(uncertain!(65.38, 0.02)));
    assert_eq!(Ga.standard_atomic_weight(), Some(uncertain!(69.723, 0.001)));
    assert_eq!(Ge.standard_atomic_weight(), Some(uncertain!(72.630, 0.008)));
    assert_eq!(As.standard_atomic_weight(), Some(uncertain!(74.921_595, 0.000_006)));
    assert_eq!(Se.standard_atomic_weight(), Some(uncertain!(78.971, 0.008)));
    assert_eq!(Br.standard_atomic_weight(), Some(uncertain!(79.901..79.907)));
    assert_eq!(Kr.standard_atomic_weight(), Some(uncertain!(83.798, 0.002)));
    assert_eq!(Rb.standard_atomic_weight(), Some(uncertain!(85.467_8, 0.000_3)));
    assert_eq!(Sr.standard_atomic_weight(), Some(uncertain!(87.62, 0.01)));
    #[cfg(feature = "nist")]
    assert_eq!(Y.standard_atomic_weight(), Some(uncertain!(88.905_84, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Y.standard_atomic_weight(), Some(uncertain!(88.905_838, 0.000_002)));
    assert_eq!(Zr.standard_atomic_weight(), Some(uncertain!(91.224, 0.002)));
    #[cfg(feature = "nist")]
    assert_eq!(Nb.standard_atomic_weight(), Some(uncertain!(92.906_37, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Nb.standard_atomic_weight(), Some(uncertain!(92.906_37, 0.000_01)));
    assert_eq!(Mo.standard_atomic_weight(), Some(uncertain!(95.95, 0.01)));
    assert_eq!(Tc.standard_atomic_weight(), None);
    assert_eq!(Ru.standard_atomic_weight(), Some(uncertain!(101.07, 0.02)));
    #[cfg(feature = "nist")]
    assert_eq!(Rh.standard_atomic_weight(), Some(uncertain!(102.905_50, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Rh.standard_atomic_weight(), Some(uncertain!(102.905_49, 0.000_02)));
    assert_eq!(Pd.standard_atomic_weight(), Some(uncertain!(106.42, 0.01)));
    assert_eq!(Ag.standard_atomic_weight(), Some(uncertain!(107.868_2, 0.000_2)));
    assert_eq!(Cd.standard_atomic_weight(), Some(uncertain!(112.414, 0.004)));
    assert_eq!(In.standard_atomic_weight(), Some(uncertain!(114.818, 0.001)));
    assert_eq!(Sn.standard_atomic_weight(), Some(uncertain!(118.710, 0.007)));
    assert_eq!(Sb.standard_atomic_weight(), Some(uncertain!(121.760, 0.001)));
    assert_eq!(Te.standard_atomic_weight(), Some(uncertain!(127.60, 0.03)));
    assert_eq!(I.standard_atomic_weight(), Some(uncertain!(126.904_47, 0.000_03)));
    assert_eq!(Xe.standard_atomic_weight(), Some(uncertain!(131.293, 0.006)));
    assert_eq!(Cs.standard_atomic_weight(), Some(uncertain!(132.905_451_96, 0.000_000_06)));
    assert_eq!(Ba.standard_atomic_weight(), Some(uncertain!(137.327, 0.007)));
    assert_eq!(La.standard_atomic_weight(), Some(uncertain!(138.905_47, 0.000_07)));
    assert_eq!(Ce.standard_atomic_weight(), Some(uncertain!(140.116, 0.001)));
    #[cfg(feature = "nist")]
    assert_eq!(Pr.standard_atomic_weight(), Some(uncertain!(140.907_66, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Pr.standard_atomic_weight(), Some(uncertain!(140.907_66, 0.000_01)));
    assert_eq!(Nd.standard_atomic_weight(), Some(uncertain!(144.242, 0.003)));
    assert_eq!(Pm.standard_atomic_weight(), None);
    assert_eq!(Sm.standard_atomic_weight(), Some(uncertain!(150.36, 0.02)));
    assert_eq!(Eu.standard_atomic_weight(), Some(uncertain!(151.964, 0.001)));
    assert_eq!(Gd.standard_atomic_weight(), Some(uncertain!(157.25, 0.03)));
    #[cfg(feature = "nist")]
    assert_eq!(Tb.standard_atomic_weight(), Some(uncertain!(158.925_35, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Tb.standard_atomic_weight(), Some(uncertain!(158.925_354, 0.000_007)));
    assert_eq!(Dy.standard_atomic_weight(), Some(uncertain!(162.500, 0.001)));
    #[cfg(feature = "nist")]
    assert_eq!(Ho.standard_atomic_weight(), Some(uncertain!(164.930_33, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Ho.standard_atomic_weight(), Some(uncertain!(164.930_329, 0.000_005)));
    assert_eq!(Er.standard_atomic_weight(), Some(uncertain!(167.259, 0.003)));
    #[cfg(feature = "nist")]
    assert_eq!(Tm.standard_atomic_weight(), Some(uncertain!(168.934_22, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Tm.standard_atomic_weight(), Some(uncertain!(168.934_219, 0.000_005)));
    #[cfg(feature = "nist")]
    assert_eq!(Yb.standard_atomic_weight(), Some(uncertain!(173.054, 0.005)));
    #[cfg(feature = "iupac")]
    assert_eq!(Yb.standard_atomic_weight(), Some(uncertain!(173.045, 0.010)));
    assert_eq!(Lu.standard_atomic_weight(), Some(uncertain!(174.966_8, 0.000_1)));
    #[cfg(feature = "nist")]
    assert_eq!(Hf.standard_atomic_weight(), Some(uncertain!(178.49, 0.02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Hf.standard_atomic_weight(), Some(uncertain!(178.486, 0.006)));
    assert_eq!(Ta.standard_atomic_weight(), Some(uncertain!(180.947_88, 0.000_02)));
    assert_eq!(W.standard_atomic_weight(), Some(uncertain!(183.84, 0.01)));
    assert_eq!(Re.standard_atomic_weight(), Some(uncertain!(186.207, 0.001)));
    assert_eq!(Os.standard_atomic_weight(), Some(uncertain!(190.23, 0.03)));
    #[cfg(feature = "nist")]
    assert_eq!(Ir.standard_atomic_weight(), Some(uncertain!(192.217, 0.003)));
    #[cfg(feature = "iupac")]
    assert_eq!(Ir.standard_atomic_weight(), Some(uncertain!(192.217, 0.002)));
    assert_eq!(Pt.standard_atomic_weight(), Some(uncertain!(195.084, 0.009)));
    #[cfg(feature = "nist")]
    assert_eq!(Au.standard_atomic_weight(), Some(uncertain!(196.966_569, 0.000_005)));
    #[cfg(feature = "iupac")]
    assert_eq!(Au.standard_atomic_weight(), Some(uncertain!(196.966_570, 0.000_004)));
    assert_eq!(Hg.standard_atomic_weight(), Some(uncertain!(200.592, 0.003)));
    assert_eq!(Tl.standard_atomic_weight(), Some(uncertain!(204.382..204.385)));
    #[cfg(feature = "nist")]
    assert_eq!(Pb.standard_atomic_weight(), Some(uncertain!(207.2, 0.1)));
    #[cfg(feature = "iupac")]
    assert_eq!(Pb.standard_atomic_weight(), Some(uncertain!(206.14..207.94)));
    assert_eq!(Bi.standard_atomic_weight(), Some(uncertain!(208.980_40, 0.000_01)));
    assert_eq!(Po.standard_atomic_weight(), None);
    assert_eq!(At.standard_atomic_weight(), None);
    assert_eq!(Rn.standard_atomic_weight(), None);
    assert_eq!(Fr.standard_atomic_weight(), None);
    assert_eq!(Ra.standard_atomic_weight(), None);
    assert_eq!(Ac.standard_atomic_weight(), None);
    assert_eq!(Th.standard_atomic_weight(), Some(uncertain!(232.037_7, 0.000_4)));
    #[cfg(feature = "nist")]
    assert_eq!(Pa.standard_atomic_weight(), Some(uncertain!(231.035_88, 0.000_02)));
    #[cfg(feature = "iupac")]
    assert_eq!(Pa.standard_atomic_weight(), Some(uncertain!(231.035_88, 0.000_01)));
    assert_eq!(U.standard_atomic_weight(), Some(uncertain!(238.028_91, 0.000_03)));
    assert_eq!(Np.standard_atomic_weight(), None);
    assert_eq!(Pu.standard_atomic_weight(), None);
    assert_eq!(Am.standard_atomic_weight(), None);
    assert_eq!(Cm.standard_atomic_weight(), None);
    assert_eq!(Bk.standard_atomic_weight(), None);
    assert_eq!(Cf.standard_atomic_weight(), None);
    assert_eq!(Es.standard_atomic_weight(), None);
    assert_eq!(Fm.standard_atomic_weight(), None);
    assert_eq!(Md.standard_atomic_weight(), None);
    assert_eq!(No.standard_atomic_weight(), None);
    assert_eq!(Lr.standard_atomic_weight(), None);
    assert_eq!(Rf.standard_atomic_weight(), None);
    assert_eq!(Db.standard_atomic_weight(), None);
    assert_eq!(Sg.standard_atomic_weight(), None);
    assert_eq!(Bh.standard_atomic_weight(), None);
    assert_eq!(Hs.standard_atomic_weight(), None);
    assert_eq!(Mt.standard_atomic_weight(), None);
    assert_eq!(Ds.standard_atomic_weight(), None);
    assert_eq!(Rg.standard_atomic_weight(), None);
    assert_eq!(Cn.standard_atomic_weight(), None);
    assert_eq!(Nh.standard_atomic_weight(), None);
    assert_eq!(Fl.standard_atomic_weight(), None);
    assert_eq!(Mc.standard_atomic_weight(), None);
    assert_eq!(Lv.standard_atomic_weight(), None);
    assert_eq!(Ts.standard_atomic_weight(), None);
    assert_eq!(Og.standard_atomic_weight(), None);
}

// #[test]
// #[rustfmt::skip]
// fn abridged_standard_atomic_weight() {
//     assert_eq!(H.standard_atomic_weight().abridged(), Some(uncertain!(1.008_0, 0.000_2)));
//     assert_eq!(He.standard_atomic_weight().abridged(), Some(uncertain!(4.002_6, 0.000_1)));
//     assert_eq!(Li.standard_atomic_weight().abridged(), Some(uncertain!(6.94, 0.06)));
//     assert_eq!(Be.standard_atomic_weight().abridged(), Some(uncertain!(9.012_2, 0.000_1)));
//     assert_eq!(B.standard_atomic_weight().abridged(), Some(uncertain!(10.81, 0.02)));
//     assert_eq!(C.standard_atomic_weight().abridged(), Some(uncertain!(12.011, 0.002)));
//     assert_eq!(N.standard_atomic_weight().abridged(), Some(uncertain!(14.007, 0.001)));
//     assert_eq!(O.standard_atomic_weight().abridged(), Some(uncertain!(15.999, 0.001)));
//     assert_eq!(F.standard_atomic_weight().abridged(), Some(uncertain!(18.998, 0.001)));
//     assert_eq!(Ne.standard_atomic_weight().abridged(), Some(uncertain!(20.180, 0.001)));
//     assert_eq!(Na.standard_atomic_weight().abridged(), Some(uncertain!(22.990, 0.001)));
//     assert_eq!(Mg.standard_atomic_weight().abridged(), Some(uncertain!(24.305, 0.002)));
//     assert_eq!(Al.standard_atomic_weight().abridged(), Some(uncertain!(26.982, 0.001)));
//     assert_eq!(Si.standard_atomic_weight().abridged(), Some(uncertain!(28.085, 0.001)));
//     assert_eq!(P.standard_atomic_weight().abridged(), Some(uncertain!(30.974, 0.001)));
//     assert_eq!(S.standard_atomic_weight().abridged(), Some(uncertain!(32.06, 0.02)));
//     assert_eq!(Cl.standard_atomic_weight().abridged(), Some(uncertain!(35.45, 0.01)));
//     assert_eq!(Ar.standard_atomic_weight().abridged(), Some(uncertain!(39.95, 0.16)));
//     assert_eq!(K.standard_atomic_weight().abridged(), Some(uncertain!(39.098, 0.001)));
//     assert_eq!(Ca.standard_atomic_weight().abridged(), Some(uncertain!(40.078, 0.004)));
//     assert_eq!(Sc.standard_atomic_weight().abridged(), Some(uncertain!(44.956, 0.001)));
//     assert_eq!(Ti.standard_atomic_weight().abridged(), Some(uncertain!(47.867, 0.001)));
//     assert_eq!(V.standard_atomic_weight().abridged(), Some(uncertain!(50.942, 0.001)));
//     assert_eq!(Cr.standard_atomic_weight().abridged(), Some(uncertain!(51.996, 0.001)));
//     assert_eq!(Mn.standard_atomic_weight().abridged(), Some(uncertain!(54.938, 0.001)));
//     assert_eq!(Fe.standard_atomic_weight().abridged(), Some(uncertain!(55.845, 0.002)));
//     assert_eq!(Co.standard_atomic_weight().abridged(), Some(uncertain!(58.933, 0.001)));
//     assert_eq!(Ni.standard_atomic_weight().abridged(), Some(uncertain!(58.693, 0.001)));
//     assert_eq!(Cu.standard_atomic_weight().abridged(), Some(uncertain!(63.546, 0.003)));
//     assert_eq!(Zn.standard_atomic_weight().abridged(), Some(uncertain!(65.38, 0.02)));
//     assert_eq!(Ga.standard_atomic_weight().abridged(), Some(uncertain!(69.723, 0.001)));
//     assert_eq!(Ge.standard_atomic_weight().abridged(), Some(uncertain!(72.630, 0.008)));
//     assert_eq!(As.standard_atomic_weight().abridged(), Some(uncertain!(74.922, 0.001)));
//     assert_eq!(Se.standard_atomic_weight().abridged(), Some(uncertain!(78.971, 0.008)));
//     assert_eq!(Br.standard_atomic_weight().abridged(), Some(uncertain!(79.904, 0.003)));
//     assert_eq!(Kr.standard_atomic_weight().abridged(), Some(uncertain!(83.798, 0.002)));
//     assert_eq!(Rb.standard_atomic_weight().abridged(), Some(uncertain!(85.468, 0.001)));
//     assert_eq!(Sr.standard_atomic_weight().abridged(), Some(uncertain!(87.62, 0.01)));
//     assert_eq!(Y.standard_atomic_weight().abridged(), Some(uncertain!(88.906, 0.001)));
//     assert_eq!(Zr.standard_atomic_weight().abridged(), Some(uncertain!(91.224, 0.002)));
//     assert_eq!(Nb.standard_atomic_weight().abridged(), Some(uncertain!(92.906, 0.001)));
//     assert_eq!(Mo.standard_atomic_weight().abridged(), Some(uncertain!(95.95, 0.01)));
//     assert_eq!(Tc.standard_atomic_weight().abridged(), None);
//     assert_eq!(Ru.standard_atomic_weight().abridged(), Some(uncertain!(101.07, 0.02)));
//     assert_eq!(Rh.standard_atomic_weight().abridged(), Some(uncertain!(102.91, 0.01)));
//     assert_eq!(Pd.standard_atomic_weight().abridged(), Some(uncertain!(106.42, 0.01)));
//     assert_eq!(Ag.standard_atomic_weight().abridged(), Some(uncertain!(107.87, 0.01)));
//     assert_eq!(Cd.standard_atomic_weight().abridged(), Some(uncertain!(112.41, 0.01)));
//     assert_eq!(In.standard_atomic_weight().abridged(), Some(uncertain!(114.82, 0.01)));
//     assert_eq!(Sn.standard_atomic_weight().abridged(), Some(uncertain!(118.71, 0.01)));
//     assert_eq!(Sb.standard_atomic_weight().abridged(), Some(uncertain!(121.76, 0.01)));
//     assert_eq!(Te.standard_atomic_weight().abridged(), Some(uncertain!(127.60, 0.03)));
//     assert_eq!(I.standard_atomic_weight().abridged(), Some(uncertain!(126.90, 0.01)));
//     assert_eq!(Xe.standard_atomic_weight().abridged(), Some(uncertain!(131.29, 0.01)));
//     assert_eq!(Cs.standard_atomic_weight().abridged(), Some(uncertain!(132.91, 0.01)));
//     assert_eq!(Ba.standard_atomic_weight().abridged(), Some(uncertain!(137.33, 0.01)));
//     assert_eq!(La.standard_atomic_weight().abridged(), Some(uncertain!(138.91, 0.01)));
//     assert_eq!(Ce.standard_atomic_weight().abridged(), Some(uncertain!(140.12, 0.01)));
//     assert_eq!(Pr.standard_atomic_weight().abridged(), Some(uncertain!(140.91, 0.01)));
//     assert_eq!(Nd.standard_atomic_weight().abridged(), Some(uncertain!(144.24, 0.01)));
//     assert_eq!(Pm.standard_atomic_weight().abridged(), None);
//     assert_eq!(Sm.standard_atomic_weight().abridged(), Some(uncertain!(150.36, 0.02)));
//     assert_eq!(Eu.standard_atomic_weight().abridged(), Some(uncertain!(151.96, 0.01)));
//     assert_eq!(Gd.standard_atomic_weight().abridged(), Some(uncertain!(157.25, 0.03)));
//     assert_eq!(Tb.standard_atomic_weight().abridged(), Some(uncertain!(158.93, 0.01)));
//     assert_eq!(Dy.standard_atomic_weight().abridged(), Some(uncertain!(162.50, 0.01)));
//     assert_eq!(Ho.standard_atomic_weight().abridged(), Some(uncertain!(164.93, 0.01)));
//     assert_eq!(Er.standard_atomic_weight().abridged(), Some(uncertain!(167.26, 0.01)));
//     assert_eq!(Tm.standard_atomic_weight().abridged(), Some(uncertain!(168.93, 0.01)));
//     assert_eq!(Yb.standard_atomic_weight().abridged(), Some(uncertain!(173.05, 0.02)));
//     assert_eq!(Lu.standard_atomic_weight().abridged(), Some(uncertain!(174.97, 0.01)));
//     assert_eq!(Hf.standard_atomic_weight().abridged(), Some(uncertain!(178.49, 0.01)));
//     assert_eq!(Ta.standard_atomic_weight().abridged(), Some(uncertain!(180.95, 0.01)));
//     assert_eq!(W.standard_atomic_weight().abridged(), Some(uncertain!(183.84, 0.01)));
//     assert_eq!(Re.standard_atomic_weight().abridged(), Some(uncertain!(186.21, 0.01)));
//     assert_eq!(Os.standard_atomic_weight().abridged(), Some(uncertain!(190.23, 0.03)));
//     assert_eq!(Ir.standard_atomic_weight().abridged(), Some(uncertain!(192.22, 0.01)));
//     assert_eq!(Pt.standard_atomic_weight().abridged(), Some(uncertain!(195.08, 0.02)));
//     assert_eq!(Au.standard_atomic_weight().abridged(), Some(uncertain!(196.97, 0.01)));
//     assert_eq!(Hg.standard_atomic_weight().abridged(), Some(uncertain!(200.59, 0.01)));
//     assert_eq!(Tl.standard_atomic_weight().abridged(), Some(uncertain!(204.38, 0.01)));
//     assert_eq!(Pb.standard_atomic_weight().abridged(), Some(uncertain!(207.2, 1.1)));
//     assert_eq!(Bi.standard_atomic_weight().abridged(), Some(uncertain!(208.98, 0.01)));
//     assert_eq!(Po.standard_atomic_weight().abridged(), None);
//     assert_eq!(At.standard_atomic_weight().abridged(), None);
//     assert_eq!(Rn.standard_atomic_weight().abridged(), None);
//     assert_eq!(Fr.standard_atomic_weight().abridged(), None);
//     assert_eq!(Ra.standard_atomic_weight().abridged(), None);
//     assert_eq!(Ac.standard_atomic_weight().abridged(), None);
//     assert_eq!(Th.standard_atomic_weight().abridged(), Some(uncertain!(232.04, 0.01)));
//     assert_eq!(Pa.standard_atomic_weight().abridged(), Some(uncertain!(231.04, 0.01)));
//     assert_eq!(U.standard_atomic_weight().abridged(), Some(uncertain!(238.03, 0.01)));
//     assert_eq!(Np.standard_atomic_weight().abridged(), None);
//     assert_eq!(Pu.standard_atomic_weight().abridged(), None);
//     assert_eq!(Am.standard_atomic_weight().abridged(), None);
//     assert_eq!(Cm.standard_atomic_weight().abridged(), None);
//     assert_eq!(Bk.standard_atomic_weight().abridged(), None);
//     assert_eq!(Cf.standard_atomic_weight().abridged(), None);
//     assert_eq!(Es.standard_atomic_weight().abridged(), None);
//     assert_eq!(Fm.standard_atomic_weight().abridged(), None);
//     assert_eq!(Md.standard_atomic_weight().abridged(), None);
//     assert_eq!(No.standard_atomic_weight().abridged(), None);
//     assert_eq!(Lr.standard_atomic_weight().abridged(), None);
//     assert_eq!(Rf.standard_atomic_weight().abridged(), None);
//     assert_eq!(Db.standard_atomic_weight().abridged(), None);
//     assert_eq!(Sg.standard_atomic_weight().abridged(), None);
//     assert_eq!(Bh.standard_atomic_weight().abridged(), None);
//     assert_eq!(Hs.standard_atomic_weight().abridged(), None);
//     assert_eq!(Mt.standard_atomic_weight().abridged(), None);
//     assert_eq!(Ds.standard_atomic_weight().abridged(), None);
//     assert_eq!(Rg.standard_atomic_weight().abridged(), None);
//     assert_eq!(Cn.standard_atomic_weight().abridged(), None);
//     assert_eq!(Nh.standard_atomic_weight().abridged(), None);
//     assert_eq!(Fl.standard_atomic_weight().abridged(), None);
//     assert_eq!(Mc.standard_atomic_weight().abridged(), None);
//     assert_eq!(Lv.standard_atomic_weight().abridged(), None);
//     assert_eq!(Ts.standard_atomic_weight().abridged(), None);
//     assert_eq!(Og.standard_atomic_weight().abridged(), None);
// }
