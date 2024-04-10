// Physical constants TODO: Add sources
// https://www.physics.nist.gov/cgi-bin/cuu/Value?eshbar|search_for=elecmag_in!
// Physicochemical constants
// https://www.physics.nist.gov/cgi-bin/cuu/Value?na|search_for=physchem_in!
pub const AVOGADRO: f64 = 6.022_140_76e23; // mol^-1
pub const BOLTZMANN: f64 = 1.380_649e-23; // J K^-1 pub const GAS_CONSTANT: f64 = 8.31446261815324; // J K^-1 mol^-1 (Molar gas constant) pub const MOLAR_VOLUME: f64 = 22.413_969_9e-3; // m^3 mol^-1 (Molar volume of an ideal gas at 0°C and 1 atm)
pub const MOLAR_MASS: f64 = 0.999_999_999_65; // kg mol^-1 (Molar mass of a substance)
pub const MOLAR_PLANCK: f64 = 3.990_312_71e-10; // J s mol^-1 (Molar Planck constant)
pub const AMU: f64 = 1.660_539_066_60e-27; // kg

// Atomic constants
// https:// www.physics.nist.gov/cgi-bin/cuu/Value?bohrrada0|search_for=atomnuc!
pub const HARTREE: f64 = 4.359_744_722_207_1; // eV
pub const HARTREE_EV: f64 = 27.211_386_245_988; // eV
pub const BOHR_MAGNETON: f64 = 9.274_009_994_57e-24; // J T^-1
pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11; // m
pub const NEUTRON_MASS: f64 = 1.674_927_498_04e-27; // kg
pub const PROTON_MASS: f64 = 1.672_621_923_69e-27; // kg
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31; // kg
pub const FINE_STRUCTURE: f64 = 7.297_352_569_3e-3; // Dimensionless
pub const RYDBERG: f64 = 10_973_731.568_160; // m^-1
                                             // Fundamental physical constants
                                             // https:// physics.nist.gov/cgi-bin/cuu/Value?c|search_for=universal_in!
pub const PLANCK: f64 = 6.626_070_15e-34; // J s
pub const HBAR: f64 = 1.0545718e-34; // J s reduced Planck constant
pub const PLANCK_LENGTH: f64 = 1.616255e-35; // m
pub const PLANCK_MASS: f64 = 2.176434e-8; // kg
pub const PLANCK_TIME: f64 = 5.391247e-44; // s
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m s^-1
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12; // F m^-1
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6; // N A^-2
pub const NEWTON: f64 = 6.674_30E-11; // m^3 kg^-1 s^-2
                                      // Earth's gravity
pub const STANDARD_GRAVITY: f64 = 9.806_65; // m s^-2
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19; // C (Electric charge of a single electron)
pub const ELECTRON_VOLT: f64 = 1.602_176_634e-19; // J
pub const FARADAY: f64 = 96_485.332_12; // C mol^-1
pub const COULOMB: f64 = 8.987_551_787_368_176e9; // N m^2 C^-2
pub const CALORIE: f64 = 4.184; // J
pub const ATMOSPHERE: f64 = 101_325.0; // Pa

// The values are taken from: https:// physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&isotype=some
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
// The values up to CM are taken from:https:// doi.org/10.1002/chem.200800987
// The values from CF to OG are taken from: https:// en.wikipedia.org/wiki/Atomic_radii_of_the_elements_(data_page)
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

// The following values were taken from rdkit and are subject to change (Source: https://raw.githubusercontent.com/rdkit/rdkit/9e1dbd2f336b8c381d4e7ef8d8ce8b9641ec3bc8/Code/GraphMol/atomic_data.cpp)
pub const VALENCIES: [Option<[Option<i8>;4]>; 119] = [
None, //Padding

Some([Some(1), None, None, None]), //H

Some([Some(0), None, None, None]), //HE

Some([Some(1), Some(-1), None, None]), //LI

Some([Some(2), None, None, None]), //BE

Some([Some(3), None, None, None]), //B

Some([Some(4), None, None, None]), //C

Some([Some(3), None, None, None]), //N

Some([Some(2), None, None, None]), //O

Some([Some(1), None, None, None]), //F

Some([Some(0), None, None, None]), //NE

Some([Some(1), Some(-1), None, None]), //NA

Some([Some(2), Some(-1), None, None]), //MG

Some([Some(3), Some(6), None, None]), //AL

Some([Some(4), Some(6), None, None]), //SI

Some([Some(3), Some(5), Some(7), None]), //P

Some([Some(2), Some(4), Some(6), None]), //S

Some([Some(1), None, None, None]), //CL

Some([Some(0), None, None, None]), //AR

Some([Some(1), Some(-1), None, None]), //K

Some([Some(2), Some(-1), None, None]), //CA

None, //SC

None, //TI

None, //V

None, //CR

None, //MN

None, //FE

None, //CO

None, //NI

None, //CU

None, //ZN

Some([Some(3), None, None, None]), //GA

Some([Some(4), None, None, None]), //GE

Some([Some(3), Some(5), Some(7), None]), //AS

Some([Some(2), Some(4), Some(6), None]), //SE

Some([Some(1), None, None, None]), //BR

Some([Some(0), None, None, None]), //KR

Some([Some(1), None, None, None]), //RB

Some([Some(2), Some(-1), None, None]), //SR

None, //Y

None, //ZR

None, //NB

None, //MO

None, //TC

None, //RU

None, //RH

None, //PD

None, //AG

None, //CD

Some([Some(3), None, None, None]), //IN

Some([Some(2), Some(4), None, None]), //SN

Some([Some(3), Some(5), Some(7), None]), //SB

Some([Some(2), Some(4), Some(6), None]), //TE

Some([Some(1), Some(3), Some(5), None]), //I

Some([Some(0), Some(2), Some(4), Some(6)]), //XE

Some([Some(1), None, None, None]), //CS

Some([Some(2), Some(-1), None, None]), //BA

None, //LA

None, //CE

None, //PR

None, //ND

None, //PM

None, //SM

None, //EU

None, //GD

None, //TB

None, //DY

None, //HO

None, //ER

None, //TM

None, //YB

None, //LU

None, //HF

None, //TA

None, //W

None, //RE

None, //OS

None, //IR

None, //PT

None, //AU

None, //HG

Some([Some(3), None, None, None]), //TL

Some([Some(2), Some(4), None, None]), //PB

Some([Some(3), Some(5), Some(7), None]), //BI

Some([Some(2), Some(4), Some(6), None]), //PO

Some([Some(1), Some(3), Some(5), None]), //AT

Some([Some(0), None, None, None]), //RN

Some([Some(1), None, None, None]), //FR

Some([Some(2), Some(-1), None, None]), //RA

None, //AC

None, //TH

None, //PA

None, //U

None, //NP

None, //PU

None, //AM

None, //CM

None, //BK

None, //CF

None, //ES

None, //FM

None, //MD

None, //NO

None, //LR

None, //RF

None, //DB

None, //SG

None, //BH

None, //HS

None, //MT

None, //DS

None, //RG

None, //CN

None, //NH

None, //FL

None, //MC

None, //LV

None, //TS

None, //OG
];

// This is a heuristic based on experience, and is not (officially) based on any scientific data yet, for transition metals this does not work well
pub const MOST_COMMON_VALENCIES: [i8; 24] = [
    0, // Dummy value
    1, // H
    0, // HE
    1, // LI
    2, // BE
    3, // B
    4, // C
    3, // N
    2, // O
    1, // F
    0, // NE
    1, // NA
    2, // MG
    3, // AL
    4, // SI
    3, // P
    2, // S
    1, // CL
    0, // AR
    1, // K
    2, // CA
    3, // SC
    4, // TI
    5, // V
];

// Taken from: https:// www.nist.gov/pml/atomic-weights-and-isotopic-compositions-relative-atomic-masses
// The first element is not used, but is included for completeness
pub const MONOISOTOPIC_MASSES: [f64; 119] = [
    0.00000000000,  // Dummy value
    1.00782503223,  // H
    3.0160293201,   // HE
    6.0151228874,   // LI
    9.012183065,    // BE
    10.01293695,    // B
    12.0000000,     // C
    14.00307400443, // N
    15.99491461957, // O
    18.99840316273, // F
    19.9924401762,  // NE
    22.989769282,   // NA
    23.985041697,   // MG
    26.98153853,    // AL
    27.97692653465, // SI
    30.97376199842, // P
    31.9720711744,  // S
    34.968852682,   // CL
    35.967545105,   // AR
    38.9637064864,  // K
    39.962590863,   // CA
    44.95590828,    // SC
    45.95262772,    // TI
    49.94715601,    // V
    49.94604183,    // CR
    54.93804391,    // MN
    53.93960899,    // FE
    58.93319429,    // CO
    57.93534241,    // NI
    62.92959772,    // CU
    63.92914201,    // ZN
    68.9255735,     // GA
    69.92424875,    // GE
    74.92159457,    // AS
    73.922475934,   // SE
    78.9183376,     // BR
    77.92036494,    // KR
    84.9117897379,  // RB
    83.9134191,     // SR
    88.9058403,     // Y
    89.9046977,     // ZR
    92.906373,      // NB
    91.90680796,    // MO
    96.9063667,     // TC
    95.90759025,    // RU
    102.905498,     // RH
    101.9056022,    // PD
    106.9050916,    // AG
    105.9064599,    // CD
    112.90406184,   // IN
    111.90482387,   // SN
    120.903812,     // SB
    119.9040593,    // TE
    126.9044719,    // I
    123.905892,     // XE
    132.905451961,  // CS
    129.9063207,    // BA
    137.9071149,    // LA
    135.90712921,   // CE
    140.9076576,    // PR
    141.907729,     // ND
    144.9127559,    // PM
    143.9120065,    // SM
    150.9198578,    // EU
    151.9197995,    // GD
    158.9253547,    // TB
    155.9242847,    // DY
    164.9303288,    // HO
    161.9287884,    // ER
    168.9342179,    // TM
    167.9338896,    // YB
    174.9407752,    // LU
    173.9400461,    // HF
    179.9474648,    // TA
    179.9467108,    // W
    184.9529545,    // RE
    183.9524885,    // OS
    190.9605893,    // IR
    189.9599297,    // PT
    196.96656879,   // AU
    195.9658326,    // HG
    202.9723446,    // TL
    203.973044,     // PB
    208.9803991,    // BI
    208.9824308,    // PO
    209.9871479,    // AT
    210.9906011,    // RN
    223.019736,     // FR
    223.0185023,    // RA
    227.0277523,    // AC
    230.0331341,    // TH
    231.0358842,    // PA
    233.0396355,    // U
    236.04657,      // NP
    238.0495601,    // PU
    241.0568293,    // AM
    243.0613893,    // CM
    247.0703073,    // BK
    249.0748539,    // CF
    252.08298,      // ES
    257.0951061,    // FM
    258.0984315,    // MD
    259.10103,      // NO
    262.10961,      // LR
    267.12179,      // RF
    268.12567,      // DB
    271.13393,      // SG
    272.13826,      // BH
    270.13429,      // HS
    276.15159,      // MT
    281.16451,      // DS
    280.16514,      // RG
    285.17712,      // CN
    284.17873,      // NH
    289.19042,      // FL
    288.19274,      // MC
    293.20449,      // LV
    292.20746,      // TS
    294.21392,      // OG
];

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

pub fn atomic_numbers(symbol: &str) -> Option<u8> {
    generate_match!(symbol; 
   ("H" ,1),
   ("HE", 2),
   ("LI", 3),
   ( "BE", 4),
   ( "B" ,5),
   ( "C" ,6),
   ( "N" ,7),
   ( "O" ,8),
   ( "F" ,9),
   ( "NE", 10),
   ( "TI", 22),
   ( "V" ,23),
   ( "CR", 24),
   ( "MN", 25),
   ( "FE", 26),
   ( "NI", 27),
   ( "CO", 28),
   ( "CU", 29),
   ( "ZN", 30),
   ( "GA", 31),
   ( "GE", 32),
   ( "AS", 33),
   ( "SE", 34),
   ( "BR", 35),
   ( "KR", 36),
   ( "RB", 37),
   ( "SR", 38),
   ( "Y" ,39),
   ( "ZR", 40),
   ( "NB", 41),
   ( "MO", 42),
   ( "TC", 43),
   ( "RU", 44),
   ( "RH", 45),
   ( "PD", 46),
   ( "AG", 47),
   ( "CD", 48),
   ( "IN", 49),
   ( "SN", 50),
   ( "SB", 51),
   ( "TE", 52),
   ( "I" ,53),
   ( "XE", 54),
   ( "CS", 55),
   ( "BA", 56),
   ( "LA", 57),
   ( "CE", 58),
   ( "PR", 59),
   ( "ND", 60),
   ( "PM", 61),
   ( "SM", 62),
   ( "EU", 63),
   ( "GD", 64),
   ( "TB", 65),
   ( "DY", 66),
   ( "HO", 67),
   ( "ER", 68),
   ( "TM", 69),
   ( "YB", 70),
   ( "LU", 71),
   ( "HF", 72),
   ( "TA", 73),
   ( "W" ,74),
   ( "RE", 75),
   ( "OS", 76),
   ( "IR", 77),
   ( "PT", 78),
   ( "AU", 79),
   ( "HG", 80),
   ( "TL", 81),
   ( "PB", 82),
   ( "BI", 83),
   ( "TH", 90),
   ( "PA", 91),
   ( "U" ,92),
   ( "NP", 93),
   ( "PU", 94),
   ( "AM", 95),
   ( "CM", 96),
   ( "BK", 97),
   ( "CF", 98),
   ( "ES", 99),
   ( "FM", 100),
   ( "MD", 101),
   ( "NO", 102),
   ( "LR", 103),
   ( "RF", 104),
   ( "DB", 105),
   ( "SG", 106),
   ( "BH", 107),
   ( "HS", 108),
   ( "MT", 109),
   ( "DS", 110),
   ( "RG", 111),
   ( "CN", 112),
   ( "NH", 113),
   ( "FL", 114),
   ( "MC", 115),
   ( "LV", 116),
   ( "TS", 117),
   ( "OG", 118),)
}

// Electronegativities baused on the Pauling scale, scaled by a factor of 100 to avoid floating point errors
//
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
    None,      // RN
    None,      // FR
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
    None,      // LR
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

// This list is taken from Wikipedia, and only includes the major oxidation states: https:// en.wikipedia.org/wiki/Oxidation_state#List_of_oxidation_states_of_the_elements
pub const OXIDATION_STATES: [Option<[Option<i8>; 9]>; 119] = [
    None, //Padding
    Some([Some(-1), Some(1), None, None, None, None, None, None, None]), //H
    Some([Some(0), None, None, None, None, None, None, None, None]), //HE
    Some([Some(1), None, None, None, None, None, None, None, None]), //LI
    Some([Some(2), None, None, None, None, None, None, None, None]), //BE
    Some([Some(3), None, None, None, None, None, None, None, None]), //B
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
    Some([Some(-2), None, None, None, None, None, None, None, None]), //O
    Some([Some(-1), None, None, None, None, None, None, None, None]), //F
    Some([Some(0), None, None, None, None, None, None, None, None]), //NE
    Some([Some(1), None, None, None, None, None, None, None, None]), //NA
    Some([Some(2), None, None, None, None, None, None, None, None]), //MG
    Some([Some(3), None, None, None, None, None, None, None, None]), //AL
    Some([Some(4), None, None, None, None, None, None, None, None]), //SI
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
    Some([Some(0), None, None, None, None, None, None, None, None]), //AR
    Some([Some(1), None, None, None, None, None, None, None, None]), //K
    Some([Some(2), None, None, None, None, None, None, None, None]), //CA
    Some([Some(3), None, None, None, None, None, None, None, None]), //SC
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
    Some([Some(2), Some(3), None, None, None, None, None, None, None]), //FE
    Some([Some(2), Some(3), None, None, None, None, None, None, None]), //CO
    Some([Some(2), None, None, None, None, None, None, None, None]), //NI
    Some([Some(1), Some(2), None, None, None, None, None, None, None]), //CU
    Some([Some(2), None, None, None, None, None, None, None, None]), //ZN
    Some([Some(3), None, None, None, None, None, None, None, None]), //GA
    Some([Some(2), Some(4), None, None, None, None, None, None, None]), //GE
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
    Some([Some(0), None, None, None, None, None, None, None, None]), //KR
    Some([Some(1), None, None, None, None, None, None, None, None]), //RB
    Some([Some(2), None, None, None, None, None, None, None, None]), //SR
    Some([Some(3), None, None, None, None, None, None, None, None]), //Y
    Some([Some(4), None, None, None, None, None, None, None, None]), //ZR
    Some([Some(5), None, None, None, None, None, None, None, None]), //NB
    Some([Some(4), Some(6), None, None, None, None, None, None, None]), //MO
    Some([Some(4), Some(7), None, None, None, None, None, None, None]), //TC
    Some([Some(3), Some(4), None, None, None, None, None, None, None]), //RU
    Some([Some(3), None, None, None, None, None, None, None, None]), //RH
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
    Some([Some(1), None, None, None, None, None, None, None, None]), //AG
    Some([Some(2), None, None, None, None, None, None, None, None]), //CD
    Some([Some(3), None, None, None, None, None, None, None, None]), //IN
    Some([Some(2), Some(4), None, None, None, None, None, None, None]), //SN
    Some([Some(3), Some(5), None, None, None, None, None, None, None]), //SB
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
    Some([Some(0), None, None, None, None, None, None, None, None]), //XE
    Some([Some(1), None, None, None, None, None, None, None, None]), //CS
    Some([Some(2), None, None, None, None, None, None, None, None]), //BA
    Some([Some(3), None, None, None, None, None, None, None, None]), //LA
    Some([Some(3), Some(4), None, None, None, None, None, None, None]), //CE
    Some([Some(3), None, None, None, None, None, None, None, None]), //PR
    Some([Some(3), None, None, None, None, None, None, None, None]), //ND
    Some([Some(3), None, None, None, None, None, None, None, None]), //PM
    Some([Some(3), None, None, None, None, None, None, None, None]), //SM
    Some([Some(2), Some(3), None, None, None, None, None, None, None]), //EU
    Some([Some(3), None, None, None, None, None, None, None, None]), //GD
    Some([Some(3), None, None, None, None, None, None, None, None]), //TB
    Some([Some(3), None, None, None, None, None, None, None, None]), //DY
    Some([Some(3), None, None, None, None, None, None, None, None]), //HO
    Some([Some(3), None, None, None, None, None, None, None, None]), //ER
    Some([Some(3), None, None, None, None, None, None, None, None]), //TM
    Some([Some(3), None, None, None, None, None, None, None, None]), //YB
    Some([Some(3), None, None, None, None, None, None, None, None]), //LU
    Some([Some(4), None, None, None, None, None, None, None, None]), //HF
    Some([Some(5), None, None, None, None, None, None, None, None]), //TA
    Some([Some(4), Some(6), None, None, None, None, None, None, None]), //W
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
    Some([Some(2), Some(4), None, None, None, None, None, None, None]), //PT
    Some([Some(1), Some(3), None, None, None, None, None, None, None]), //AU
    Some([Some(1), Some(2), None, None, None, None, None, None, None]), //HG
    Some([Some(1), Some(3), None, None, None, None, None, None, None]), //TL
    Some([Some(2), Some(4), None, None, None, None, None, None, None]), //PB
    Some([Some(3), None, None, None, None, None, None, None, None]), //BI
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
    Some([Some(2), None, None, None, None, None, None, None, None]), //RN
    Some([Some(1), None, None, None, None, None, None, None, None]), //FR
    Some([Some(2), None, None, None, None, None, None, None, None]), //RA
    Some([Some(3), None, None, None, None, None, None, None, None]), //AC
    Some([Some(4), None, None, None, None, None, None, None, None]), //TH
    Some([Some(5), None, None, None, None, None, None, None, None]), //PA
    Some([Some(4), Some(6), None, None, None, None, None, None, None]), //U
    Some([Some(5), None, None, None, None, None, None, None, None]), //NP
    Some([Some(4), None, None, None, None, None, None, None, None]), //PU
    Some([Some(3), None, None, None, None, None, None, None, None]), //AM
    Some([Some(3), None, None, None, None, None, None, None, None]), //CM
    Some([Some(3), None, None, None, None, None, None, None, None]), //BK
    Some([Some(3), None, None, None, None, None, None, None, None]), //CF
    Some([Some(3), None, None, None, None, None, None, None, None]), //ES
    Some([Some(3), None, None, None, None, None, None, None, None]), //FM
    Some([Some(3), None, None, None, None, None, None, None, None]), //MD
    Some([Some(2), None, None, None, None, None, None, None, None]), //NO
    Some([Some(3), None, None, None, None, None, None, None, None]), //LR
    Some([Some(4), None, None, None, None, None, None, None, None]), //RF
    Some([Some(5), None, None, None, None, None, None, None, None]), //DB
    Some([Some(6), None, None, None, None, None, None, None, None]), //SG
    Some([Some(7), None, None, None, None, None, None, None, None]), //BH
    Some([Some(8), None, None, None, None, None, None, None, None]), //HS
    None, //MT
    None, //DS
    None, //RG
    Some([Some(2), None, None, None, None, None, None, None, None]), //CN
    None, //NH
    None, //FL
    None, //MC
    None, //LV
    None, //TS
    None, //OG
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
// Taken from: https:// physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&all=all&ascii=ascii&isotype=some
// Zero padding is used to make the array index match the atomic number
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
    [
        Some(iso(230.0331341, 232.0377)),
        Some(iso(232.0380558, 1.0)),
        None,
        None,
    ], // TH
    [Some(iso(231.0358842, 1.0)), None, None, None], // PA
    [
        Some(iso(233.0396355, 238.02891)),
        Some(iso(238.0507884, 0.992742)),
        Some(iso(235.0439301, 0.007204)),
        Some(iso(234.0409523, 5.4e-05)),
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

pub trait AtomProperties {
    fn atomic_number(&self) -> Option<u8>;
    fn atomic_symbol(&self) -> Option<&str>;
    fn monoisotopic_mass(&self) -> Option<f64>;
    fn standard_atomic_weight(&self) -> Option<f64> {
        self.atomic_number()
            .and_then(|n| STANDARD_ATOMIC_WEIGHTS.get(n as usize).copied())?
    }
    /// The Pauling electronegativity of the element
    /// (https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page))
    /// The values are scaled by a factor of 100 to avoid floating point errors
    /// This means hydrogen has an electronegativity of 100
    /// The return type was chosen to be u16 as it is the smallest integer type that can hold all the values
    fn electronegativity(&self) -> Option<u16> {
        self.atomic_number()
            .and_then(|n| ELECTRONEGATIVITIES_PAULING.get(n as usize).copied())?
    }
    /// The Allen electronegativity of the element
    /// (https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page))
    /// The values are scaled by a factor of 100 to avoid floating point errors
    /// This means hydrogen has an electronegativity of 100
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
            let Some(oxidation_states) = OXIDATION_STATES[n as usize] else {
                return None;
            };
            Some(OxidationStateIter::new(oxidation_states))
        })?
    }
    fn oxidation_states_exhaustive(&self) -> Option<OxidationStateExhaustiveIter> {
        self.atomic_number().map(|n| {
            let Some(oxidation_states) = OXIDATION_STATES_EXHAUSTIVE[n as usize] else {
                return None;
            };
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
            let Some(valencies) = VALENCIES[n as usize] else {
                return None;
            };
            Some(ValencyIter::new(valencies))
        })?
    }
}

impl AtomProperties for &str {
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
impl AtomProperties for u8 {
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
