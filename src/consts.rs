// Physical constants TODO: Add sources
// <https://www.physics.nist.gov/cgi-bin/cuu/Value?eshbar|search_for=elecmag_in!>
// Physicochemical constants
// <https://www.physics.nist.gov/cgi-bin/cuu/Value?na|search_for=physchem_in!>
pub const AVOGADRO_PER_MOL: f64 = 6.022_140_76e23; // mol^-1
pub const BOLTZMANN_J_PER_K: f64 = 1.380_649e-23; // J K^-1
pub const GAS_CONSTANT_J_PER_MOL_K: f64 = 8.314_462_618_153_24; // J K^-1 mol^-1
pub const MOLAR_VOLUME_M3_PER_MOL: f64 = 22.413_969_9e-3; // m^3 mol^-1 (ideal gas at 0°C and 1 atm)
pub const MOLAR_MASS_KG_PER_MOL: f64 = 0.999_999_999_65e-3; // kg mol^-1
pub const MOLAR_PLANCK_J_S_PER_MOL: f64 = 3.990_312_71e-10; // J s mol^-1
pub const AMU_KG: f64 = 1.660_539_066_60e-27; // kg

// Atomic constants
// <https:// www.physics.nist.gov/cgi-bin/cuu/Value?bohrrada0|search_for=atomnuc!>
pub const HARTREE_J: f64 = 4.359_744_722_207_1e-18; // J
pub const HARTREE_EV: f64 = 27.211_386_245_988; // eV
pub const BOHR_MAGNETON_J_PER_T: f64 = 9.274_010_078_3e-24; // J T^-1
pub const BOHR_RADIUS_M: f64 = 5.291_772_109_03e-11; // m
pub const NEUTRON_MASS_KG: f64 = 1.674_927_498_04e-27; // kg
pub const NEUTRON_MASS_AMU: f64 = NEUTRON_MASS_KG / AMU_KG;
pub const PROTON_MASS_KG: f64 = 1.672_621_923_69e-27; // kg
pub const PROTON_MASS_AMU: f64 = PROTON_MASS_KG / AMU_KG;
pub const ELECTRON_MASS_KG: f64 = 9.109_383_701_5e-31; // kg
pub const ELECTRON_MASS_AMU: f64 = ELECTRON_MASS_KG / AMU_KG;
pub const FINE_STRUCTURE: f64 = 7.297_352_569_3e-3; // dimensionless
pub const RYDBERG_PER_M: f64 = 10_973_731.568_160; // m^-1

// Fundamental physical constants
// <https:// physics.nist.gov/cgi-bin/cuu/Value?c|search_for=universal_in!>
pub const PLANCK_J_S: f64 = 6.626_070_15e-34; // J s
pub const HBAR_J_S: f64 = PLANCK_J_S / (2.0 * std::f64::consts::PI); // J s (reduced Planck constant)
pub const PLANCK_LENGTH_M: f64 = 1.616255e-35; // m
pub const PLANCK_MASS_KG: f64 = 2.176434e-8; // kg
pub const PLANCK_TIME_S: f64 = 5.391247e-44; // s
pub const SPEED_OF_LIGHT_M_PER_S: f64 = 299_792_458.0; // m s^-1
pub const VACUUM_PERMITTIVITY_F_PER_M: f64 = 8.854_187_812_8e-12; // F m^-1
pub const VACUUM_PERMEABILITY_N_PER_A2: f64 = 1.256_637_062_12e-6; // N A^-2
pub const STANDARD_GRAVITY_M_PER_S2: f64 = 9.806_65; // m s^-2
pub const ELEMENTARY_CHARGE_C: f64 = 1.602_176_634e-19; // C
pub const ELECTRON_VOLT_J: f64 = 1.602_176_634e-19; // J
pub const FARADAY_C_PER_MOL: f64 = 96_485.332_12; // C mol^-1
pub const COULOMB_N_M2_PER_C2: f64 = 1.0 / (4.0 * std::f64::consts::PI * VACUUM_PERMITTIVITY_F_PER_M); // N m^2 C^-2 (Coulomb's constant)
pub const CALORIE_J: f64 = 4.184; // J
pub const ATMOSPHERE_PA: f64 = 101_325.0; // Pa
pub const ZERO_CELSIUS_K: f64 = 273.15; // K
pub const ANGSTROM_M: f64 = 1e-10; // m
pub const STEFAN_BOLTZMANN_W_PER_M2_K4: f64 = 5.670_374_419e-8; // W m^-2 K^-4
pub const NUCLEAR_MAGNETON_J_PER_T: f64 = 5.050_783_746e-27; // J T^-1
pub const HARTREE_KJ_PER_MOL: f64 = HARTREE_J * AVOGADRO_PER_MOL * 1e-3; // kJ mol^-1

// The values are taken from: <https:// physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&isotype=some
// The padding is necessary to make the array index match the atomic number
// Some elements are not stable and therefore do not have a standard atomic weight
pub const STANDARD_ATOMIC_WEIGHTS: [Option<f64>; 119] = [
    None,                     // Dummy value
    Some(1.007975),           // H
    Some(4.002602),           // He
    Some(6.967499999999999),  // Li
    Some(9.0121831),          // Be
    Some(10.8135),            // B
    Some(12.0106),            // C
    Some(14.006855),          // N
    Some(15.9994),            // O
    Some(18.998403163),       // F
    Some(20.1797),            // Ne
    Some(22.98976928),        // Na
    Some(24.3055),            // Mg
    Some(26.9815385),         // Al
    Some(28.085),             // Si
    Some(30.973761998),       // P
    Some(32.067499999999995), // S
    Some(35.451499999999996), // Cl
    Some(39.948),             // Ar
    Some(39.0983),            // K
    Some(40.078),             // Ca
    Some(44.955908),          // Sc
    Some(47.867),             // Ti
    Some(50.9415),            // V
    Some(51.9961),            // Cr
    Some(54.938044),          // Mn
    Some(55.845),             // Fe
    Some(58.933194),          // Co
    Some(58.6934),            // Ni
    Some(63.546),             // Cu
    Some(65.38),              // Zn
    Some(69.723),             // Ga
    Some(72.63),              // Ge
    Some(74.921595),          // As
    Some(78.971),             // Se
    Some(79.904),             // Br
    Some(83.798),             // Kr
    Some(85.4678),            // Rb
    Some(87.62),              // Sr
    Some(88.90584),           // Y
    Some(91.224),             // Zr
    Some(92.90637),           // Nb
    Some(95.95),              // Mo
    Some(98.0),               // Tc
    Some(101.07),             // Ru
    Some(102.9055),           // Rh
    Some(106.42),             // Pd
    Some(107.8682),           // Ag
    Some(112.414),            // Cd
    Some(114.818),            // In
    Some(118.71),             // Sn
    Some(121.76),             // Sb
    Some(127.6),              // Te
    Some(126.90447),          // I
    Some(131.293),            // Xe
    Some(132.90545196),       // Cs
    Some(137.327),            // Ba
    Some(138.90547),          // La
    Some(140.116),            // Ce
    Some(140.90766),          // Pr
    Some(144.242),            // Nd
    Some(145.0),              // Pm
    Some(150.36),             // Sm
    Some(151.964),            // Eu
    Some(157.25),             // Gd
    Some(158.92535),          // Tb
    Some(162.5),              // Dy
    Some(164.93033),          // Ho
    Some(167.259),            // Er
    Some(168.93422),          // Tm
    Some(173.054),            // Yb
    Some(174.9668),           // Lu
    Some(178.49),             // Hf
    Some(180.94788),          // Ta
    Some(183.84),             // W
    Some(186.207),            // Re
    Some(190.23),             // Os
    Some(192.217),            // Ir
    Some(195.084),            // Pt
    Some(196.966569),         // Au
    Some(200.592),            // Hg
    Some(204.3835),           // Tl
    Some(207.2),              // Pb
    Some(208.9804),           // Bi
    Some(209.0),              // Po
    Some(210.0),              // At
    Some(222.0),              // Rn
    Some(223.0),              // Fr
    Some(226.0),              // Ra
    Some(227.0),              // Ac
    Some(232.0377),           // Th
    Some(231.03588),          // Pa
    Some(238.02891),          // U
    Some(237.0),              // Np
    Some(244.0),              // Pu
    None,                     // Am
    None,                     // Cm
    None,                     // Bk
    None,                     // Cf
    None,                     // Es
    None,                     // Fm
    None,                     // Md
    None,                     // No
    None,                     // Lr
    None,                     // Rf
    None,                     // Db
    None,                     // Sg
    None,                     // Bh
    None,                     // Hs
    None,                     // Mt
    None,                     // Ds
    None,                     // Rg
    None,                     // Cn
    None,                     // Nh
    None,                     // Fl
    None,                     // Mc
    None,                     // Lv
    None,                     // Ts
    None,                     // Og
];

// This array represent the covalent radii of the elements in angstroms (Å) only single bond radii are used as they are the largest and therefore most restrictive
// The values up to CM are taken from: <https:// doi.org/10.1002/chem.200800987>
// The values from CF to OG are taken from: <https:// en.wikipedia.org/wiki/Atomic_radii_of_the_elements_(data_page)>
pub const COVALENT_RADII: [f64; 119] = [
    0.0,  // Dummy value
    0.31, // H
    0.28, // HE
    1.28, // LI
    0.96, // BE
    0.84, // B
    0.76, // C
    0.71, // N
    0.66, // O
    0.57, // F
    0.58, // NE
    1.66, // NA
    1.41, // MG
    1.21, // AL
    1.11, // SI
    1.07, // P
    1.05, // S
    1.02, // CL
    1.06, // AR
    2.03, // K
    1.76, // CA
    1.70, // SC
    1.60, // TI
    1.53, // V
    1.39, // CR
    1.39, // MN
    1.32, // FE
    1.26, // CO
    1.24, // NI
    1.32, // CU
    1.22, // ZN
    1.22, // GA
    1.20, // GE
    1.19, // AS
    1.20, // SE
    1.20, // BR
    1.16, // KR
    2.20, // RB
    1.95, // SR
    1.90, // Y
    1.75, // ZR
    1.64, // NB
    1.54, // MO
    1.47, // TC
    1.46, // RU
    1.42, // RH
    1.39, // PD
    1.45, // AG
    1.44, // CD
    1.42, // IN
    1.39, // SN
    1.39, // SB
    1.38, // TE
    1.39, // I
    1.40, // XE
    2.44, // CS
    2.15, // BA
    2.07, // LA
    2.04, // CE
    2.03, // PR
    2.01, // ND
    1.99, // PM
    1.98, // SM
    1.98, // EU
    1.96, // GD
    1.94, // TB
    1.92, // DY
    1.92, // HO
    1.89, // ER
    1.90, // TM
    1.87, // YB
    1.75, // LU
    1.87, // HF
    1.70, // TA
    1.62, // W
    1.51, // RE
    1.44, // OS
    1.41, // IR
    1.36, // PT
    1.36, // AU
    1.32, // HG
    1.45, // TL
    1.46, // PB
    1.48, // BI
    1.40, // PO
    1.50, // AT
    1.50, // RN
    2.60, // FR
    2.21, // RA
    2.15, // AC
    2.06, // TH
    2.05, // PA
    1.96, // U
    1.90, // NP
    1.87, // PU
    1.80, // AM
    1.69, // CM
    1.69, // BK
    1.68, // CF
    1.65, // ES
    1.67, // FM
    1.73, // MD
    1.76, // NO
    1.61, // LR
    1.57, // RF
    1.49, // DB
    1.43, // SG
    1.41, // BH
    1.34, // HS
    1.29, // MT
    1.28, // DS
    1.21, // RG
    1.22, // CN
    1.36, // NH
    1.43, // FL
    1.62, // MC
    1.75, // LV
    1.65, // TS
    1.57, // OG
];

// The following values were taken from rdkit and are subject to change
// Source: <https://raw.githubusercontent.com/rdkit/rdkit/9e1dbd2f336b8c381d4e7ef8d8ce8b9641ec3bc8/Code/GraphMol/atomic_data.cpp>
pub const VALENCIES: [Option<[Option<i8>; 4]>; 119] = [
    None,                                       //Padding
    Some([Some(1), None, None, None]),          //H
    Some([Some(0), None, None, None]),          //HE
    Some([Some(1), Some(-1), None, None]),      //LI
    Some([Some(2), None, None, None]),          //BE
    Some([Some(3), None, None, None]),          //B
    Some([Some(4), None, None, None]),          //C
    Some([Some(3), None, None, None]),          //N
    Some([Some(2), None, None, None]),          //O
    Some([Some(1), None, None, None]),          //F
    Some([Some(0), None, None, None]),          //NE
    Some([Some(1), Some(-1), None, None]),      //NA
    Some([Some(2), Some(-1), None, None]),      //MG
    Some([Some(3), Some(6), None, None]),       //AL
    Some([Some(4), Some(6), None, None]),       //SI
    Some([Some(3), Some(5), Some(7), None]),    //P
    Some([Some(2), Some(4), Some(6), None]),    //S
    Some([Some(1), None, None, None]),          //CL
    Some([Some(0), None, None, None]),          //AR
    Some([Some(1), Some(-1), None, None]),      //K
    Some([Some(2), Some(-1), None, None]),      //CA
    None,                                       //SC
    None,                                       //TI
    None,                                       //V
    None,                                       //CR
    None,                                       //MN
    None,                                       //FE
    None,                                       //CO
    None,                                       //NI
    None,                                       //CU
    None,                                       //ZN
    Some([Some(3), None, None, None]),          //GA
    Some([Some(4), None, None, None]),          //GE
    Some([Some(3), Some(5), Some(7), None]),    //AS
    Some([Some(2), Some(4), Some(6), None]),    //SE
    Some([Some(1), None, None, None]),          //BR
    Some([Some(0), None, None, None]),          //KR
    Some([Some(1), None, None, None]),          //RB
    Some([Some(2), Some(-1), None, None]),      //SR
    None,                                       //Y
    None,                                       //ZR
    None,                                       //NB
    None,                                       //MO
    None,                                       //TC
    None,                                       //RU
    None,                                       //RH
    None,                                       //PD
    None,                                       //AG
    None,                                       //CD
    Some([Some(3), None, None, None]),          //IN
    Some([Some(2), Some(4), None, None]),       //SN
    Some([Some(3), Some(5), Some(7), None]),    //SB
    Some([Some(2), Some(4), Some(6), None]),    //TE
    Some([Some(1), Some(3), Some(5), None]),    //I
    Some([Some(0), Some(2), Some(4), Some(6)]), //XE
    Some([Some(1), None, None, None]),          //CS
    Some([Some(2), Some(-1), None, None]),      //BA
    None,                                       //LA
    None,                                       //CE
    None,                                       //PR
    None,                                       //ND
    None,                                       //PM
    None,                                       //SM
    None,                                       //EU
    None,                                       //GD
    None,                                       //TB
    None,                                       //DY
    None,                                       //HO
    None,                                       //ER
    None,                                       //TM
    None,                                       //YB
    None,                                       //LU
    None,                                       //HF
    None,                                       //TA
    None,                                       //W
    None,                                       //RE
    None,                                       //OS
    None,                                       //IR
    None,                                       //PT
    None,                                       //AU
    None,                                       //HG
    Some([Some(3), None, None, None]),          //TL
    Some([Some(2), Some(4), None, None]),       //PB
    Some([Some(3), Some(5), Some(7), None]),    //BI
    Some([Some(2), Some(4), Some(6), None]),    //PO
    Some([Some(1), Some(3), Some(5), None]),    //AT
    Some([Some(0), None, None, None]),          //RN
    Some([Some(1), None, None, None]),          //FR
    Some([Some(2), Some(-1), None, None]),      //RA
    None,                                       //AC
    None,                                       //TH
    None,                                       //PA
    None,                                       //U
    None,                                       //NP
    None,                                       //PU
    None,                                       //AM
    None,                                       //CM
    None,                                       //BK
    None,                                       //CF
    None,                                       //ES
    None,                                       //FM
    None,                                       //MD
    None,                                       //NO
    None,                                       //LR
    None,                                       //RF
    None,                                       //DB
    None,                                       //SG
    None,                                       //BH
    None,                                       //HS
    None,                                       //MT
    None,                                       //DS
    None,                                       //RG
    None,                                       //CN
    None,                                       //NH
    None,                                       //FL
    None,                                       //MC
    None,                                       //LV
    None,                                       //TS
    None,                                       //OG
];

// Mass of each element's MOST ABUNDANT isotope, indexed by atomic number.
//
// Derived from ISOTOPES rather than written out, because the two are the same
// fact and keeping them as two hand-maintained tables is what went wrong: this
// array used to hold each element's LIGHTEST isotope instead, which differs
// from the most abundant one for 39 elements. Helium read 3.0160 (helium-3,
// abundance 0.0001%) instead of 4.0026, lithium 6.015 instead of 7.016, iron
// 53.940 instead of 55.935. Anything computing an exact mass was wrong for
// every molecule containing one of them, silently, because both values are
// plausible masses for the element. Deriving it makes that class of drift
// impossible.
//
// ISOTOPES is ordered by decreasing abundance, so entry 0 is the one wanted.
// For the 34 elements with no stable isotope ISOTOPES is empty and the value
// comes from UNSTABLE_ELEMENT_MASSES below.
pub const MONOISOTOPIC_MASSES: [f64; 119] = build_monoisotopic_masses();

// Reference nuclide masses for the elements that have no stable isotope, where
// ISOTOPES has nothing to offer.
//
// The convention is the IUPAC one: the longest-lived nuclide. That is also the
// nuclide STANDARD_ATOMIC_WEIGHTS names for these elements, and the five marked
// below used to contradict it -- radon read Rn-211 (15 hours) where the weight
// table says Rn-222 (3.8 days), radium Ra-223 against Ra-226, plutonium Pu-238
// against Pu-244. Two tables in one crate naming different nuclides for the
// same element is the same failure as the drift above, one table further out.
//
// Elements 95 and up have no standard atomic weight to check against, so those
// entries stand on their existing provenance, which is not recorded.
const UNSTABLE_ELEMENT_MASSES: [(u8, f64); 34] = [
    (43, 97.907216),    // Tc, was Tc-97
    (61, 144.9127559),  // Pm
    (84, 208.9824308),  // Po
    (85, 209.9871479),  // At
    (86, 222.0175777),  // Rn, was Rn-211
    (87, 223.019736),   // Fr
    (88, 226.0254098),  // Ra, was Ra-223
    (89, 227.0277523),  // Ac
    (93, 237.0481734),  // Np, was Np-236
    (94, 244.064204),   // Pu, was Pu-238
    (95, 241.0568293),  // Am
    (96, 243.0613893),  // Cm
    (97, 247.0703073),  // Bk
    (98, 249.0748539),  // Cf
    (99, 252.08298),    // Es
    (100, 257.0951061), // Fm
    (101, 258.0984315), // Md
    (102, 259.10103),   // No
    (103, 262.10961),   // Lr
    (104, 267.12179),   // Rf
    (105, 268.12567),   // Db
    (106, 271.13393),   // Sg
    (107, 272.13826),   // Bh
    (108, 270.13429),   // Hs
    (109, 276.15159),   // Mt
    (110, 281.16451),   // Ds
    (111, 280.16514),   // Rg
    (112, 285.17712),   // Cn
    (113, 284.17873),   // Nh
    (114, 289.19042),   // Fl
    (115, 288.19274),   // Mc
    (116, 293.20449),   // Lv
    (117, 292.20746),   // Ts
    (118, 294.21392),   // Og
];

const fn build_monoisotopic_masses() -> [f64; 119] {
    let mut masses = [0.0; 119];
    let mut z = 1;
    while z < 119 {
        masses[z] = match ISOTOPES[z][0] {
            Some(isotope) => isotope.mass,
            None => unstable_element_mass(z as u8),
        };
        z += 1;
    }
    masses
}

const fn unstable_element_mass(atomic_number: u8) -> f64 {
    let mut index = 0;
    while index < UNSTABLE_ELEMENT_MASSES.len() {
        if UNSTABLE_ELEMENT_MASSES[index].0 == atomic_number {
            return UNSTABLE_ELEMENT_MASSES[index].1;
        }
        index += 1;
    }
    0.0
}

// These values are simply based on the respective elements group in the periodic table
pub const VALENCE_ELECTRONS: [u8; 119] = [
    0,  // Dummy value
    1,  // H
    2,  // HE
    1,  // LI
    2,  // BE
    3,  // B
    4,  // C
    5,  // N
    6,  // O
    7,  // F
    8,  // NE
    1,  // NA
    2,  // MG
    3,  // AL
    4,  // SI
    5,  // P
    6,  // S
    7,  // CL
    8,  // AR
    1,  // K
    2,  // CA
    3,  // SC
    4,  // TI
    5,  // V
    6,  // CR
    7,  // MN
    8,  // FE
    9,  // CO
    10, // NI
    11, // CU
    2,  // ZN
    3,  // GA
    4,  // GE
    5,  // AS
    6,  // SE
    7,  // BR
    8,  // KR
    1,  // RB
    2,  // SR
    3,  // Y
    4,  // ZR
    5,  // NB
    6,  // MO
    7,  // TC
    8,  // RU
    9,  // RH
    10, // PD
    11, // AG
    2,  // CD
    3,  // IN
    4,  // SN
    5,  // SB
    6,  // TE
    7,  // I
    8,  // XE
    1,  // CS
    2,  // BA
    3,  // LA
    4,  // CE
    3,  // PR
    4,  // ND
    5,  // PM
    6,  // SM
    7,  // EU
    8,  // GD
    9,  // TB
    10, // DY
    11, // HO
    12, // ER
    13, // TM
    14, // YB
    15, // LU
    4,  // HF
    5,  // TA
    6,  // W
    7,  // RE
    8,  // OS
    9,  // IR
    10, // PT
    11, // AU
    2,  // HG
    3,  // TL
    4,  // PB
    5,  // BI
    6,  // PO
    7,  // AT
    8,  // RN
    1,  // FR
    2,  // RA
    3,  // AC
    4,  // TH
    3,  // PA
    4,  // U
    5,  // NP
    6,  // PU
    7,  // AM
    8,  // CM
    9,  // BK
    10, // CF
    11, // ES
    12, // FM
    13, // MD
    14, // NO
    3,  // LR
    4,  // RF
    5,  // DB
    6,  // SG
    7,  // BH
    8,  // HS
    9,  // MT
    10, // DS
    11, // RG
    12, // CN
    3,  // NH
    4,  // FL
    5,  // MC
    6,  // LV
    7,  // TS
    8,  // OG
];

pub const ATOMIC_SYMBOLS: [&str; 119] = [
    "None", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
    "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge",
    "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd",
    "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd",
    "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg",
    "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm",
    "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn",
    "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
];

macro_rules! generate_match {
    ( $target:expr; $(($key:literal, $result:expr)),*$(,)? ) => {
        {
            match $target {
                $(
                    $key => Some($result),
                )*
                _ => None,
            }
        }
    };
}

/// The atomic number of an element symbol, in any capitalisation.
///
/// `"Fe"`, `"FE"` and `"fe"` all give iron. It used to accept only the
/// uppercase form, which meant the crate would not take back the symbol it had
/// just handed out -- `ATOMIC_SYMBOLS` and [`ElementProperties::atomic_symbol`]
/// return the conventional mixed case -- and every caller wrapped this in a
/// `to_uppercase()` that allocated a `String` per lookup.
///
/// Folding case here costs nothing instead. An element symbol is one or two
/// ASCII letters, so the uppercase form fits in two bytes on the stack, and the
/// comparison happens against byte strings rather than `&str`. Anything longer
/// than two bytes, or not ASCII, cannot be a symbol and returns `None` before
/// the table is consulted.
pub fn atomic_numbers(symbol: &str) -> Option<u8> {
    let mut uppercase = [0u8; 2];
    let length = match *symbol.as_bytes() {
        [first] => {
            uppercase[0] = first.to_ascii_uppercase();
            1
        }
        [first, second] => {
            uppercase[0] = first.to_ascii_uppercase();
            uppercase[1] = second.to_ascii_uppercase();
            2
        }
        _ => return None,
    };

    generate_match!(&uppercase[..length];
   (b"H", 1),
   (b"HE", 2),
   (b"LI", 3),
   (b"BE", 4),
   (b"B", 5),
   (b"C", 6),
   (b"N", 7),
   (b"O", 8),
   (b"F", 9),
   (b"NE", 10),
    (b"NA", 11),
   (b"MG", 12),
   (b"AL", 13),
   (b"SI", 14),
   (b"P", 15),
   (b"S", 16),
   (b"CL", 17),
   (b"AR", 18),
   (b"K", 19),
   (b"CA", 20),
   (b"SC", 21),
   (b"TI", 22),
   (b"V", 23),
   (b"CR", 24),
   (b"MN", 25),
   (b"FE", 26),
   (b"CO", 27),
   (b"NI", 28),
   (b"CU", 29),
   (b"ZN", 30),
   (b"GA", 31),
   (b"GE", 32),
   (b"AS", 33),
   (b"SE", 34),
   (b"BR", 35),
   (b"KR", 36),
   (b"RB", 37),
   (b"SR", 38),
   (b"Y", 39),
   (b"ZR", 40),
   (b"NB", 41),
   (b"MO", 42),
   (b"TC", 43),
   (b"RU", 44),
   (b"RH", 45),
   (b"PD", 46),
   (b"AG", 47),
   (b"CD", 48),
   (b"IN", 49),
   (b"SN", 50),
   (b"SB", 51),
   (b"TE", 52),
   (b"I", 53),
   (b"XE", 54),
   (b"CS", 55),
   (b"BA", 56),
   (b"LA", 57),
   (b"CE", 58),
   (b"PR", 59),
   (b"ND", 60),
   (b"PM", 61),
   (b"SM", 62),
   (b"EU", 63),
   (b"GD", 64),
   (b"TB", 65),
   (b"DY", 66),
   (b"HO", 67),
   (b"ER", 68),
   (b"TM", 69),
   (b"YB", 70),
   (b"LU", 71),
   (b"HF", 72),
   (b"TA", 73),
   (b"W", 74),
   (b"RE", 75),
   (b"OS", 76),
   (b"IR", 77),
   (b"PT", 78),
   (b"AU", 79),
   (b"HG", 80),
   (b"TL", 81),
   (b"PB", 82),
   (b"BI", 83),
   (b"PO", 84),
   (b"AT", 85),
   (b"RN", 86),
   (b"FR", 87),
   (b"RA", 88),
   (b"AC", 89),
   (b"TH", 90),
   (b"PA", 91),
   (b"U", 92),
   (b"NP", 93),
   (b"PU", 94),
   (b"AM", 95),
   (b"CM", 96),
   (b"BK", 97),
   (b"CF", 98),
   (b"ES", 99),
   (b"FM", 100),
   (b"MD", 101),
   (b"NO", 102),
   (b"LR", 103),
   (b"RF", 104),
   (b"DB", 105),
   (b"SG", 106),
   (b"BH", 107),
   (b"HS", 108),
   (b"MT", 109),
   (b"DS", 110),
   (b"RG", 111),
   (b"CN", 112),
   (b"NH", 113),
   (b"FL", 114),
   (b"MC", 115),
   (b"LV", 116),
   (b"TS", 117),
   (b"OG", 118),)
}

// Electronegativities baused on the Pauling scale, scaled by a factor of 100 to avoid floating point errors
// Sources:
// L. Pauling, The Nature of the Chemical Bond, Cornell Univ., USA, 3rd ed., 1960.
// Allred, A. Louis. "Electronegativity values from thermochemical data." Journal of inorganic and nuclear chemistry 17.3-4 (1961): 215-221.
// Elements 95-102 are based on the work of J. E. Huheey, E. A. Keiter, and R. L. Keiter in their book "Inorganic Chemistry: Principles of Structure and Reactivity"
pub const ELECTRONEGATIVITIES_PAULING: [Option<u16>; 119] = [
    None,      // Dummy value
    Some(220), // H
    None,      // HE
    Some(98),  // LI
    Some(157), // BE
    Some(204), // B
    Some(254), // C
    Some(304), // N
    Some(350), // O
    Some(398), // F
    None,      // NE
    Some(93),  // NA
    Some(131), // MG
    Some(161), // AL
    Some(190), // SI
    Some(219), // P
    Some(258), // S
    Some(316), // CL
    None,      // AR
    Some(82),  // K
    Some(100), // CA
    Some(136), // SC
    Some(154), // TI
    Some(163), // V
    Some(166), // CR
    Some(155), // MN
    Some(183), // FE
    Some(188), // CO
    Some(191), // NI
    Some(190), // CU
    Some(165), // ZN
    Some(181), // GA
    Some(200), // GE
    Some(218), // AS
    Some(254), // SE
    Some(296), // BR
    Some(300), // KR
    Some(82),  // RB
    Some(95),  // SR
    Some(122), // Y
    Some(133), // ZR
    Some(160), // NB
    Some(216), // MO
    Some(190), // TC
    Some(220), // RU
    Some(227), // RH
    Some(220), // PD
    Some(193), // AG
    Some(169), // CD
    Some(178), // IN
    Some(196), // SN
    Some(204), // SB
    Some(210), // TE
    Some(266), // I
    Some(260), // XE
    Some(79),  // CS
    Some(89),  // BA
    Some(110), // LA
    Some(112), // CE
    Some(112), // PR
    Some(113), // ND
    None,      // PM
    Some(117), // SM
    None,      // EU
    Some(120), // GD
    None,      // TB
    Some(122), // DY
    Some(123), // HO
    Some(124), // ER
    Some(125), // TM
    None,      // YB
    Some(127), // LU
    Some(130), // HF
    Some(150), // TA
    Some(236), // W
    Some(190), // RE
    Some(220), // OS
    Some(220), // IR
    Some(227), // PT
    Some(254), // AU
    Some(200), // HG
    Some(162), // TL
    Some(233), // PB
    Some(202), // BI
    Some(200), // PO
    Some(220), // AT
    Some(220), // RN
    Some(79),  // FR
    Some(90),  // RA
    Some(110), // AC
    Some(130), // TH
    Some(150), // PA
    Some(138), // U
    Some(136), // NP
    Some(128), // PU
    Some(130), // AM
    Some(130), // CM
    Some(130), // BK
    Some(130), // CF
    Some(130), // ES
    Some(130), // FM
    Some(130), // MD
    Some(130), // NO
    Some(130), // LR
    None,      // RF
    None,      // DB
    None,      // SG
    None,      // BH
    None,      // HS
    None,      // MT
    None,      // DS
    None,      // RG
    None,      // CN
    None,      // NH
    None,      // FL
    None,      // MC
    None,      // LV
    None,      // TS
    None,      // OG
];

// TODO add references
pub const ELECTRONEGATIVITIES_ALLEN: [Option<u16>; 119] = [
    None,       // Dummy value
    Some(2300), // H
    Some(4160), // HE
    Some(912),  // LI
    Some(1576), // BE
    Some(2051), // B
    Some(2544), // C
    Some(3066), // N
    Some(3610), // O
    Some(4193), // F
    Some(4787), // NE
    Some(869),  // NA
    Some(1293), // MG
    Some(1613), // AL
    Some(1916), // SI
    Some(2253), // P
    Some(2589), // S
    Some(2869), // CL
    Some(3242), // AR
    Some(734),  // K
    Some(1034), // CA
    Some(1190), // SC
    Some(1380), // TI
    Some(1530), // V
    Some(1650), // CR
    Some(1750), // MN
    Some(1800), // FE
    Some(1840), // CO
    Some(1880), // NI
    Some(1850), // CU
    Some(1588), // ZN
    Some(1756), // GA
    Some(1994), // GE
    Some(2211), // AS
    Some(2424), // SE
    Some(2685), // BR
    Some(2966), // KR
    Some(706),  // RB
    Some(963),  // SR
    Some(1120), // Y
    Some(1320), // ZR
    Some(1410), // NB
    Some(1470), // MO
    Some(1510), // TC
    Some(1540), // RU
    Some(1560), // RH
    Some(1580), // PD
    Some(1870), // AG
    Some(1521), // CD
    Some(1656), // IN
    Some(1824), // SN
    Some(1984), // SB
    Some(2158), // TE
    Some(2359), // I
    Some(2582), // XE
    Some(659),  // CS
    Some(881),  // BA
    None,       // LA
    None,       // CE
    None,       // PR
    None,       // ND
    None,       // PM
    None,       // SM
    None,       // EU
    None,       // GD
    None,       // TB
    None,       // DY
    None,       // HO
    None,       // ER
    None,       // TM
    None,       // YB
    Some(1090), // LU
    Some(1160), // HF
    Some(1340), // TA
    Some(1470), // W
    Some(1600), // RE
    Some(1650), // OS
    Some(1680), // IR
    Some(1720), // PT
    Some(1920), // AU
    Some(1765), // HG
    Some(1789), // TL
    Some(1854), // PB
    Some(2009), // BI
    Some(2190), // PO
    Some(2390), // AT
    Some(2600), // RN
    Some(670),  // FR
    Some(890),  // RA
    None,       // AC
    None,       // TH
    None,       // PA
    None,       // U
    None,       // NP
    None,       // PU
    None,       // AM
    None,       // CM
    None,       // BK
    None,       // CF
    None,       // ES
    None,       // FM
    None,       // MD
    None,       // NO
    None,       // LR
    None,       // RF
    None,       // DB
    None,       // SG
    None,       // BH
    None,       // HS
    None,       // MT
    None,       // DS
    None,       // RG
    None,       // CN
    None,       // NH
    None,       // FL
    None,       // MC
    None,       // LV
    None,       // TS
    None,       // OG
];

// This list is taken from Wikipedia, and only includes the major oxidation states: <https:// en.wikipedia.org/wiki/Oxidation_state#List_of_oxidation_states_of_the_elements>
pub const OXIDATION_STATES: [Option<[Option<i8>; 9]>; 119] = [
    None,                                                                //Padding
    Some([Some(-1), Some(1), None, None, None, None, None, None, None]), //H
    Some([Some(0), None, None, None, None, None, None, None, None]),     //HE
    Some([Some(1), None, None, None, None, None, None, None, None]),     //LI
    Some([Some(2), None, None, None, None, None, None, None, None]),     //BE
    Some([Some(3), None, None, None, None, None, None, None, None]),     //B
    Some([
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
    ]), //C
    Some([
        Some(-3),
        Some(3),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //N
    Some([Some(-2), None, None, None, None, None, None, None, None]),    //O
    Some([Some(-1), None, None, None, None, None, None, None, None]),    //F
    Some([Some(0), None, None, None, None, None, None, None, None]),     //NE
    Some([Some(1), None, None, None, None, None, None, None, None]),     //NA
    Some([Some(2), None, None, None, None, None, None, None, None]),     //MG
    Some([Some(3), None, None, None, None, None, None, None, None]),     //AL
    Some([Some(4), None, None, None, None, None, None, None, None]),     //SI
    Some([
        Some(-3),
        Some(3),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //P
    Some([
        Some(-2),
        Some(2),
        Some(4),
        Some(6),
        None,
        None,
        None,
        None,
        None,
    ]), //S
    Some([
        Some(-1),
        Some(1),
        Some(3),
        Some(5),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //CL
    Some([Some(0), None, None, None, None, None, None, None, None]),     //AR
    Some([Some(1), None, None, None, None, None, None, None, None]),     //K
    Some([Some(2), None, None, None, None, None, None, None, None]),     //CA
    Some([Some(3), None, None, None, None, None, None, None, None]),     //SC
    Some([
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TI
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //V
    Some([
        Some(2),
        Some(3),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CR
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //MN
    Some([Some(2), Some(3), None, None, None, None, None, None, None]),  //FE
    Some([Some(2), Some(3), None, None, None, None, None, None, None]),  //CO
    Some([Some(2), None, None, None, None, None, None, None, None]),     //NI
    Some([Some(1), Some(2), None, None, None, None, None, None, None]),  //CU
    Some([Some(2), None, None, None, None, None, None, None, None]),     //ZN
    Some([Some(3), None, None, None, None, None, None, None, None]),     //GA
    Some([Some(2), Some(4), None, None, None, None, None, None, None]),  //GE
    Some([
        Some(-3),
        Some(3),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AS
    Some([
        Some(-2),
        Some(2),
        Some(4),
        Some(6),
        None,
        None,
        None,
        None,
        None,
    ]), //SE
    Some([
        Some(-1),
        Some(1),
        Some(3),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //BR
    Some([Some(0), None, None, None, None, None, None, None, None]),     //KR
    Some([Some(1), None, None, None, None, None, None, None, None]),     //RB
    Some([Some(2), None, None, None, None, None, None, None, None]),     //SR
    Some([Some(3), None, None, None, None, None, None, None, None]),     //Y
    Some([Some(4), None, None, None, None, None, None, None, None]),     //ZR
    Some([Some(5), None, None, None, None, None, None, None, None]),     //NB
    Some([Some(4), Some(6), None, None, None, None, None, None, None]),  //MO
    Some([Some(4), Some(7), None, None, None, None, None, None, None]),  //TC
    Some([Some(3), Some(4), None, None, None, None, None, None, None]),  //RU
    Some([Some(3), None, None, None, None, None, None, None, None]),     //RH
    Some([
        Some(0),
        Some(2),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PD
    Some([Some(1), None, None, None, None, None, None, None, None]),     //AG
    Some([Some(2), None, None, None, None, None, None, None, None]),     //CD
    Some([Some(3), None, None, None, None, None, None, None, None]),     //IN
    Some([Some(2), Some(4), None, None, None, None, None, None, None]),  //SN
    Some([Some(3), Some(5), None, None, None, None, None, None, None]),  //SB
    Some([
        Some(-2),
        Some(2),
        Some(4),
        Some(6),
        None,
        None,
        None,
        None,
        None,
    ]), //TE
    Some([
        Some(-1),
        Some(1),
        Some(3),
        Some(5),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //I
    Some([Some(0), None, None, None, None, None, None, None, None]),     //XE
    Some([Some(1), None, None, None, None, None, None, None, None]),     //CS
    Some([Some(2), None, None, None, None, None, None, None, None]),     //BA
    Some([Some(3), None, None, None, None, None, None, None, None]),     //LA
    Some([Some(3), Some(4), None, None, None, None, None, None, None]),  //CE
    Some([Some(3), None, None, None, None, None, None, None, None]),     //PR
    Some([Some(3), None, None, None, None, None, None, None, None]),     //ND
    Some([Some(3), None, None, None, None, None, None, None, None]),     //PM
    Some([Some(3), None, None, None, None, None, None, None, None]),     //SM
    Some([Some(2), Some(3), None, None, None, None, None, None, None]),  //EU
    Some([Some(3), None, None, None, None, None, None, None, None]),     //GD
    Some([Some(3), None, None, None, None, None, None, None, None]),     //TB
    Some([Some(3), None, None, None, None, None, None, None, None]),     //DY
    Some([Some(3), None, None, None, None, None, None, None, None]),     //HO
    Some([Some(3), None, None, None, None, None, None, None, None]),     //ER
    Some([Some(3), None, None, None, None, None, None, None, None]),     //TM
    Some([Some(3), None, None, None, None, None, None, None, None]),     //YB
    Some([Some(3), None, None, None, None, None, None, None, None]),     //LU
    Some([Some(4), None, None, None, None, None, None, None, None]),     //HF
    Some([Some(5), None, None, None, None, None, None, None, None]),     //TA
    Some([Some(4), Some(6), None, None, None, None, None, None, None]),  //W
    Some([
        Some(3),
        Some(4),
        Some(7),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //RE
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(8),
        None,
        None,
        None,
        None,
        None,
    ]), //OS
    Some([
        Some(1),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //IR
    Some([Some(2), Some(4), None, None, None, None, None, None, None]),  //PT
    Some([Some(1), Some(3), None, None, None, None, None, None, None]),  //AU
    Some([Some(1), Some(2), None, None, None, None, None, None, None]),  //HG
    Some([Some(1), Some(3), None, None, None, None, None, None, None]),  //TL
    Some([Some(2), Some(4), None, None, None, None, None, None, None]),  //PB
    Some([Some(3), None, None, None, None, None, None, None, None]),     //BI
    Some([
        Some(-2),
        Some(2),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PO
    Some([Some(-1), Some(1), None, None, None, None, None, None, None]), //AT
    Some([Some(2), None, None, None, None, None, None, None, None]),     //RN
    Some([Some(1), None, None, None, None, None, None, None, None]),     //FR
    Some([Some(2), None, None, None, None, None, None, None, None]),     //RA
    Some([Some(3), None, None, None, None, None, None, None, None]),     //AC
    Some([Some(4), None, None, None, None, None, None, None, None]),     //TH
    Some([Some(5), None, None, None, None, None, None, None, None]),     //PA
    Some([Some(4), Some(6), None, None, None, None, None, None, None]),  //U
    Some([Some(5), None, None, None, None, None, None, None, None]),     //NP
    Some([Some(4), None, None, None, None, None, None, None, None]),     //PU
    Some([Some(3), None, None, None, None, None, None, None, None]),     //AM
    Some([Some(3), None, None, None, None, None, None, None, None]),     //CM
    Some([Some(3), None, None, None, None, None, None, None, None]),     //BK
    Some([Some(3), None, None, None, None, None, None, None, None]),     //CF
    Some([Some(3), None, None, None, None, None, None, None, None]),     //ES
    Some([Some(3), None, None, None, None, None, None, None, None]),     //FM
    Some([Some(3), None, None, None, None, None, None, None, None]),     //MD
    Some([Some(2), None, None, None, None, None, None, None, None]),     //NO
    Some([Some(3), None, None, None, None, None, None, None, None]),     //LR
    Some([Some(4), None, None, None, None, None, None, None, None]),     //RF
    Some([Some(5), None, None, None, None, None, None, None, None]),     //DB
    Some([Some(6), None, None, None, None, None, None, None, None]),     //SG
    Some([Some(7), None, None, None, None, None, None, None, None]),     //BH
    Some([Some(8), None, None, None, None, None, None, None, None]),     //HS
    None,                                                                //MT
    None,                                                                //DS
    None,                                                                //RG
    Some([Some(2), None, None, None, None, None, None, None, None]),     //CN
    None,                                                                //NH
    None,                                                                //FL
    None,                                                                //MC
    None,                                                                //LV
    None,                                                                //TS
    None,                                                                //OG
];

// This array provides all known oxidation states for each element based on the wikipedia entry
// which lists many sources
pub const OXIDATION_STATES_EXHAUSTIVE: [Option<[Option<i8>; 13]>; 119] = [
    None, //Padding
    Some([
        Some(-1),
        Some(0),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //H
    Some([
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //HE
    Some([
        Some(0),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //LI
    Some([
        Some(0),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //BE
    Some([
        Some(-5),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //B
    Some([
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
    ]), //C
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
    ]), //N
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //O
    Some([
        Some(-1),
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //F
    Some([
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //NE
    Some([
        Some(-1),
        Some(0),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //NA
    Some([
        Some(0),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //MG
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AL
    Some([
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
    ]), //SI
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
    ]), //P
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
    ]), //S
    Some([
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //CL
    Some([
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AR
    Some([
        Some(-1),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //K
    Some([
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CA
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //SC
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TI
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //V
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
    ]), //CR
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
    ]), //MN
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
    ]), //FE
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //CO
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //NI
    Some([
        Some(-2),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CU
    Some([
        Some(-2),
        Some(0),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //ZN
    Some([
        Some(-5),
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
    ]), //GA
    Some([
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
    ]), //GE
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
    ]), //AS
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
    ]), //SE
    Some([
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(7),
        None,
        None,
        None,
        None,
        None,
    ]), //BR
    Some([
        Some(0),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //KR
    Some([
        Some(-1),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //RB
    Some([
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //SR
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //Y
    Some([
        Some(-2),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //ZR
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //NB
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
    ]), //MO
    Some([
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //TC
    Some([
        Some(-4),
        Some(-2),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        None,
        None,
    ]), //RU
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
    ]), //RH
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PD
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AG
    Some([
        Some(-2),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CD
    Some([
        Some(-5),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //IN
    Some([
        Some(-4),
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
    ]), //SN
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
    ]), //SB
    Some([
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
    ]), //TE
    Some([
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
    ]), //I
    Some([
        Some(0),
        Some(2),
        Some(4),
        Some(6),
        Some(8),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //XE
    Some([
        Some(-1),
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CS
    Some([
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //BA
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //LA
    Some([
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CE
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PR
    Some([
        Some(0),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //ND
    Some([
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PM
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //SM
    Some([
        Some(0),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //EU
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //GD
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TB
    Some([
        Some(0),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //DY
    Some([
        Some(0),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //HO
    Some([
        Some(0),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //ER
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TM
    Some([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //YB
    Some([
        Some(0),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //LU
    Some([
        Some(-2),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //HF
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //TA
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
    ]), //W
    Some([
        Some(-3),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
    ]), //RE
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        None,
    ]), //OS
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
    ]), //IR
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
    ]), //PT
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(5),
        None,
        None,
        None,
        None,
        None,
    ]), //AU
    Some([
        Some(-2),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //HG
    Some([
        Some(-5),
        Some(-2),
        Some(-1),
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TL
    Some([
        Some(-4),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
    ]), //PB
    Some([
        Some(-3),
        Some(-2),
        Some(-1),
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
    ]), //BI
    Some([
        Some(-2),
        Some(2),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PO
    Some([
        Some(-1),
        Some(1),
        Some(3),
        Some(5),
        Some(7),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AT
    Some([
        Some(2),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //RN
    Some([
        Some(1),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //FR
    Some([
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //RA
    Some([
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AC
    Some([
        Some(-1),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //TH
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PA
    Some([
        Some(-1),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //U
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //NP
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //PU
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //AM
    Some([
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CM
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //BK
    Some([
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CF
    Some([
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //ES
    Some([
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //FM
    Some([
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //MD
    Some([
        Some(2),
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //NO
    Some([
        Some(3),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //LR
    Some([
        Some(4),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //RF
    Some([
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //DB
    Some([
        Some(0),
        Some(6),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //SG
    Some([
        Some(7),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //BH
    Some([
        Some(8),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //HS
    None, //MT
    None, //DS
    None, //RG
    Some([
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]), //CN
    None, //NH
    None, //FL
    None, //MC
    None, //LV
    None, //TS
    None, //OG
];

#[derive(Clone, Copy)]
pub struct Isotope {
    pub mass: f64,
    pub abundance: f64,
}

pub struct IsotopeIter {
    isotopes: [Option<Isotope>; 4],
    index: usize,
}

impl IsotopeIter {
    pub fn new(isotopes: [Option<Isotope>; 4]) -> IsotopeIter {
        IsotopeIter { isotopes, index: 0 }
    }
}

impl Iterator for IsotopeIter {
    type Item = Isotope;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.isotopes.len() {
            if let Some(isotope) = &self.isotopes[self.index] {
                self.index += 1;
                return Some(*isotope);
            }
            self.index += 1;
        }
        None
    }
}

pub struct OxidationStateIter {
    oxidation_states: [Option<i8>; 9],
    index: usize,
}

impl OxidationStateIter {
    pub fn new(oxidation_states: [Option<i8>; 9]) -> OxidationStateIter {
        OxidationStateIter {
            oxidation_states,
            index: 0,
        }
    }
}

impl Iterator for OxidationStateIter {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.oxidation_states.len() {
            if let Some(oxidation_state) = self.oxidation_states[self.index] {
                self.index += 1;
                return Some(oxidation_state);
            }
            self.index += 1;
        }
        None
    }
}

pub struct OxidationStateExhaustiveIter {
    oxidation_states: [Option<i8>; 13],
    index: usize,
}

impl OxidationStateExhaustiveIter {
    pub fn new(oxidation_states: [Option<i8>; 13]) -> OxidationStateExhaustiveIter {
        OxidationStateExhaustiveIter {
            oxidation_states,
            index: 0,
        }
    }
}

impl Iterator for OxidationStateExhaustiveIter {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.oxidation_states.len() {
            if let Some(oxidation_state) = self.oxidation_states[self.index] {
                self.index += 1;
                return Some(oxidation_state);
            }
            self.index += 1;
        }
        None
    }
}

pub struct ValencyIter {
    valencies: [Option<i8>; 4],
    index: usize,
}

impl ValencyIter {
    fn new(valencies: [Option<i8>; 4]) -> ValencyIter {
        ValencyIter {
            valencies,
            index: 0,
        }
    }
}

impl Iterator for ValencyIter {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.valencies.len() {
            if let Some(valency) = self.valencies[self.index] {
                self.index += 1;
                return Some(valency);
            }
            self.index += 1;
        }
        None
    }
}

// This is to shorten the isotopes array
const fn iso(mass: f64, abundance: f64) -> Isotope {
    Isotope { mass, abundance }
}
// Taken from: <https://physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&all=all&ascii=ascii&isotype=some>
// Zero padding is used to make the array index match the atomic number
// They are sorted by their abundance
pub const ISOTOPES: [[Option<Isotope>; 4]; 119] = [
    [None, None, None, None], // Padding
    [
        Some(iso(1.00782503223, 0.999885)),
        Some(iso(2.01410177812, 0.000115)),
        None,
        None,
    ], // H
    [
        Some(iso(4.00260325413, 0.99999866)),
        Some(Isotope {
            mass: 3.0160293201,
            abundance: 1.34e-06,
        }),
        None,
        None,
    ], // HE
    [
        Some(iso(7.0160034366, 0.9241)),
        Some(iso(6.0151228874, 0.0759)),
        None,
        None,
    ], // LI
    [Some(iso(9.012183065, 1.0)), None, None, None], // BE
    [
        Some(iso(11.00930536, 0.801)),
        Some(iso(10.01293695, 0.199)),
        None,
        None,
    ], // B
    [
        Some(iso(12.0, 0.9893)),
        Some(iso(13.00335483507, 0.0107)),
        None,
        None,
    ], // C
    [
        Some(iso(14.00307400443, 0.99636)),
        Some(iso(15.00010889888, 0.00364)),
        None,
        None,
    ], // N
    [
        Some(iso(15.99491461957, 0.99757)),
        Some(iso(17.99915961286, 0.00205)),
        Some(iso(16.9991317565, 0.00038)),
        None,
    ], // O
    [Some(iso(18.99840316273, 1.0)), None, None, None], // F
    [
        Some(iso(19.9924401762, 0.9048)),
        Some(iso(21.991385114, 0.0925)),
        Some(iso(20.993846685, 0.0027)),
        None,
    ], // NE
    [Some(iso(22.989769282, 1.0)), None, None, None], // NA
    [
        Some(iso(23.985041697, 0.7899)),
        Some(iso(25.982592968, 0.1101)),
        Some(iso(24.985836976, 0.1)),
        None,
    ], // MG
    [Some(iso(26.98153853, 1.0)), None, None, None], // AL
    [
        Some(iso(27.97692653465, 0.92223)),
        Some(iso(28.9764946649, 0.04685)),
        Some(iso(29.973770136, 0.03092)),
        None,
    ], // SI
    [Some(iso(30.97376199842, 1.0)), None, None, None], // P
    [
        Some(iso(31.9720711744, 0.9499)),
        Some(iso(33.967867004, 0.0425)),
        Some(iso(32.9714589098, 0.0075)),
        Some(iso(35.96708071, 0.0001)),
    ], // S
    [
        Some(iso(34.968852682, 0.7576)),
        Some(iso(36.965902602, 0.2424)),
        None,
        None,
    ], // CL
    [
        Some(Isotope {
            mass: 39.9623831237,
            abundance: 0.996035,
        }),
        Some(iso(35.967545105, 0.003336)),
        Some(iso(37.96273211, 0.000629)),
        None,
    ], // AR
    [
        Some(iso(38.9637064864, 0.932581)),
        Some(iso(40.9618252579, 0.067302)),
        Some(iso(39.963998166, 0.000117)),
        None,
    ], // K
    [
        Some(iso(39.962590863, 0.96941)),
        Some(iso(43.95548156, 0.02086)),
        Some(iso(41.95861783, 0.00647)),
        Some(iso(47.95252276, 0.00187)),
    ], // CA
    [Some(iso(44.95590828, 1.0)), None, None, None], // SC
    [
        Some(iso(47.94794198, 0.7372)),
        Some(iso(45.95262772, 0.0825)),
        Some(iso(46.95175879, 0.0744)),
        Some(iso(48.94786568, 0.0541)),
    ], // TI
    [
        Some(iso(50.94395704, 0.9975)),
        Some(iso(49.94715601, 0.0025)),
        None,
        None,
    ], // V
    [
        Some(iso(51.94050623, 0.83789)),
        Some(iso(52.94064815, 0.09501)),
        Some(iso(49.94604183, 0.04345)),
        Some(iso(53.93887916, 0.02365)),
    ], // CR
    [Some(iso(54.93804391, 1.0)), None, None, None], // MN
    [
        Some(iso(55.93493633, 0.91754)),
        Some(iso(53.93960899, 0.05845)),
        Some(iso(56.93539284, 0.02119)),
        Some(iso(57.93327443, 0.00282)),
    ], // FE
    [Some(iso(58.93319429, 1.0)), None, None, None], // CO
    [
        Some(iso(57.93534241, 0.68077)),
        Some(iso(59.93078588, 0.26223)),
        Some(iso(61.92834537, 0.036346)),
        Some(iso(60.93105557, 0.011399)),
    ], // NI
    [
        Some(iso(62.92959772, 0.6915)),
        Some(iso(64.9277897, 0.3085)),
        None,
        None,
    ], // CU
    [
        Some(iso(63.92914201, 0.4917)),
        Some(iso(65.92603381, 0.2773)),
        Some(iso(67.92484455, 0.1845)),
        Some(iso(66.92712775, 0.0404)),
    ], // ZN
    [
        Some(iso(68.9255735, 0.60108)),
        Some(iso(70.92470258, 0.39892)),
        None,
        None,
    ], // GA
    [
        Some(iso(73.921177761, 0.365)),
        Some(iso(71.922075826, 0.2745)),
        Some(iso(69.92424875, 0.2057)),
        Some(iso(72.923458956, 0.0775)),
    ], // GE
    [Some(iso(74.92159457, 1.0)), None, None, None], // AS
    [
        Some(iso(79.9165218, 0.4961)),
        Some(iso(77.91730928, 0.2377)),
        Some(iso(75.919213704, 0.0937)),
        Some(iso(81.9166995, 0.0873)),
    ], // SE
    [
        Some(iso(78.9183376, 0.5069)),
        Some(iso(80.9162897, 0.4931)),
        None,
        None,
    ], // BR
    [
        Some(iso(83.9114977282, 0.56987)),
        Some(iso(85.9106106269, 0.17279)),
        Some(iso(81.91348273, 0.11593)),
        Some(iso(82.91412716, 0.115)),
    ], // KR
    [
        Some(iso(84.9117897379, 0.7217)),
        Some(iso(86.909180531, 0.2783)),
        None,
        None,
    ], // RB
    [
        Some(iso(87.9056125, 0.8258)),
        Some(iso(85.9092606, 0.0986)),
        Some(iso(86.9088775, 0.07)),
        Some(iso(83.9134191, 0.0056)),
    ], // SR
    [Some(iso(88.9058403, 1.0)), None, None, None], // Y
    [
        Some(iso(89.9046977, 0.5145)),
        Some(iso(93.9063108, 0.1738)),
        Some(iso(91.9050347, 0.1715)),
        Some(iso(90.9056396, 0.1122)),
    ], // ZR
    [Some(iso(92.906373, 1.0)), None, None, None], // NB
    [
        Some(iso(97.90540482, 0.2439)),
        Some(iso(95.90467612, 0.1667)),
        Some(iso(94.90583877, 0.1584)),
        Some(iso(91.90680796, 0.1453)),
    ], // MO
    [None, None, None, None], // TC
    [
        Some(iso(101.9043441, 0.3155)),
        Some(iso(103.9054275, 0.1862)),
        Some(iso(100.9055769, 0.1706)),
        Some(iso(98.9059341, 0.1276)),
    ], // RU
    [Some(iso(102.905498, 1.0)), None, None, None], // RH
    [
        Some(iso(105.9034804, 0.2733)),
        Some(iso(107.9038916, 0.2646)),
        Some(iso(104.9050796, 0.2233)),
        Some(iso(109.9051722, 0.1172)),
    ], // PD
    [
        Some(iso(106.9050916, 0.51839)),
        Some(iso(108.9047553, 0.48161)),
        None,
        None,
    ], // AG
    [
        Some(iso(113.90336509, 0.2873)),
        Some(iso(111.90276287, 0.2413)),
        Some(iso(110.90418287, 0.128)),
        Some(iso(109.90300661, 0.1249)),
    ], // CD
    [
        Some(iso(114.903878776, 0.9571)),
        Some(iso(112.90406184, 0.0429)),
        None,
        None,
    ], // IN
    [
        Some(iso(119.90220163, 0.3258)),
        Some(iso(117.90160657, 0.2422)),
        Some(iso(115.9017428, 0.1454)),
        Some(iso(118.90331117, 0.0859)),
    ], // SN
    [
        Some(iso(120.903812, 0.5721)),
        Some(iso(122.9042132, 0.4279)),
        None,
        None,
    ], // SB
    [
        Some(iso(129.906222748, 0.3408)),
        Some(iso(127.90446128, 0.3174)),
        Some(iso(125.9033109, 0.1884)),
        Some(iso(124.9044299, 0.0707)),
    ], // TE
    [Some(iso(126.9044719, 1.0)), None, None, None], // I
    [
        Some(iso(131.9041550856, 0.269086)),
        Some(iso(128.9047808611, 0.264006)),
        Some(iso(130.90508406, 0.212324)),
        Some(iso(133.90539466, 0.104357)),
    ], // XE
    [Some(iso(132.905451961, 1.0)), None, None, None], // CS
    [
        Some(iso(137.905247, 0.71698)),
        Some(iso(136.90582714, 0.11232)),
        Some(iso(135.90457573, 0.07854)),
        Some(iso(134.90568838, 0.06592)),
    ], // BA
    [
        Some(iso(138.9063563, 0.9991119)),
        Some(iso(137.9071149, 0.0008881)),
        None,
        None,
    ], // LA
    [
        Some(iso(139.9054431, 0.8845)),
        Some(iso(141.9092504, 0.11114)),
        Some(iso(137.905991, 0.00251)),
        Some(iso(135.90712921, 0.00185)),
    ], // CE
    [Some(iso(140.9076576, 1.0)), None, None, None], // PR
    [
        Some(iso(141.907729, 0.27152)),
        Some(iso(143.910093, 0.23798)),
        Some(iso(145.9131226, 0.17189)),
        Some(iso(142.90982, 0.12174)),
    ], // ND
    [None, None, None, None], // PM
    [
        Some(iso(151.9197397, 0.2675)),
        Some(iso(153.9222169, 0.2275)),
        Some(iso(146.9149044, 0.1499)),
        Some(iso(148.9171921, 0.1382)),
    ], // SM
    [
        Some(iso(152.921238, 0.5219)),
        Some(iso(150.9198578, 0.4781)),
        None,
        None,
    ], // EU
    [
        Some(iso(157.9241123, 0.2484)),
        Some(iso(159.9270624, 0.2186)),
        Some(iso(155.9221312, 0.2047)),
        Some(iso(156.9239686, 0.1565)),
    ], // GD
    [Some(iso(158.9253547, 1.0)), None, None, None], // TB
    [
        Some(iso(163.9291819, 0.2826)),
        Some(iso(161.9268056, 0.25475)),
        Some(iso(162.9287383, 0.24896)),
        Some(iso(160.9269405, 0.18889)),
    ], // DY
    [Some(iso(164.9303288, 1.0)), None, None, None], // HO
    [
        Some(iso(165.9302995, 0.33503)),
        Some(iso(167.9323767, 0.26978)),
        Some(iso(166.9320546, 0.22869)),
        Some(iso(169.9354702, 0.1491)),
    ], // ER
    [Some(iso(168.9342179, 1.0)), None, None, None], // TM
    [
        Some(iso(173.9388664, 0.32026)),
        Some(iso(171.9363859, 0.2168)),
        Some(iso(172.9382151, 0.16103)),
        Some(iso(170.9363302, 0.1409)),
    ], // YB
    [
        Some(iso(174.9407752, 0.97401)),
        Some(iso(175.9426897, 0.02599)),
        None,
        None,
    ], // LU
    [
        Some(iso(179.946557, 0.3508)),
        Some(iso(177.9437058, 0.2728)),
        Some(iso(176.9432277, 0.186)),
        Some(iso(178.9458232, 0.1362)),
    ], // HF
    [
        Some(iso(180.9479958, 0.9998799)),
        Some(iso(179.9474648, 0.0001201)),
        None,
        None,
    ], // TA
    [
        Some(iso(183.95093092, 0.3064)),
        Some(iso(185.9543628, 0.2843)),
        Some(iso(181.94820394, 0.265)),
        Some(iso(182.95022275, 0.1431)),
    ], // W
    [
        Some(iso(186.9557501, 0.626)),
        Some(iso(184.9529545, 0.374)),
        None,
        None,
    ], // RE
    [
        Some(iso(191.961477, 0.4078)),
        Some(iso(189.9584437, 0.2626)),
        Some(iso(188.9581442, 0.1615)),
        Some(iso(187.9558352, 0.1324)),
    ], // OS
    [
        Some(iso(192.9629216, 0.627)),
        Some(iso(190.9605893, 0.373)),
        None,
        None,
    ], // IR
    [
        Some(iso(194.9647917, 0.3378)),
        Some(iso(193.9626809, 0.3286)),
        Some(iso(195.96495209, 0.2521)),
        Some(iso(197.9678949, 0.07356)),
    ], // PT
    [Some(iso(196.96656879, 1.0)), None, None, None], // AU
    [
        Some(iso(201.9706434, 0.2986)),
        Some(iso(199.96832659, 0.231)),
        Some(iso(198.96828064, 0.1687)),
        Some(iso(200.97030284, 0.1318)),
    ], // HG
    [
        Some(iso(204.9744278, 0.7048)),
        Some(iso(202.9723446, 0.2952)),
        None,
        None,
    ], // TL
    [
        Some(iso(207.9766525, 0.524)),
        Some(iso(205.9744657, 0.241)),
        Some(iso(206.9758973, 0.221)),
        Some(iso(203.973044, 0.014)),
    ], // PB
    [Some(iso(208.9803991, 1.0)), None, None, None], // BI
    [None, None, None, None], // PO
    [None, None, None, None], // AT
    [None, None, None, None], // RN
    [None, None, None, None], // FR
    [None, None, None, None], // RA
    [None, None, None, None], // AC
    // Thorium and uranium each used to carry a leading entry whose abundance
    // field held the element's STANDARD ATOMIC WEIGHT (232.0377 and 238.02891)
    // rather than an abundance, paired with a mass that is not the dominant
    // nuclide. Since this table is ordered by decreasing abundance, a value of
    // 232 sorted ahead of a genuine 1.0, so everything reading the first entry
    // -- `most_abundant_isotope`, and MONOISOTOPIC_MASSES through it -- got
    // Th-230 and U-233 instead of Th-232 and U-238. Natural thorium is
    // essentially all Th-232 and natural uranium 99.27% U-238.
    [Some(iso(232.0380558, 1.0)), None, None, None], // TH
    [Some(iso(231.0358842, 1.0)), None, None, None], // PA
    [
        Some(iso(238.0507884, 0.992742)),
        Some(iso(235.0439301, 0.007204)),
        Some(iso(234.0409523, 5.4e-05)),
        None,
    ], // U
    [None, None, None, None], // NP
    [None, None, None, None], // PU
    [None, None, None, None], // AM
    [None, None, None, None], // CM
    [None, None, None, None], // BK
    [None, None, None, None], // CF
    [None, None, None, None], // ES
    [None, None, None, None], // FM
    [None, None, None, None], // MD
    [None, None, None, None], // NO
    [None, None, None, None], // LR
    [None, None, None, None], // RF
    [None, None, None, None], // DB
    [None, None, None, None], // SG
    [None, None, None, None], // BH
    [None, None, None, None], // HS
    [None, None, None, None], // MT
    [None, None, None, None], // DS
    [None, None, None, None], // RG
    [None, None, None, None], // CN
    [None, None, None, None], // NH
    [None, None, None, None], // FL
    [None, None, None, None], // MC
    [None, None, None, None], // LV
    [None, None, None, None], // TS
    [None, None, None, None], // OG
];

pub trait ElementProperties {
    fn atomic_number(&self) -> Option<u8>;
    fn atomic_symbol(&self) -> Option<&str>;
    fn monoisotopic_mass(&self) -> Option<f64>;
    fn standard_atomic_weight(&self) -> Option<f64> {
        self.atomic_number()
            .and_then(|n| STANDARD_ATOMIC_WEIGHTS.get(n as usize).copied())?
    }
    /// The Pauling electronegativity of the element
    /// <https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)>
    /// The values are scaled by a factor of 100 to avoid floating point errors
    /// This means hydrogen has an electronegativity of 100
    /// The return type was chosen to be u16 as it is the smallest integer type that can hold all the values
    fn electronegativity(&self) -> Option<u16> {
        self.atomic_number()
            .and_then(|n| ELECTRONEGATIVITIES_PAULING.get(n as usize).copied())?
    }
    /// The Allen electronegativity of the element
    /// <https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)>
    /// The return type was chosen to be u16 as it is the smallest integer type that can hold all the values
    fn electronegativity_allen(&self) -> Option<u16> {
        self.atomic_number()
            .and_then(|n| ELECTRONEGATIVITIES_ALLEN.get(n as usize).copied())?
    }
    fn is_valid_element(&self) -> bool {
        self.atomic_number().is_some()
    }
    /// This function returns the typical oxidation states of the element
    /// The values are sorted in increasing order
    /// For an exhaustive list of oxidation states use the `oxidation_states_exhaustive` function
    fn oxidation_states(&self) -> Option<OxidationStateIter> {
        self.atomic_number().map(|n| {
            let oxidation_states = OXIDATION_STATES[n as usize]?;
            Some(OxidationStateIter::new(oxidation_states))
        })?
    }
    fn oxidation_states_exhaustive(&self) -> Option<OxidationStateExhaustiveIter> {
        self.atomic_number().map(|n| {
            let oxidation_states = OXIDATION_STATES_EXHAUSTIVE[n as usize]?;
            Some(OxidationStateExhaustiveIter::new(oxidation_states))
        })?
    }
    fn isotopes(&self) -> Option<IsotopeIter> {
        self.atomic_number().map(|n| {
            let isotopes = ISOTOPES[n as usize];
            IsotopeIter::new(isotopes)
        })
    }
    fn valencies(&self) -> Option<ValencyIter> {
        self.atomic_number().map(|n| {
            let valencies = VALENCIES[n as usize]?;
            Some(ValencyIter::new(valencies))
        })?
    }
    fn covalent_radius(&self) -> Option<f64> {
        self.atomic_number()
            .and_then(|n| COVALENT_RADII.get(n as usize).copied())
    }
    /// The MASS NUMBER of the element's most abundant isotope, or `None` for an
    /// element with no stable isotope.
    ///
    /// A mass number is a nucleon count, so the nuclide mass has to be rounded
    /// to it and not truncated: `as u16` cut oxygen's 15.9949 down to 15 and
    /// fluorine's 18.9984 to 18, and because the mass defect puts almost every
    /// nuclide just under its integer, that was wrong for 74 of the 84
    /// elements this can answer for. It happened to be right for hydrogen and
    /// carbon, which is presumably why it survived.
    fn most_abundant_isotope(&self) -> Option<u16> {
        self.isotopes()
            .and_then(|mut isotopes| isotopes.next())
            .map(|isotope| isotope.mass.round() as u16)
    }
}

impl ElementProperties for &str {
    fn atomic_number(&self) -> Option<u8> {
        atomic_numbers(self)
    }
    fn atomic_symbol(&self) -> Option<&str> {
        Some(self)
    }
    fn monoisotopic_mass(&self) -> Option<f64> {
        atomic_numbers(self).and_then(|n| MONOISOTOPIC_MASSES.get(n as usize).copied())
    }
}

// Can't make this generic due to the &str impl
impl ElementProperties for u8 {
    fn atomic_number(&self) -> Option<u8> {
        if *self < 1 || *self > 118 {
            None
        } else {
            Some(*self)
        }
    }
    fn atomic_symbol(&self) -> Option<&str> {
        ATOMIC_SYMBOLS.get(*self as usize).copied()
    }
    fn monoisotopic_mass(&self) -> Option<f64> {
        MONOISOTOPIC_MASSES.get(*self as usize).copied()
    }
}

impl ElementProperties for u16 {
    fn atomic_number(&self) -> Option<u8> {
        if *self < 1 || *self > 118 {
            None
        } else {
            Some(*self as u8)
        }
    }
    fn atomic_symbol(&self) -> Option<&str> {
        ATOMIC_SYMBOLS.get(*self as usize).copied()
    }
    fn monoisotopic_mass(&self) -> Option<f64> {
        MONOISOTOPIC_MASSES.get(*self as usize).copied()
    }
}

impl ElementProperties for u32 {
    fn atomic_number(&self) -> Option<u8> {
        if *self < 1 || *self > 118 {
            None
        } else {
            Some(*self as u8)
        }
    }
    fn atomic_symbol(&self) -> Option<&str> {
        ATOMIC_SYMBOLS.get(*self as usize).copied()
    }
    fn monoisotopic_mass(&self) -> Option<f64> {
        MONOISOTOPIC_MASSES.get(*self as usize).copied()
    }
}

impl ElementProperties for u64 {
    fn atomic_number(&self) -> Option<u8> {
        if *self < 1 || *self > 118 {
            None
        } else {
            Some(*self as u8)
        }
    }
    fn atomic_symbol(&self) -> Option<&str> {
        ATOMIC_SYMBOLS.get(*self as usize).copied()
    }
    fn monoisotopic_mass(&self) -> Option<f64> {
        MONOISOTOPIC_MASSES.get(*self as usize).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hybridization {
    Sp3,
    Sp2,
    Sp,
    /// Specific to Sulfoxides
    So,
    /// Specific to Sulfones
    So2,
    Unspecified,
}

/// Returns the Gasteiger (a, b, c) parameters based on atomic number (u8) and Hybridization.
/// Data sourced directly from RDKit's GasteigerParams.cpp see: <https://github.com/rdkit/rdkit/blob/872b054d5caa868da0ab7e8a616760ca306e95ff/Code/GraphMol/PartialCharges/GasteigerParams.cpp>
pub fn get_gasteiger_params(atomic_number: u8, mode: Hybridization) -> Option<(f64, f64, f64)> {
    match (atomic_number, mode) {
        // H (Hydrogen)
        (1, Hybridization::Unspecified) => Some((7.17, 6.24, -0.56)),

        // Be (Beryllium)
        (4, Hybridization::Sp3) => Some((3.845, 6.755, 3.165)),
        (4, Hybridization::Sp2) => Some((4.005, 6.725, 3.035)),

        // B (Boron)
        (5, Hybridization::Sp3) => Some((5.980, 6.820, 1.605)),
        (5, Hybridization::Sp2) => Some((6.420, 6.807, 1.322)),

        // C (Carbon)
        (6, Hybridization::Sp3) => Some((7.98, 9.18, 1.88)),
        (6, Hybridization::Sp2) => Some((8.79, 9.32, 1.51)),
        (6, Hybridization::Sp) => Some((10.39, 9.45, 0.73)),

        // N (Nitrogen)
        (7, Hybridization::Sp3) => Some((11.54, 10.82, 1.36)),
        (7, Hybridization::Sp2) => Some((12.87, 11.15, 0.85)),
        (7, Hybridization::Sp) => Some((15.68, 11.7, -0.27)),

        // O (Oxygen)
        (8, Hybridization::Sp3) => Some((14.18, 12.92, 1.39)),
        (8, Hybridization::Sp2) => Some((17.07, 13.79, 0.47)),

        // F (Fluorine)
        (9, Hybridization::Sp3) => Some((14.66, 13.85, 2.31)),

        // Mg (Magnesium)
        (12, Hybridization::Sp3) => Some((3.300, 5.587, 2.447)),
        (12, Hybridization::Sp2) => Some((3.565, 5.572, 2.197)),
        (12, Hybridization::Sp) => Some((4.040, 5.472, 1.823)),

        // Al (Aluminum)
        (13, Hybridization::Sp3) => Some((5.375, 4.953, 0.867)),
        (13, Hybridization::Sp2) => Some((5.795, 5.020, 0.695)),

        // Si (Silicon)
        (14, Hybridization::Sp3) => Some((7.300, 6.567, 0.657)),
        (14, Hybridization::Sp2) => Some((7.905, 6.748, 0.443)),
        (14, Hybridization::Sp) => Some((9.065, 7.027, -0.002)),

        // P (Phosphorus)
        (15, Hybridization::Sp3) => Some((8.90, 8.24, 0.96)),
        (15, Hybridization::Sp2) => Some((9.665, 8.530, 0.735)),

        // S (Sulfur)
        (16, Hybridization::Sp3) => Some((10.14, 9.13, 1.38)),
        (16, Hybridization::Sp2) => Some((10.88, 9.49, 1.33)),
        (16, Hybridization::So) => Some((10.14, 9.13, 1.38)),
        (16, Hybridization::So2) => Some((12.00, 10.81, 1.20)),

        // Cl (Chlorine)
        (17, Hybridization::Sp3) => Some((11.00, 9.69, 1.35)),

        // Br (Bromine)
        (35, Hybridization::Sp3) => Some((10.08, 8.47, 1.16)),

        // I (Iodine)
        (53, Hybridization::Sp3) => Some((9.9, 7.96, 0.96)),

        // X (Dummy atom, represented as atomic number 0)
        (0, Hybridization::Unspecified) => Some((0.00, 0.00, 0.00)),

        // Fallback for missing elements or unrecognized hybridization states
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Number of elements, ignoring the padding entry at index 0.
    const ELEMENTS: usize = 118;

    /// Every table is indexed by atomic number, so they all need the padding
    /// entry and they all need to be the same length.
    #[test]
    fn every_table_is_indexed_by_atomic_number() {
        assert_eq!(STANDARD_ATOMIC_WEIGHTS.len(), ELEMENTS + 1);
        assert_eq!(COVALENT_RADII.len(), ELEMENTS + 1);
        assert_eq!(VALENCIES.len(), ELEMENTS + 1);
        assert_eq!(MONOISOTOPIC_MASSES.len(), ELEMENTS + 1);
        assert_eq!(VALENCE_ELECTRONS.len(), ELEMENTS + 1);
        assert_eq!(ATOMIC_SYMBOLS.len(), ELEMENTS + 1);
        assert_eq!(ELECTRONEGATIVITIES_PAULING.len(), ELEMENTS + 1);
        assert_eq!(ELECTRONEGATIVITIES_ALLEN.len(), ELEMENTS + 1);
        assert_eq!(OXIDATION_STATES.len(), ELEMENTS + 1);
        assert_eq!(OXIDATION_STATES_EXHAUSTIVE.len(), ELEMENTS + 1);
        assert_eq!(ISOTOPES.len(), ELEMENTS + 1);
    }

    /// A symbol has to map back to the atomic number it is stored under.
    ///
    /// Exactly as handed out, with no uppercasing on the way in. The lookup
    /// used to accept only the uppercase form while `ATOMIC_SYMBOLS` returns
    /// the conventional mixed case, so the crate would not take back its own
    /// output and this test had to launder it through `to_uppercase()` to
    /// pass -- which hid the problem rather than catching it.
    #[test]
    fn symbols_and_atomic_numbers_agree() {
        for (atomic_number, symbol) in ATOMIC_SYMBOLS.iter().enumerate().skip(1) {
            assert_eq!(
                atomic_numbers(symbol),
                Some(atomic_number as u8),
                "{symbol} does not map back to {atomic_number}"
            );
        }
    }

    /// Capitalisation does not change which element a symbol names.
    #[test]
    fn symbol_lookup_ignores_case() {
        for (atomic_number, symbol) in ATOMIC_SYMBOLS.iter().enumerate().skip(1) {
            let expected = Some(atomic_number as u8);
            assert_eq!(atomic_numbers(&symbol.to_uppercase()), expected, "{symbol}");
            assert_eq!(atomic_numbers(&symbol.to_lowercase()), expected, "{symbol}");
            assert_eq!(symbol.atomic_number(), expected, "{symbol}");
        }
        // Including the mixed case nobody writes, since nothing distinguishes it.
        assert_eq!(atomic_numbers("hE"), Some(2));
        assert_eq!(atomic_numbers("fE"), Some(26));
    }

    /// Anything that is not one or two ASCII letters is not a symbol.
    ///
    /// The multi-byte cases matter: uppercasing runs over raw bytes, so a
    /// character that does not fit in one must be rejected rather than folded
    /// into something that accidentally matches.
    #[test]
    fn non_symbols_are_rejected() {
        for input in ["", "Xx", "Q", "Zzz", "H2", "  ", "\u{e9}", "\u{4e2d}", "H\u{e9}"] {
            assert_eq!(atomic_numbers(input), None, "{input:?} was taken for an element");
        }
    }

    /// ISOTOPES is ordered by decreasing abundance, which is the whole reason
    /// entry 0 can be read as "the most abundant one".
    #[test]
    fn isotopes_are_ordered_by_decreasing_abundance() {
        for atomic_number in 1..=ELEMENTS {
            let mut previous = f64::INFINITY;
            for isotope in ISOTOPES[atomic_number].iter().flatten() {
                assert!(
                    isotope.abundance <= previous,
                    "{} is not ordered by abundance",
                    ATOMIC_SYMBOLS[atomic_number]
                );
                previous = isotope.abundance;
            }
        }
    }

    /// An abundance is a fraction, and the fractions of one element cannot add
    /// up to more than the whole.
    ///
    /// They are allowed to add up to *less*: the table holds at most four
    /// isotopes and a dozen elements have more than that. What this catches is
    /// a value that is not an abundance at all, which is exactly how thorium
    /// and uranium were broken -- each carried a leading entry whose abundance
    /// field held the element's standard atomic weight, so it sorted ahead of
    /// the real most abundant nuclide and everything downstream read the wrong
    /// isotope.
    #[test]
    fn abundances_are_fractions() {
        for atomic_number in 1..=ELEMENTS {
            let mut total = 0.0;
            for isotope in ISOTOPES[atomic_number].iter().flatten() {
                assert!(
                    isotope.abundance > 0.0 && isotope.abundance <= 1.0,
                    "{} has an abundance of {}, which is not a fraction",
                    ATOMIC_SYMBOLS[atomic_number],
                    isotope.abundance
                );
                total += isotope.abundance;
            }
            assert!(
                total <= 1.0 + 1e-9,
                "{} abundances sum to {total}",
                ATOMIC_SYMBOLS[atomic_number]
            );
        }
    }

    /// A nuclide's mass is close to its nucleon count, and its nucleon count is
    /// at least the proton count.
    #[test]
    fn isotope_masses_are_near_a_whole_number_of_nucleons() {
        for atomic_number in 1..=ELEMENTS {
            for isotope in ISOTOPES[atomic_number].iter().flatten() {
                let nucleons = isotope.mass.round();
                assert!(
                    (isotope.mass - nucleons).abs() < 0.2,
                    "{} has a nuclide mass of {}, which is not near a whole number",
                    ATOMIC_SYMBOLS[atomic_number],
                    isotope.mass
                );
                assert!(
                    nucleons >= atomic_number as f64,
                    "{} has a nuclide with fewer nucleons than protons",
                    ATOMIC_SYMBOLS[atomic_number]
                );
            }
        }
    }

    /// MONOISOTOPIC_MASSES is derived from ISOTOPES, and has to stay that way.
    ///
    /// This is the invariant that was violated for years: the two were
    /// maintained separately and this one drifted onto each element's lightest
    /// isotope rather than its most abundant.
    #[test]
    fn monoisotopic_masses_come_from_the_isotope_table() {
        for atomic_number in 1..=ELEMENTS {
            if let Some(isotope) = ISOTOPES[atomic_number][0] {
                assert_eq!(
                    MONOISOTOPIC_MASSES[atomic_number], isotope.mass,
                    "{} disagrees with its isotope table entry",
                    ATOMIC_SYMBOLS[atomic_number]
                );
            } else {
                assert!(
                    MONOISOTOPIC_MASSES[atomic_number] > 0.0,
                    "{} has no mass at all",
                    ATOMIC_SYMBOLS[atomic_number]
                );
            }
        }
    }

    /// Spot checks for the elements the drift actually broke, so a regression
    /// names itself instead of showing up as a wrong molecular mass much later.
    #[test]
    fn the_most_abundant_isotope_is_not_the_lightest_one() {
        for (symbol, expected) in [
            ("HE", 4.00260325413),
            ("LI", 7.0160034366),
            ("B", 11.00930536),
            ("FE", 55.93493633),
            ("TH", 232.0380558),
            ("U", 238.0507884),
        ] {
            assert_eq!(
                symbol.monoisotopic_mass(),
                Some(expected),
                "{symbol} is back on its lightest isotope"
            );
        }
    }

    /// A mass number is a count of nucleons, so it is the rounded nuclide mass.
    ///
    /// Truncating instead gave oxygen 15 and fluorine 18: the mass defect puts
    /// almost every nuclide just below its integer.
    #[test]
    fn the_most_abundant_isotope_is_a_mass_number() {
        for (symbol, expected) in [("H", 1), ("C", 12), ("O", 16), ("F", 19), ("FE", 56)] {
            assert_eq!(symbol.most_abundant_isotope(), Some(expected), "{symbol}");
        }
        for atomic_number in 1..=ELEMENTS as u8 {
            let Some(mass_number) = atomic_number.most_abundant_isotope() else {
                continue;
            };
            assert_eq!(
                f64::from(mass_number),
                MONOISOTOPIC_MASSES[atomic_number as usize].round(),
                "{} mass number does not match its mass",
                ATOMIC_SYMBOLS[atomic_number as usize]
            );
        }
    }

    /// An element with no stable isotope cannot report a most abundant one.
    #[test]
    fn unstable_elements_have_no_most_abundant_isotope() {
        for symbol in ["TC", "PM", "PO", "RN", "PU", "OG"] {
            assert_eq!(symbol.most_abundant_isotope(), None, "{symbol}");
            assert!(symbol.monoisotopic_mass().is_some_and(|m| m > 0.0), "{symbol}");
        }
    }

    /// For an element with several isotopes the most abundant one sits near the
    /// standard atomic weight, which is the abundance-weighted average.
    ///
    /// The bound is loose on purpose. The most abundant nuclide need not be the
    /// one nearest the average: tellurium's Te-130 is 2.3 units above a weight
    /// of 127.6 because six of its eight isotopes are lighter, and neodymium
    /// sits 2.3 below for the mirror-image reason. What this still catches is a
    /// nuclide several mass units out, which is what picking the lightest
    /// isotope produced -- helium was off by one, iron by two, uranium by five.
    #[test]
    fn monoisotopic_masses_track_the_standard_atomic_weights() {
        for atomic_number in 1..=ELEMENTS {
            let Some(weight) = STANDARD_ATOMIC_WEIGHTS[atomic_number] else {
                continue;
            };
            if ISOTOPES[atomic_number][0].is_none() {
                continue; // handled exactly by the next test
            }
            let mass = MONOISOTOPIC_MASSES[atomic_number];
            assert!(
                (mass - weight).abs() < 3.0,
                "{}: most abundant isotope {mass} is far from its atomic weight {weight}",
                ATOMIC_SYMBOLS[atomic_number]
            );
        }
    }

    /// An element with no stable isotope has no average to be near, so the two
    /// tables have to name the same nuclide outright.
    ///
    /// Radon disagreed by eleven mass units, naming Rn-211 here and Rn-222
    /// there, and radium, technetium, neptunium and plutonium disagreed too.
    #[test]
    fn unstable_elements_name_one_nuclide_in_both_tables() {
        for atomic_number in 1..=ELEMENTS {
            if ISOTOPES[atomic_number][0].is_some() {
                continue;
            }
            let Some(weight) = STANDARD_ATOMIC_WEIGHTS[atomic_number] else {
                continue; // element 95 and up: no weight to check against
            };
            assert_eq!(
                MONOISOTOPIC_MASSES[atomic_number].round(),
                weight.round(),
                "{}: nuclide {} here, {weight} in the atomic weights",
                ATOMIC_SYMBOLS[atomic_number],
                MONOISOTOPIC_MASSES[atomic_number]
            );
        }
    }
}
