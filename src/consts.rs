use phf::phf_map;

// Physical constants TODO: Add sources
//https://www.physics.nist.gov/cgi-bin/cuu/Value?eshbar|search_for=elecmag_in!
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19; // C (Electric charge of a single electron)
// Physicochemical constants
//https://www.physics.nist.gov/cgi-bin/cuu/Value?na|search_for=physchem_in!
pub const AVOGADRO: f64 = 6.022_140_76e23; // mol^-1
pub const BOLTZMANN: f64 = 1.380_649e-23; // J K^-1
pub const GAS_CONSTANT: f64 = 8.31446261815324; // J K^-1 mol^-1 (Molar gas constant)
pub const MOLAR_VOLUME: f64 = 22.413_969_54e-3; // m^3 mol^-1 (Molar volume of an ideal gas at 0°C and 1 atm)
pub const MOLAR_MASS: f64 = 0.999_999_999_65; // kg mol^-1 (Molar mass of a substance)
pub const MOLAR_PLANCK: f64 = 3.990_312_72e-10; // J s mol^-1 (Molar Planck constant)
pub const AMU: f64 = 1.660_539_066_60e-27; // kg

// Atomic constants
// https://www.physics.nist.gov/cgi-bin/cuu/Value?bohrrada0|search_for=atomnuc!
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
// https://physics.nist.gov/cgi-bin/cuu/Value?c|search_for=universal_in!
pub const PLANCK: f64 = 6.62607015e-34; // J s
pub const HBAR: f64 = 1.0545718e-34; // J s reduced Planck constant
pub const PLANCK_LENGTH: f64 = 1.616255e-35; // m
pub const PLANCK_MASS: f64 = 2.176434e-8; // kg
pub const PLANCK_TIME: f64 = 5.391247e-44; // s
pub const SPEED_OF_LIGHT: f64 = 299792458.0; // m s^-1
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12; // F m^-1
pub const VACUUM_PERMEABILITY: f64 = 1.25663706212e-6; // N A^-2
pub const NEWTON: f64 = 6.674_30E-11; // m^3 kg^-1 s^-2
// Earth's gravity
pub const STANDARD_GRAVITY: f64 = 9.80665; // m s^-2
// https://physics.nist.gov/cgi-bin/cuu/Value?bg|search_for=universal_in!
pub const ELECTRON_VOLT: f64 = 1.602_176_634e-19; // J
pub const FARADAY: f64 = 96485.33212; // C mol^-1
pub const COULOMB: f64 = 8.987551787368176e9; // N m^2 C^-2
pub const CALORIE: f64 = 4.184; // J
pub const ATMOSPHERE: f64 = 101325.0; // Pa 
// The padding is necessary to make the array index match the atomic number 
// The values are taken from: https://physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&isotype=some
pub const STANDARD_ATOMIC_WEIGHTS: [Option<f64>; 118] = [
None,//Dummy value
Some(1.007975),//H
Some(4.002602),//He
Some(6.967499999999999),//Li
Some(9.0121831),//Be
Some(10.8135),//B
Some(12.0106),//C
Some(14.006855),//N
Some(15.9994),//O
Some(18.998403163),//F
Some(20.1797),//Ne
Some(22.98976928),//Na
Some(24.3055),//Mg
Some(26.9815385),//Al
Some(28.085),//Si
Some(30.973761998),//P
Some(32.067499999999995),//S
Some(35.451499999999996),//Cl Some(39.948),//Ar
Some(39.0983),//K
Some(40.078),//Ca
Some(44.955908),//Sc
Some(47.867),//Ti
Some(50.9415),//V
Some(51.9961),//Cr
Some(54.938044),//Mn
Some(55.845),//Fe
Some(58.933194),//Co
Some(58.6934),//Ni
Some(63.546),//Cu
Some(65.38),//Zn
Some(69.723),//Ga
Some(72.63),//Ge
Some(74.921595),//As
Some(78.971),//Se
Some(79.904),//Br
Some(83.798),//Kr
Some(85.4678),//Rb
Some(87.62),//Sr
Some(88.90584),//Y
Some(91.224),//Zr
Some(92.90637),//Nb
Some(95.95),//Mo
Some(98.0),//Tc
Some(101.07),//Ru
Some(102.9055),//Rh
Some(106.42),//Pd
Some(107.8682),//Ag
Some(112.414),//Cd
Some(114.818),//In
Some(118.71),//Sn
Some(121.76),//Sb
Some(127.6),//Te
Some(126.90447),//I
Some(131.293),//Xe
Some(132.90545196),//Cs
Some(137.327),//Ba
Some(138.90547),//La
Some(140.116),//Ce
Some(140.90766),//Pr
Some(144.242),//Nd
Some(145.0),//Pm
Some(150.36),//Sm
Some(151.964),//Eu
Some(157.25),//Gd
Some(158.92535),//Tb
Some(162.5),//Dy
Some(164.93033),//Ho
Some(167.259),//Er
Some(168.93422),//Tm
Some(173.054),//Yb
Some(174.9668),//Lu
Some(178.49),//Hf
Some(180.94788),//Ta
Some(183.84),//W
Some(186.207),//Re
Some(190.23),//Os
Some(192.217),//Ir
Some(195.084),//Pt
Some(196.966569),//Au
Some(200.592),//Hg
Some(204.3835),//Tl
Some(207.2),//Pb
Some(208.9804),//Bi
Some(209.0),//Po
Some(210.0),//At
Some(222.0),//Rn
Some(223.0),//Fr
Some(226.0),//Ra
Some(227.0),//Ac
Some(232.0377),//Th
Some(231.03588),//Pa
Some(238.02891),//U
Some(237.0),//Np
Some(244.0),//Pu
None,//Am
None,//Cm
None,//Bk
None,//Cf
None,//Es
None,//Fm
None,//Md
None,//No
None,//Lr
None,//Rf
None,//Db
None,//Sg
None,//Bh
None,//Hs
None,//Mt
None,//Ds
None,//Rg
None,//Cn
None,//Nh
None,//Fl
None,//Mc
None,//Lv
None,//Ts
None,//Og
];


// Only single bonds are considered, and the values are taken from: https://chemistry-europe.onlinelibrary.wiley.com/doi/10.1002/chem.200800987
pub static ATOMIC_RADII: [f64; 55] = [
    0.0, 0.32, 0.46, 1.33, 1.02, 0.85, 0.75, 0.75, 0.73, 0.72, 0.58, 1.60, 1.39, 1.26, 1.16, 1.11,
    1.03, 0.99, 0.97, 2.03, 1.74, 1.44, 1.32, 1.22, 1.18, 1.17, 1.17, 1.16, 1.15, 1.17, 1.25, 1.26,
    1.22, 1.21, 1.16, 1.14, 1.12, 2.16, 1.91, 1.62, 1.45, 1.34, 1.29, 1.27, 1.25, 1.25, 1.20, 1.39,
    1.44, 1.42, 1.39, 1.39, 1.38, 1.39, 1.40, // XE
];

// This is a heuristic based on experience, and is not (officially) based on any scientific data yet
// We will add an initial check soon that will try to find the initial valence of each atom
pub const VALENCIES: [i8; 33] = [
    0, // Dummy value
    1, // H
    0, // HE
    1, // LI
    2, // ...
    3, 
    4, 
    3, 
    2, 
    1, 
    0, 
    1, 
    2, 
    3, 
    4, 
    3, 
    2, 
    1, 
    0, 
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 3, 4,
];

// Taken from: https://www.nist.gov/pml/atomic-weights-and-isotopic-compositions-relative-atomic-masses
// The first element is not used, but is included for completeness
pub const MONOISOTOPIC_MASSES: [f64; 119] = [
    0.00000000000,
    1.00782503223,
    3.0160293201,
    6.0151228874,
    9.012183065,
    10.01293695,
    12.0000000,
    14.00307400443,
    15.99491461957,
    18.99840316273,
    19.9924401762,
    22.989769282,
    23.985041697,
    26.98153853,
    27.97692653465,
    30.97376199842,
    31.9720711744,
    34.968852682,
    35.967545105,
    38.9637064864,
    39.962590863,
    44.95590828,
    45.95262772,
    49.94715601,
    49.94604183,
    54.93804391,
    53.93960899,
    58.93319429,
    57.93534241,
    62.92959772,
    63.92914201,
    68.9255735,
    69.92424875,
    74.92159457,
    73.922475934,
    78.9183376,
    77.92036494,
    84.9117897379,
    83.9134191,
    88.9058403,
    89.9046977,
    92.906373,
    91.90680796,
    96.9063667,
    95.90759025,
    102.905498,
    101.9056022,
    106.9050916,
    105.9064599,
    112.90406184,
    111.90482387,
    120.903812,
    119.9040593,
    126.9044719,
    123.905892,
    132.905451961,
    129.9063207,
    137.9071149,
    135.90712921,
    140.9076576,
    141.907729,
    144.9127559,
    143.9120065,
    150.9198578,
    151.9197995,
    158.9253547,
    155.9242847,
    164.9303288,
    161.9287884,
    168.9342179,
    167.9338896,
    174.9407752,
    173.9400461,
    179.9474648,
    179.9467108,
    184.9529545,
    183.9524885,
    190.9605893,
    189.9599297,
    196.96656879,
    195.9658326,
    202.9723446,
    203.973044,
    208.9803991,
    208.9824308,
    209.9871479,
    210.9906011,
    223.019736,
    223.0185023,
    227.0277523,
    230.0331341,
    231.0358842,
    233.0396355,
    236.04657,
    238.0495601,
    241.0568293,
    243.0613893,
    247.0703073,
    249.0748539,
    252.08298,
    257.0951061,
    258.0984315,
    259.10103,
    262.10961,
    267.12179,
    268.12567,
    271.13393,
    272.13826,
    270.13429,
    276.15159,
    281.16451,
    280.16514,
    285.17712,
    284.17873,
    289.19042,
    288.19274,
    293.20449,
    292.20746,
    294.21392,
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

pub const ATOMIC_NUMBERS: phf::Map<&'static str, u8> = phf_map! {
"H" => 1,
"HE" => 2,
"LI" => 3,
"BE" => 4,
"B" => 5,
"C" => 6,
"N" => 7,
"O" => 8,
"F" => 9,
"NE" => 10,
"TI" => 22,
"V" => 23,
"CR" => 24,
"MN" => 25,
"FE" => 26,
"NI" => 27,
"CO" => 28,
"CU" => 29,
"ZN" => 30,
"GA" => 31,
"GE" => 32,
"AS" => 33,
"SE" => 34,
"BR" => 35,
"KR" => 36,
"RB" => 37,
"SR" => 38,
"Y" => 39,
"ZR" => 40,
"NB" => 41,
"MO" => 42,
"TC" => 43,
"RU" => 44,
"RH" => 45,
"PD" => 46,
"AG" => 47,
"CD" => 48,
"IN" => 49,
"SN" => 50,
"SB" => 51,
"TE" => 52,
"I" => 53,
"XE" => 54,
"CS" => 55,
"BA" => 56,
"LA" => 57,
"CE" => 58,
"PR" => 59,
"ND" => 60,
"PM" => 61,
"SM" => 62,
"EU" => 63,
"GD" => 64,
"TB" => 65,
"DY" => 66,
"HO" => 67,
"ER" => 68,
"TM" => 69,
"YB" => 70,
"LU" => 71,
"HF" => 72,
"TA" => 73,
"W" => 74,
"RE" => 75,
"OS" => 76,
"IR" => 77,
"PT" => 78,
"AU" => 79,
"HG" => 80,
"TL" => 81,
"PB" => 82,
"BI" => 83,
"TH" => 90,
"PA" => 91,
"U" => 92,
"NP" => 93,
"PU" => 94,
"AM" => 95,
"CM" => 96,
"BK" => 97,
"CF" => 98,
"ES" => 99,
"FM" => 100,
"MD" => 101,
"NO" => 102,
"LR" => 103,
"RF" => 104,
"DB" => 105,
"SG" => 106,
"BH" => 107,
"HS" => 108,
"MT" => 109,
"DS" => 110,
"RG" => 111,
"CN" => 112,
"NH" => 113,
"FL" => 114,
"MC" => 115,
"LV" => 116,
"TS" => 117,
"OG" => 118,
};

// Electronegativities based on the Pauling scale, scaled by a factor of 100 to avoid floating point errors
pub const ELECTRONEGATIVITIES: [u32; 103] = [
    0, 220, 0, 98, 157, 204, 255, 304, 350, 398, 0, 93, 131, 161, 190, 219, 258, 316, 0, 82, 100,
    136, 154, 163, 166, 155, 183, 188, 191, 190, 165, 181, 201, 218, 255, 296, 300, 82, 95, 122,
    133, 160, 216, 190, 220, 228, 220, 193, 169, 178, 196, 205, 210, 266, 260, 79, 89, 110, 112,
    113, 114, 0, 117, 0, 120, 0, 122, 123, 124, 125, 0, 127, 130, 150, 236, 190, 220, 220, 228,
    254, 200, 162, 233, 202, 200, 220, 0, 0, 90, 110, 130, 150, 138, 136, 128, 130, 130, 130, 130,
    130, 130, 130, 130,
];

pub struct Isotope {
    pub mass: f64,
    pub abundance: f64,
}

const fn iso(mass: f64, abundance: f64) -> Isotope {
    Isotope { mass, abundance }
}
// Taken from: https://physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&all=all&ascii=ascii&isotype=some
// Zero padding is used to make the array index match the atomic number
pub const ISOTOPES: [[Option<Isotope>; 4]; 119] = [ 
[None, None, None, None], // Padding
    [Some(iso(1.00782503223,0.999885)), Some(iso(2.01410177812,0.000115)), None, None], // H
    [Some(iso(4.00260325413,0.99999866)), Some(Isotope { mass: 3.0160293201, abundance: 1.34e-06 }), None, None],  // HE
    [Some(iso(7.0160034366,0.9241)), Some(iso(6.0151228874,0.0759)), None, None],  // LI
    [Some(iso(9.012183065,1.0)), None, None, None],  // BE
    [Some(iso(11.00930536,0.801)), Some(iso(10.01293695,0.199)), None, None],  // B
    [Some(iso(12.0,0.9893)), Some(iso(13.00335483507,0.0107)), None, None], // C
    [Some(iso(14.00307400443,0.99636)), Some(iso(15.00010889888,0.00364)), None, None], // N
    [Some(iso(15.99491461957,0.99757)), Some(iso(17.99915961286,0.00205)), Some(iso(16.9991317565,0.00038)), None], // O
    [Some(iso(18.99840316273,1.0)), None, None, None], // F
    [Some(iso(19.9924401762,0.9048)), Some(iso(21.991385114,0.0925)), Some(iso(20.993846685,0.0027)), None], // NE
    [Some(iso(22.989769282,1.0)), None, None, None], // NA
    [Some(iso(23.985041697,0.7899)), Some(iso(25.982592968,0.1101)), Some(iso(24.985836976,0.1)), None], // MG
    [Some(iso(26.98153853,1.0)), None, None, None], // AL
    [Some(iso(27.97692653465,0.92223)), Some(iso(28.9764946649,0.04685)), Some(iso(29.973770136,0.03092)), None], // SI
    [Some(iso(30.97376199842,1.0)), None, None, None], // P
    [Some(iso(31.9720711744,0.9499)), Some(iso(33.967867004,0.0425)), Some(iso(32.9714589098,0.0075)), Some(iso(35.96708071,0.0001))], // S
    [Some(iso(34.968852682,0.7576)), Some(iso(36.965902602,0.2424)), None, None], // CL
    [Some(Isotope {mass: 39.9623831237, abundance: 0.996035 }), Some(iso(35.967545105,0.003336)), Some(iso(37.96273211,0.000629)), None], // AR
    [Some(iso(38.9637064864,0.932581)), Some(iso(40.9618252579,0.067302)), Some(iso(39.963998166,0.000117)), None], // K
    [Some(iso(39.962590863,0.96941)), Some(iso(43.95548156,0.02086)), Some(iso(41.95861783,0.00647)), Some(iso(47.95252276,0.00187))], // CA
    [Some(iso(44.95590828,1.0)), None, None, None], // SC
    [Some(iso(47.94794198,0.7372)), Some(iso(45.95262772,0.0825)), Some(iso(46.95175879,0.0744)), Some(iso(48.94786568,0.0541))], // TI
    [Some(iso(50.94395704,0.9975)), Some(iso(49.94715601,0.0025)), None, None], // V
    [Some(iso(51.94050623,0.83789)), Some(iso(52.94064815,0.09501)), Some(iso(49.94604183,0.04345)), Some(iso(53.93887916,0.02365))], // CR
    [Some(iso(54.93804391,1.0)), None, None, None], // MN
    [Some(iso(55.93493633,0.91754)), Some(iso(53.93960899,0.05845)), Some(iso(56.93539284,0.02119)), Some(iso(57.93327443,0.00282))], // FE
    [Some(iso(58.93319429,1.0)), None, None, None], // CO
    [Some(iso(57.93534241,0.68077)), Some(iso(59.93078588,0.26223)), Some(iso(61.92834537,0.036346)), Some(iso(60.93105557,0.011399))], // NI
    [Some(iso(62.92959772,0.6915)), Some(iso(64.9277897,0.3085)), None, None], // CU
    [Some(iso(63.92914201,0.4917)), Some(iso(65.92603381,0.2773)), Some(iso(67.92484455,0.1845)), Some(iso(66.92712775,0.0404))], // ZN
    [Some(iso(68.9255735,0.60108)), Some(iso(70.92470258,0.39892)), None, None], // GA
    [Some(iso(73.921177761,0.365)), Some(iso(71.922075826,0.2745)), Some(iso(69.92424875,0.2057)), Some(iso(72.923458956,0.0775))], // GE
    [Some(iso(74.92159457,1.0)), None, None, None], // AS
    [Some(iso(79.9165218,0.4961)), Some(iso(77.91730928,0.2377)), Some(iso(75.919213704,0.0937)), Some(iso(81.9166995,0.0873))], // SE
    [Some(iso(78.9183376,0.5069)), Some(iso(80.9162897,0.4931)), None, None], // BR
    [Some(iso(83.9114977282,0.56987)), Some(iso(85.9106106269,0.17279)), Some(iso(81.91348273,0.11593)), Some(iso(82.91412716,0.115))], // KR
    [Some(iso(84.9117897379,0.7217)), Some(iso(86.909180531,0.2783)), None, None], // RB
    [Some(iso(87.9056125,0.8258)), Some(iso(85.9092606,0.0986)), Some(iso(86.9088775,0.07)), Some(iso(83.9134191,0.0056))], // SR
    [Some(iso(88.9058403,1.0)), None, None, None], // Y
    [Some(iso(89.9046977,0.5145)), Some(iso(93.9063108,0.1738)), Some(iso(91.9050347,0.1715)), Some(iso(90.9056396,0.1122))], // ZR
    [Some(iso(92.906373,1.0)), None, None, None], // NB
    [Some(iso(97.90540482,0.2439)), Some(iso(95.90467612,0.1667)), Some(iso(94.90583877,0.1584)), Some(iso(91.90680796,0.1453))], // MO
    [None, None, None, None], // TC
    [Some(iso(101.9043441,0.3155)), Some(iso(103.9054275,0.1862)), Some(iso(100.9055769,0.1706)), Some(iso(98.9059341,0.1276))], // RU
    [Some(iso(102.905498,1.0)), None, None, None], // RH
    [Some(iso(105.9034804,0.2733)), Some(iso(107.9038916,0.2646)), Some(iso(104.9050796,0.2233)), Some(iso(109.9051722,0.1172))], // PD
    [Some(iso(106.9050916,0.51839)), Some(iso(108.9047553,0.48161)), None, None], // AG
    [Some(iso(113.90336509,0.2873)), Some(iso(111.90276287,0.2413)), Some(iso(110.90418287,0.128)), Some(iso(109.90300661,0.1249))], // CD
    [Some(iso(114.903878776,0.9571)), Some(iso(112.90406184,0.0429)), None, None], // IN
    [Some(iso(119.90220163,0.3258)), Some(iso(117.90160657,0.2422)), Some(iso(115.9017428,0.1454)), Some(iso(118.90331117,0.0859))], // SN
    [Some(iso(120.903812,0.5721)), Some(iso(122.9042132,0.4279)), None, None], // SB
    [Some(iso(129.906222748,0.3408)), Some(iso(127.90446128,0.3174)), Some(iso(125.9033109,0.1884)), Some(iso(124.9044299,0.0707))], // TE
    [Some(iso(126.9044719,1.0)), None, None, None], // I
    [Some(iso(131.9041550856,0.269086)), Some(iso(128.9047808611,0.264006)), Some(iso(130.90508406,0.212324)), Some(iso(133.90539466,0.104357))], // XE
    [Some(iso(132.905451961,1.0)), None, None, None], // CS
    [Some(iso(137.905247,0.71698)), Some(iso(136.90582714,0.11232)), Some(iso(135.90457573,0.07854)), Some(iso(134.90568838,0.06592))], // BA
    [Some(iso(138.9063563,0.9991119)), Some(iso(137.9071149,0.0008881)), None, None], // LA
    [Some(iso(139.9054431,0.8845)), Some(iso(141.9092504,0.11114)), Some(iso(137.905991,0.00251)), Some(iso(135.90712921,0.00185))], // CE
    [Some(iso(140.9076576,1.0)), None, None, None], // PR
    [Some(iso(141.907729,0.27152)), Some(iso(143.910093,0.23798)), Some(iso(145.9131226,0.17189)), Some(iso(142.90982,0.12174))], // ND
    [None, None, None, None], // PM
    [Some(iso(151.9197397,0.2675)), Some(iso(153.9222169,0.2275)), Some(iso(146.9149044,0.1499)), Some(iso(148.9171921,0.1382))], // SM
    [Some(iso(152.921238,0.5219)), Some(iso(150.9198578,0.4781)), None, None], // EU
    [Some(iso(157.9241123,0.2484)), Some(iso(159.9270624,0.2186)), Some(iso(155.9221312,0.2047)), Some(iso(156.9239686,0.1565))], // GD
    [Some(iso(158.9253547,1.0)), None, None, None], // TB
    [Some(iso(163.9291819,0.2826)), Some(iso(161.9268056,0.25475)), Some(iso(162.9287383,0.24896)), Some(iso(160.9269405,0.18889))], // DY
    [Some(iso(164.9303288,1.0)), None, None, None], // HO
    [Some(iso(165.9302995,0.33503)), Some(iso(167.9323767,0.26978)), Some(iso(166.9320546,0.22869)), Some(iso(169.9354702,0.1491))], // ER
    [Some(iso(168.9342179,1.0)), None, None, None], // TM
    [Some(iso(173.9388664,0.32026)), Some(iso(171.9363859,0.2168)), Some(iso(172.9382151,0.16103)), Some(iso(170.9363302,0.1409))], // YB
    [Some(iso(174.9407752,0.97401)), Some(iso(175.9426897,0.02599)), None, None], // LU
    [Some(iso(179.946557,0.3508)), Some(iso(177.9437058,0.2728)), Some(iso(176.9432277,0.186)), Some(iso(178.9458232,0.1362))], // HF
    [Some(iso(180.9479958,0.9998799)), Some(iso(179.9474648,0.0001201)), None, None], // TA
    [Some(iso(183.95093092,0.3064)), Some(iso(185.9543628,0.2843)), Some(iso(181.94820394,0.265)), Some(iso(182.95022275,0.1431))], // W
    [Some(iso(186.9557501,0.626)), Some(iso(184.9529545,0.374)), None, None], // RE
    [Some(iso(191.961477,0.4078)), Some(iso(189.9584437,0.2626)), Some(iso(188.9581442,0.1615)), Some(iso(187.9558352,0.1324))], // OS
    [Some(iso(192.9629216,0.627)), Some(iso(190.9605893,0.373)), None, None], // IR
    [Some(iso(194.9647917,0.3378)), Some(iso(193.9626809,0.3286)), Some(iso(195.96495209,0.2521)), Some(iso(197.9678949,0.07356))], // PT
    [Some(iso(196.96656879,1.0)), None, None, None], // AU
    [Some(iso(201.9706434,0.2986)), Some(iso(199.96832659,0.231)), Some(iso(198.96828064,0.1687)), Some(iso(200.97030284,0.1318))], // HG
    [Some(iso(204.9744278,0.7048)), Some(iso(202.9723446,0.2952)), None, None], // TL
    [Some(iso(207.9766525,0.524)), Some(iso(205.9744657,0.241)), Some(iso(206.9758973,0.221)), Some(iso(203.973044,0.014))], // PB
    [Some(iso(208.9803991,1.0)), None, None, None], // BI
    [None, None, None, None], // PO
    [None, None, None, None], // AT
    [None, None, None, None], // RN
    [None, None, None, None], // FR
    [None, None, None, None], // RA
    [None, None, None, None], // AC
    [Some(iso(230.0331341,232.0377)), Some(iso(232.0380558,1.0)), None, None], // TH
    [Some(iso(231.0358842,1.0)), None, None, None], // PA
    [Some(iso(233.0396355,238.02891)), Some(iso(238.0507884,0.992742)), Some(iso(235.0439301,0.007204)), Some(iso(234.0409523,5.4e-05))], // U
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
