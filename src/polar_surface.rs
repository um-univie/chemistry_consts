//! Ertl topological polar surface area contributions in square angstroms.
//!
//! Each table documents its atom-environment key. These are reference
//! contributions; callers decide atom typing and whether to include S/P.

/// Aliphatic nitrogen environment: hydrogens, singles, doubles, triples,
/// formal charge, and membership in a three-membered ring.
pub type NitrogenEnvironment = (u8, u8, u8, u8, i8, bool);
/// Aromatic nitrogen environment: hydrogens, aromatic bonds, singles,
/// doubles, and formal charge.
pub type AromaticNitrogenEnvironment = (u8, u8, u8, u8, i8);
/// Oxygen environment: hydrogens, singles, doubles, formal charge,
/// aromaticity, and membership in a three-membered ring.
pub type OxygenEnvironment = (u8, u8, u8, i8, bool, bool);
/// Sulfur environment: hydrogens, singles, doubles, formal charge, aromaticity.
pub type SulfurEnvironment = (u8, u8, u8, i8, bool);
/// Phosphorus environment: hydrogens, singles, doubles, and formal charge.
pub type PhosphorusEnvironment = (u8, u8, u8, i8);

/// Nitrogen, aliphatic. Keyed by `(hydrogens, singles, doubles, triples,
/// charge, in three-membered ring)`.
pub const NITROGEN: &[(NitrogenEnvironment, f64)] = &[
    ((0, 3, 0, 0, 0, false), 3.24),
    ((0, 3, 0, 0, 0, true), 3.01),
    ((0, 1, 1, 0, 0, false), 12.36),
    ((0, 0, 0, 1, 0, false), 23.79),
    ((0, 1, 2, 0, 0, false), 11.68),
    ((0, 0, 1, 1, 0, false), 13.60),
    ((1, 2, 0, 0, 0, false), 12.03),
    ((1, 2, 0, 0, 0, true), 21.94),
    ((1, 0, 1, 0, 0, false), 23.85),
    ((2, 1, 0, 0, 0, false), 26.02),
    ((0, 4, 0, 0, 1, false), 0.00),
    ((0, 2, 1, 0, 1, false), 3.01),
    ((0, 1, 0, 1, 1, false), 4.36),
    ((1, 3, 0, 0, 1, false), 4.44),
    ((1, 1, 1, 0, 1, false), 13.97),
    ((2, 2, 0, 0, 1, false), 16.61),
    ((2, 0, 1, 0, 1, false), 25.59),
    ((3, 1, 0, 0, 1, false), 27.64),
];

/// Nitrogen, aromatic. Keyed by `(hydrogens, aromatic bonds, single bonds,
/// double bonds, charge)`.
pub const AROMATIC_NITROGEN: &[(AromaticNitrogenEnvironment, f64)] = &[
    ((0, 2, 0, 0, 0), 12.89),
    ((0, 3, 0, 0, 0), 4.41),
    ((0, 2, 1, 0, 0), 4.93),
    ((0, 2, 0, 1, 0), 8.39),
    ((1, 2, 0, 0, 0), 15.79),
    ((0, 3, 0, 0, 1), 4.10),
    ((0, 2, 1, 0, 1), 3.88),
    ((1, 2, 0, 0, 1), 14.14),
];

/// Oxygen. Keyed by `(hydrogens, singles, doubles, charge, aromatic,
/// in three-membered ring)`.
pub const OXYGEN: &[(OxygenEnvironment, f64)] = &[
    ((0, 2, 0, 0, false, false), 9.23),
    ((0, 2, 0, 0, false, true), 12.53),
    ((0, 0, 1, 0, false, false), 17.07),
    ((1, 1, 0, 0, false, false), 20.23),
    ((0, 1, 0, -1, false, false), 23.06),
    ((0, 0, 0, 0, true, false), 13.14),
];

/// Sulfur. Keyed by `(hydrogens, singles, doubles, charge, aromatic)`.
pub const SULFUR: &[(SulfurEnvironment, f64)] = &[
    ((0, 2, 0, 0, false), 25.30),
    ((0, 0, 1, 0, false), 32.09),
    ((0, 2, 1, 0, false), 19.21),
    ((0, 2, 2, 0, false), 8.38),
    ((1, 1, 0, 0, false), 38.80),
    ((0, 0, 0, 0, true), 28.24),
    ((0, 0, 1, 0, true), 21.70),
];

/// Phosphorus. Keyed by `(hydrogens, singles, doubles, charge)`.
pub const PHOSPHORUS: &[(PhosphorusEnvironment, f64)] = &[
    ((0, 3, 0, 0), 13.59),
    ((0, 1, 1, 0), 34.14),
    ((0, 3, 1, 0), 9.81),
    ((1, 2, 1, 0), 23.47),
];
