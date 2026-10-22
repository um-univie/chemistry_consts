//! Universal Force Field parameters, one row per UFF atom type.
//!
//! Rappe, Casewit, Colwell, Goddard & Skiff, *J. Am. Chem. Soc.* **1992**,
//! 114, 10024-10035. Transcribed from RDKit's `Code/ForceField/UFF/Params.cpp`,
//! which carries them under the BSD 3-Clause licence -- see `THIRD_PARTY.md`
//! at the repository root for the notice.
//!
//! UFF names a type by element, hybridization and, where it matters,
//! oxidation state: `C_3` is tetrahedral carbon, `C_R` aromatic, `N_2`
//! trigonal nitrogen, `S_3+6` sulfur in a sulfone. Atom typing belongs
//! to the caller; this module only supplies the reference data.
//!
//! The three GMP columns RDKit's table also carries -- electronegativity,
//! hardness and radius -- are for charge equilibration; see also
//! [`crate::QEQ_PARAMETERS`] for a separate QEq parameter set. Only the
//! electronegativity is kept, because UFF's bond-length rule uses it.

/// One row of the UFF table. Energies are kcal/mol, lengths angstrom,
/// angles degrees.
#[derive(Debug, Clone, Copy)]
pub struct UffType {
    pub label: &'static str,
    /// `r_I`, the valence bond radius.
    pub bond_radius: f64,
    /// `theta_0`, the equilibrium valence angle.
    pub angle: f64,
    /// `x_I`, the van der Waals characteristic length.
    pub vdw_distance: f64,
    /// `D_I`, the van der Waals well depth.
    pub vdw_energy: f64,
    /// `zeta`, the van der Waals scaling term. UFF's Lennard-Jones form
    /// does not use it -- it belongs to the exponential-6 alternative the
    /// paper also defines -- but it is kept so a row here is a row of the
    /// published table rather than a subset of one.
    #[allow(dead_code)]
    pub vdw_scale: f64,
    /// `Z_I`, the effective charge, which sets bond and angle stiffness.
    pub effective_charge: f64,
    /// `V_I`, the sp3 torsional barrier.
    pub sp3_torsion_barrier: f64,
    /// `U_I`, the torsional contribution for sp2-sp3 bonds.
    pub sp2_torsion_contribution: f64,
    /// `chi_I`, GMP electronegativity, used by the bond-length
    /// electronegativity correction.
    pub electronegativity: f64,
}

// Hafnium's van der Waals distance is 3.141 angstrom, which clippy reads as
// a botched pi. It is a measured parameter that happens to land there.
#[allow(clippy::approx_constant)]
#[rustfmt::skip]
pub const UFF_TYPES: &[UffType] = &[
    UffType { label: "H_", bond_radius: 0.354, angle: 180.0, vdw_distance: 2.886, vdw_energy: 0.044, vdw_scale: 12.0, effective_charge: 0.712, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 4.528 },
    UffType { label: "H_b", bond_radius: 0.46, angle: 83.5, vdw_distance: 2.886, vdw_energy: 0.044, vdw_scale: 12.0, effective_charge: 0.712, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 4.528 },
    UffType { label: "He4+4", bond_radius: 0.849, angle: 90.0, vdw_distance: 2.362, vdw_energy: 0.056, vdw_scale: 15.24, effective_charge: 0.098, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 9.66 },
    UffType { label: "Li", bond_radius: 1.336, angle: 180.0, vdw_distance: 2.451, vdw_energy: 0.025, vdw_scale: 12.0, effective_charge: 1.026, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 3.006 },
    UffType { label: "Be3+2", bond_radius: 1.074, angle: 109.47, vdw_distance: 2.745, vdw_energy: 0.085, vdw_scale: 12.0, effective_charge: 1.565, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 4.877 },
    UffType { label: "B_3", bond_radius: 0.838, angle: 109.47, vdw_distance: 4.083, vdw_energy: 0.18, vdw_scale: 12.052, effective_charge: 1.755, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 5.11 },
    UffType { label: "B_2", bond_radius: 0.828, angle: 120.0, vdw_distance: 4.083, vdw_energy: 0.18, vdw_scale: 12.052, effective_charge: 1.755, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 5.11 },
    UffType { label: "C_3", bond_radius: 0.757, angle: 109.47, vdw_distance: 3.851, vdw_energy: 0.105, vdw_scale: 12.73, effective_charge: 1.912, sp3_torsion_barrier: 2.119, sp2_torsion_contribution: 2.0, electronegativity: 5.343 },
    UffType { label: "C_R", bond_radius: 0.729, angle: 120.0, vdw_distance: 3.851, vdw_energy: 0.105, vdw_scale: 12.73, effective_charge: 1.912, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 5.343 },
    UffType { label: "C_2", bond_radius: 0.732, angle: 120.0, vdw_distance: 3.851, vdw_energy: 0.105, vdw_scale: 12.73, effective_charge: 1.912, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 5.343 },
    UffType { label: "C_1", bond_radius: 0.706, angle: 180.0, vdw_distance: 3.851, vdw_energy: 0.105, vdw_scale: 12.73, effective_charge: 1.912, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 5.343 },
    UffType { label: "N_3", bond_radius: 0.7, angle: 106.7, vdw_distance: 3.66, vdw_energy: 0.069, vdw_scale: 13.407, effective_charge: 2.544, sp3_torsion_barrier: 0.45, sp2_torsion_contribution: 2.0, electronegativity: 6.899 },
    UffType { label: "N_R", bond_radius: 0.699, angle: 120.0, vdw_distance: 3.66, vdw_energy: 0.069, vdw_scale: 13.407, effective_charge: 2.544, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 6.899 },
    UffType { label: "N_2", bond_radius: 0.685, angle: 111.2, vdw_distance: 3.66, vdw_energy: 0.069, vdw_scale: 13.407, effective_charge: 2.544, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 6.899 },
    UffType { label: "N_1", bond_radius: 0.656, angle: 180.0, vdw_distance: 3.66, vdw_energy: 0.069, vdw_scale: 13.407, effective_charge: 2.544, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 6.899 },
    UffType { label: "O_3", bond_radius: 0.658, angle: 104.51, vdw_distance: 3.5, vdw_energy: 0.06, vdw_scale: 14.085, effective_charge: 2.3, sp3_torsion_barrier: 0.018, sp2_torsion_contribution: 2.0, electronegativity: 8.741 },
    UffType { label: "O_3_z", bond_radius: 0.528, angle: 146.0, vdw_distance: 3.5, vdw_energy: 0.06, vdw_scale: 14.085, effective_charge: 2.3, sp3_torsion_barrier: 0.018, sp2_torsion_contribution: 2.0, electronegativity: 8.741 },
    UffType { label: "O_R", bond_radius: 0.68, angle: 110.0, vdw_distance: 3.5, vdw_energy: 0.06, vdw_scale: 14.085, effective_charge: 2.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 8.741 },
    UffType { label: "O_2", bond_radius: 0.634, angle: 120.0, vdw_distance: 3.5, vdw_energy: 0.06, vdw_scale: 14.085, effective_charge: 2.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 8.741 },
    UffType { label: "O_1", bond_radius: 0.639, angle: 180.0, vdw_distance: 3.5, vdw_energy: 0.06, vdw_scale: 14.085, effective_charge: 2.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 8.741 },
    UffType { label: "F_", bond_radius: 0.668, angle: 180.0, vdw_distance: 3.364, vdw_energy: 0.05, vdw_scale: 14.762, effective_charge: 1.735, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 10.874 },
    UffType { label: "Ne4+4", bond_radius: 0.92, angle: 90.0, vdw_distance: 3.243, vdw_energy: 0.042, vdw_scale: 15.44, effective_charge: 0.194, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 2.0, electronegativity: 11.04 },
    UffType { label: "Na", bond_radius: 1.539, angle: 180.0, vdw_distance: 2.983, vdw_energy: 0.03, vdw_scale: 12.0, effective_charge: 1.081, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 2.843 },
    UffType { label: "Mg3+2", bond_radius: 1.421, angle: 109.47, vdw_distance: 3.021, vdw_energy: 0.111, vdw_scale: 12.0, effective_charge: 1.787, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 3.951 },
    UffType { label: "Al3", bond_radius: 1.244, angle: 109.47, vdw_distance: 4.499, vdw_energy: 0.505, vdw_scale: 11.278, effective_charge: 1.792, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 4.06 },
    UffType { label: "Si3", bond_radius: 1.117, angle: 109.47, vdw_distance: 4.295, vdw_energy: 0.402, vdw_scale: 12.175, effective_charge: 2.323, sp3_torsion_barrier: 1.225, sp2_torsion_contribution: 1.25, electronegativity: 4.168 },
    UffType { label: "P_3+3", bond_radius: 1.101, angle: 93.8, vdw_distance: 4.147, vdw_energy: 0.305, vdw_scale: 13.072, effective_charge: 2.863, sp3_torsion_barrier: 2.4, sp2_torsion_contribution: 1.25, electronegativity: 5.463 },
    UffType { label: "P_3+5", bond_radius: 1.056, angle: 109.47, vdw_distance: 4.147, vdw_energy: 0.305, vdw_scale: 13.072, effective_charge: 2.863, sp3_torsion_barrier: 2.4, sp2_torsion_contribution: 1.25, electronegativity: 5.463 },
    UffType { label: "P_3+q", bond_radius: 1.056, angle: 109.47, vdw_distance: 4.147, vdw_energy: 0.305, vdw_scale: 13.072, effective_charge: 2.863, sp3_torsion_barrier: 2.4, sp2_torsion_contribution: 1.25, electronegativity: 5.463 },
    UffType { label: "S_3+2", bond_radius: 1.064, angle: 92.1, vdw_distance: 4.035, vdw_energy: 0.274, vdw_scale: 13.969, effective_charge: 2.703, sp3_torsion_barrier: 0.484, sp2_torsion_contribution: 1.25, electronegativity: 6.928 },
    UffType { label: "S_3+4", bond_radius: 1.049, angle: 103.2, vdw_distance: 4.035, vdw_energy: 0.274, vdw_scale: 13.969, effective_charge: 2.703, sp3_torsion_barrier: 0.484, sp2_torsion_contribution: 1.25, electronegativity: 6.928 },
    UffType { label: "S_3+6", bond_radius: 1.027, angle: 109.47, vdw_distance: 4.035, vdw_energy: 0.274, vdw_scale: 13.969, effective_charge: 2.703, sp3_torsion_barrier: 0.484, sp2_torsion_contribution: 1.25, electronegativity: 6.928 },
    UffType { label: "S_R", bond_radius: 1.077, angle: 92.2, vdw_distance: 4.035, vdw_energy: 0.274, vdw_scale: 13.969, effective_charge: 2.703, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 6.928 },
    UffType { label: "S_2", bond_radius: 0.854, angle: 120.0, vdw_distance: 4.035, vdw_energy: 0.274, vdw_scale: 13.969, effective_charge: 2.703, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 6.928 },
    UffType { label: "Cl", bond_radius: 1.044, angle: 180.0, vdw_distance: 3.947, vdw_energy: 0.227, vdw_scale: 14.866, effective_charge: 2.348, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 8.564 },
    UffType { label: "Ar4+4", bond_radius: 1.032, angle: 90.0, vdw_distance: 3.868, vdw_energy: 0.185, vdw_scale: 15.763, effective_charge: 0.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 1.25, electronegativity: 9.465 },
    UffType { label: "K_", bond_radius: 1.953, angle: 180.0, vdw_distance: 3.812, vdw_energy: 0.035, vdw_scale: 12.0, effective_charge: 1.165, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 2.421 },
    UffType { label: "Ca6+2", bond_radius: 1.761, angle: 90.0, vdw_distance: 3.399, vdw_energy: 0.238, vdw_scale: 12.0, effective_charge: 2.141, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.231 },
    UffType { label: "Sc3+3", bond_radius: 1.513, angle: 109.47, vdw_distance: 3.295, vdw_energy: 0.019, vdw_scale: 12.0, effective_charge: 2.592, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.395 },
    UffType { label: "Ti3+4", bond_radius: 1.412, angle: 109.47, vdw_distance: 3.175, vdw_energy: 0.017, vdw_scale: 12.0, effective_charge: 2.659, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.47 },
    UffType { label: "Ti6+4", bond_radius: 1.412, angle: 90.0, vdw_distance: 3.175, vdw_energy: 0.017, vdw_scale: 12.0, effective_charge: 2.659, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.47 },
    UffType { label: "V_3+5", bond_radius: 1.402, angle: 109.47, vdw_distance: 3.144, vdw_energy: 0.016, vdw_scale: 12.0, effective_charge: 2.679, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.65 },
    UffType { label: "Cr6+3", bond_radius: 1.345, angle: 90.0, vdw_distance: 3.023, vdw_energy: 0.015, vdw_scale: 12.0, effective_charge: 2.463, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.415 },
    UffType { label: "Mn6+2", bond_radius: 1.382, angle: 90.0, vdw_distance: 2.961, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 2.43, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.325 },
    UffType { label: "Fe3+2", bond_radius: 1.27, angle: 109.47, vdw_distance: 2.912, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 2.43, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.76 },
    UffType { label: "Fe6+2", bond_radius: 1.335, angle: 90.0, vdw_distance: 2.912, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 2.43, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.76 },
    UffType { label: "Co6+3", bond_radius: 1.241, angle: 90.0, vdw_distance: 2.872, vdw_energy: 0.014, vdw_scale: 12.0, effective_charge: 2.43, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 4.105 },
    UffType { label: "Ni4+2", bond_radius: 1.164, angle: 90.0, vdw_distance: 2.834, vdw_energy: 0.015, vdw_scale: 12.0, effective_charge: 2.43, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 4.465 },
    UffType { label: "Cu3+1", bond_radius: 1.302, angle: 109.47, vdw_distance: 3.495, vdw_energy: 0.005, vdw_scale: 12.0, effective_charge: 1.756, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 4.2 },
    UffType { label: "Zn3+2", bond_radius: 1.193, angle: 109.47, vdw_distance: 2.763, vdw_energy: 0.124, vdw_scale: 12.0, effective_charge: 1.308, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 5.106 },
    UffType { label: "Ga3+3", bond_radius: 1.26, angle: 109.47, vdw_distance: 4.383, vdw_energy: 0.415, vdw_scale: 11.0, effective_charge: 1.821, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 3.641 },
    UffType { label: "Ge3", bond_radius: 1.197, angle: 109.47, vdw_distance: 4.28, vdw_energy: 0.379, vdw_scale: 12.0, effective_charge: 2.789, sp3_torsion_barrier: 0.701, sp2_torsion_contribution: 0.7, electronegativity: 4.051 },
    UffType { label: "As3+3", bond_radius: 1.211, angle: 92.1, vdw_distance: 4.23, vdw_energy: 0.309, vdw_scale: 13.0, effective_charge: 2.864, sp3_torsion_barrier: 1.5, sp2_torsion_contribution: 0.7, electronegativity: 5.188 },
    UffType { label: "Se3+2", bond_radius: 1.19, angle: 90.6, vdw_distance: 4.205, vdw_energy: 0.291, vdw_scale: 14.0, effective_charge: 2.764, sp3_torsion_barrier: 0.335, sp2_torsion_contribution: 0.7, electronegativity: 6.428 },
    UffType { label: "Br", bond_radius: 1.192, angle: 180.0, vdw_distance: 4.189, vdw_energy: 0.251, vdw_scale: 15.0, effective_charge: 2.519, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 7.79 },
    UffType { label: "Kr4+4", bond_radius: 1.147, angle: 90.0, vdw_distance: 4.141, vdw_energy: 0.22, vdw_scale: 16.0, effective_charge: 0.452, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.7, electronegativity: 8.505 },
    UffType { label: "Rb", bond_radius: 2.26, angle: 180.0, vdw_distance: 4.114, vdw_energy: 0.04, vdw_scale: 12.0, effective_charge: 1.592, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 2.331 },
    UffType { label: "Sr6+2", bond_radius: 2.052, angle: 90.0, vdw_distance: 3.641, vdw_energy: 0.235, vdw_scale: 12.0, effective_charge: 2.449, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.024 },
    UffType { label: "Y_3+3", bond_radius: 1.698, angle: 109.47, vdw_distance: 3.345, vdw_energy: 0.072, vdw_scale: 12.0, effective_charge: 3.257, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.83 },
    UffType { label: "Zr3+4", bond_radius: 1.564, angle: 109.47, vdw_distance: 3.124, vdw_energy: 0.069, vdw_scale: 12.0, effective_charge: 3.667, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.4 },
    UffType { label: "Nb3+5", bond_radius: 1.473, angle: 109.47, vdw_distance: 3.165, vdw_energy: 0.059, vdw_scale: 12.0, effective_charge: 3.618, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.55 },
    UffType { label: "Mo6+6", bond_radius: 1.467, angle: 90.0, vdw_distance: 3.052, vdw_energy: 0.056, vdw_scale: 12.0, effective_charge: 3.4, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.465 },
    UffType { label: "Mo3+6", bond_radius: 1.484, angle: 109.47, vdw_distance: 3.052, vdw_energy: 0.056, vdw_scale: 12.0, effective_charge: 3.4, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.465 },
    UffType { label: "Tc6+5", bond_radius: 1.322, angle: 90.0, vdw_distance: 2.998, vdw_energy: 0.048, vdw_scale: 12.0, effective_charge: 3.4, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.29 },
    UffType { label: "Ru6+2", bond_radius: 1.478, angle: 90.0, vdw_distance: 2.963, vdw_energy: 0.056, vdw_scale: 12.0, effective_charge: 3.4, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.575 },
    UffType { label: "Rh6+3", bond_radius: 1.332, angle: 90.0, vdw_distance: 2.929, vdw_energy: 0.053, vdw_scale: 12.0, effective_charge: 3.5, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.975 },
    UffType { label: "Pd4+2", bond_radius: 1.338, angle: 90.0, vdw_distance: 2.899, vdw_energy: 0.048, vdw_scale: 12.0, effective_charge: 3.21, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 4.32 },
    UffType { label: "Ag1+1", bond_radius: 1.386, angle: 180.0, vdw_distance: 3.148, vdw_energy: 0.036, vdw_scale: 12.0, effective_charge: 1.956, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 4.436 },
    UffType { label: "Cd3+2", bond_radius: 1.403, angle: 109.47, vdw_distance: 2.848, vdw_energy: 0.228, vdw_scale: 12.0, effective_charge: 1.65, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 5.034 },
    UffType { label: "In3+3", bond_radius: 1.459, angle: 109.47, vdw_distance: 4.463, vdw_energy: 0.599, vdw_scale: 11.0, effective_charge: 2.07, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 3.506 },
    UffType { label: "Sn3", bond_radius: 1.398, angle: 109.47, vdw_distance: 4.392, vdw_energy: 0.567, vdw_scale: 12.0, effective_charge: 2.961, sp3_torsion_barrier: 0.199, sp2_torsion_contribution: 0.2, electronegativity: 3.987 },
    UffType { label: "Sb3+3", bond_radius: 1.407, angle: 91.6, vdw_distance: 4.42, vdw_energy: 0.449, vdw_scale: 13.0, effective_charge: 2.704, sp3_torsion_barrier: 1.1, sp2_torsion_contribution: 0.2, electronegativity: 4.899 },
    UffType { label: "Te3+2", bond_radius: 1.386, angle: 90.25, vdw_distance: 4.47, vdw_energy: 0.398, vdw_scale: 14.0, effective_charge: 2.882, sp3_torsion_barrier: 0.3, sp2_torsion_contribution: 0.2, electronegativity: 5.816 },
    UffType { label: "I_", bond_radius: 1.382, angle: 180.0, vdw_distance: 4.5, vdw_energy: 0.339, vdw_scale: 15.0, effective_charge: 2.65, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 6.822 },
    UffType { label: "Xe4+4", bond_radius: 1.267, angle: 90.0, vdw_distance: 4.404, vdw_energy: 0.332, vdw_scale: 12.0, effective_charge: 0.556, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.2, electronegativity: 7.595 },
    UffType { label: "Cs", bond_radius: 2.57, angle: 180.0, vdw_distance: 4.517, vdw_energy: 0.045, vdw_scale: 12.0, effective_charge: 1.573, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.183 },
    UffType { label: "Ba6+2", bond_radius: 2.277, angle: 90.0, vdw_distance: 3.703, vdw_energy: 0.364, vdw_scale: 12.0, effective_charge: 2.727, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.814 },
    UffType { label: "La3+3", bond_radius: 1.943, angle: 109.47, vdw_distance: 3.522, vdw_energy: 0.017, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.8355 },
    UffType { label: "Ce6+3", bond_radius: 1.841, angle: 90.0, vdw_distance: 3.556, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.774 },
    UffType { label: "Pr6+3", bond_radius: 1.823, angle: 90.0, vdw_distance: 3.606, vdw_energy: 0.01, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.858 },
    UffType { label: "Nd6+3", bond_radius: 1.816, angle: 90.0, vdw_distance: 3.575, vdw_energy: 0.01, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.8685 },
    UffType { label: "Pm6+3", bond_radius: 1.801, angle: 90.0, vdw_distance: 3.547, vdw_energy: 0.009, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.881 },
    UffType { label: "Sm6+3", bond_radius: 1.78, angle: 90.0, vdw_distance: 3.52, vdw_energy: 0.008, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.9115 },
    UffType { label: "Eu6+3", bond_radius: 1.771, angle: 90.0, vdw_distance: 3.493, vdw_energy: 0.008, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.8785 },
    UffType { label: "Gd6+3", bond_radius: 1.735, angle: 90.0, vdw_distance: 3.368, vdw_energy: 0.009, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.1665 },
    UffType { label: "Tb6+3", bond_radius: 1.732, angle: 90.0, vdw_distance: 3.451, vdw_energy: 0.007, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.018 },
    UffType { label: "Dy6+3", bond_radius: 1.71, angle: 90.0, vdw_distance: 3.428, vdw_energy: 0.007, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.0555 },
    UffType { label: "Ho6+3", bond_radius: 1.696, angle: 90.0, vdw_distance: 3.409, vdw_energy: 0.007, vdw_scale: 12.0, effective_charge: 3.416, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.127 },
    UffType { label: "Er6+3", bond_radius: 1.673, angle: 90.0, vdw_distance: 3.391, vdw_energy: 0.007, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.1865 },
    UffType { label: "Tm6+3", bond_radius: 1.66, angle: 90.0, vdw_distance: 3.374, vdw_energy: 0.006, vdw_scale: 12.0, effective_charge: 3.3, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.2514 },
    UffType { label: "Yb6+3", bond_radius: 1.637, angle: 90.0, vdw_distance: 3.355, vdw_energy: 0.228, vdw_scale: 12.0, effective_charge: 2.618, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.2889 },
    UffType { label: "Lu6+3", bond_radius: 1.671, angle: 90.0, vdw_distance: 3.64, vdw_energy: 0.041, vdw_scale: 12.0, effective_charge: 3.271, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 2.9629 },
    UffType { label: "Hf3+4", bond_radius: 1.611, angle: 109.47, vdw_distance: 3.141, vdw_energy: 0.072, vdw_scale: 12.0, effective_charge: 3.921, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.7 },
    UffType { label: "Ta3+5", bond_radius: 1.511, angle: 109.47, vdw_distance: 3.17, vdw_energy: 0.081, vdw_scale: 12.0, effective_charge: 4.075, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 5.1 },
    UffType { label: "W_6+6", bond_radius: 1.392, angle: 90.0, vdw_distance: 3.069, vdw_energy: 0.067, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.63 },
    UffType { label: "W_3+4", bond_radius: 1.526, angle: 109.47, vdw_distance: 3.069, vdw_energy: 0.067, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.63 },
    UffType { label: "W_3+6", bond_radius: 1.38, angle: 109.47, vdw_distance: 3.069, vdw_energy: 0.067, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.63 },
    UffType { label: "Re6+5", bond_radius: 1.372, angle: 90.0, vdw_distance: 2.954, vdw_energy: 0.066, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.96 },
    UffType { label: "Re3+7", bond_radius: 1.314, angle: 109.47, vdw_distance: 2.954, vdw_energy: 0.066, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.96 },
    UffType { label: "Os6+6", bond_radius: 1.372, angle: 90.0, vdw_distance: 3.12, vdw_energy: 0.037, vdw_scale: 12.0, effective_charge: 3.7, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 5.14 },
    UffType { label: "Ir6+3", bond_radius: 1.371, angle: 90.0, vdw_distance: 2.84, vdw_energy: 0.073, vdw_scale: 12.0, effective_charge: 3.731, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 5.0 },
    UffType { label: "Pt4+2", bond_radius: 1.364, angle: 90.0, vdw_distance: 2.754, vdw_energy: 0.08, vdw_scale: 12.0, effective_charge: 3.382, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.79 },
    UffType { label: "Au4+3", bond_radius: 1.262, angle: 90.0, vdw_distance: 3.293, vdw_energy: 0.039, vdw_scale: 12.0, effective_charge: 2.625, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.894 },
    UffType { label: "Hg1+2", bond_radius: 1.34, angle: 180.0, vdw_distance: 2.705, vdw_energy: 0.385, vdw_scale: 12.0, effective_charge: 1.75, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 6.27 },
    UffType { label: "Tl3+3", bond_radius: 1.518, angle: 120.0, vdw_distance: 4.347, vdw_energy: 0.68, vdw_scale: 11.0, effective_charge: 2.068, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 3.2 },
    UffType { label: "Pb3", bond_radius: 1.459, angle: 109.47, vdw_distance: 4.297, vdw_energy: 0.663, vdw_scale: 12.0, effective_charge: 2.846, sp3_torsion_barrier: 0.1, sp2_torsion_contribution: 0.1, electronegativity: 3.9 },
    UffType { label: "Bi3+3", bond_radius: 1.512, angle: 90.0, vdw_distance: 4.37, vdw_energy: 0.518, vdw_scale: 13.0, effective_charge: 2.47, sp3_torsion_barrier: 1.0, sp2_torsion_contribution: 0.1, electronegativity: 4.69 },
    UffType { label: "Po3+2", bond_radius: 1.5, angle: 90.0, vdw_distance: 4.709, vdw_energy: 0.325, vdw_scale: 14.0, effective_charge: 2.33, sp3_torsion_barrier: 0.3, sp2_torsion_contribution: 0.1, electronegativity: 4.21 },
    UffType { label: "At", bond_radius: 1.545, angle: 180.0, vdw_distance: 4.75, vdw_energy: 0.284, vdw_scale: 15.0, effective_charge: 2.24, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 4.75 },
    UffType { label: "Rn4+4", bond_radius: 1.42, angle: 90.0, vdw_distance: 4.765, vdw_energy: 0.248, vdw_scale: 16.0, effective_charge: 0.583, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.1, electronegativity: 5.37 },
    UffType { label: "Fr", bond_radius: 2.88, angle: 180.0, vdw_distance: 4.9, vdw_energy: 0.05, vdw_scale: 12.0, effective_charge: 1.847, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.0 },
    UffType { label: "Ra6+2", bond_radius: 2.512, angle: 90.0, vdw_distance: 3.677, vdw_energy: 0.404, vdw_scale: 12.0, effective_charge: 2.92, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.843 },
    UffType { label: "Ac6+3", bond_radius: 1.983, angle: 90.0, vdw_distance: 3.478, vdw_energy: 0.033, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.835 },
    UffType { label: "Th6+4", bond_radius: 1.721, angle: 90.0, vdw_distance: 3.396, vdw_energy: 0.026, vdw_scale: 12.0, effective_charge: 4.202, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.175 },
    UffType { label: "Pa6+4", bond_radius: 1.711, angle: 90.0, vdw_distance: 3.424, vdw_energy: 0.022, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.985 },
    UffType { label: "U_6+4", bond_radius: 1.684, angle: 90.0, vdw_distance: 3.395, vdw_energy: 0.022, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.341 },
    UffType { label: "Np6+4", bond_radius: 1.666, angle: 90.0, vdw_distance: 3.424, vdw_energy: 0.019, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.549 },
    UffType { label: "Pu6+4", bond_radius: 1.657, angle: 90.0, vdw_distance: 3.424, vdw_energy: 0.016, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.243 },
    UffType { label: "Am6+4", bond_radius: 1.66, angle: 90.0, vdw_distance: 3.381, vdw_energy: 0.014, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.9895 },
    UffType { label: "Cm6+3", bond_radius: 1.801, angle: 90.0, vdw_distance: 3.326, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 2.8315 },
    UffType { label: "Bk6+3", bond_radius: 1.761, angle: 90.0, vdw_distance: 3.339, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.1935 },
    UffType { label: "Cf6+3", bond_radius: 1.75, angle: 90.0, vdw_distance: 3.313, vdw_energy: 0.013, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.197 },
    UffType { label: "Es6+3", bond_radius: 1.724, angle: 90.0, vdw_distance: 3.299, vdw_energy: 0.012, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.333 },
    UffType { label: "Fm6+3", bond_radius: 1.712, angle: 90.0, vdw_distance: 3.286, vdw_energy: 0.012, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.4 },
    UffType { label: "Md6+3", bond_radius: 1.689, angle: 90.0, vdw_distance: 3.274, vdw_energy: 0.011, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.47 },
    UffType { label: "No6+3", bond_radius: 1.679, angle: 90.0, vdw_distance: 3.248, vdw_energy: 0.011, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.475 },
    UffType { label: "Lw6+3", bond_radius: 1.698, angle: 90.0, vdw_distance: 3.236, vdw_energy: 0.011, vdw_scale: 12.0, effective_charge: 3.9, sp3_torsion_barrier: 0.0, sp2_torsion_contribution: 0.0, electronegativity: 3.5 },
];

/// The row for `label`, or `None` if the table has no such type.
pub fn lookup(label: &str) -> Option<&'static UffType> {
    UFF_TYPES.iter().find(|t| t.label == label)
}

/// UFF electrostatic prefactor in kcal Å / (mol e²), at the model's precision.
/// Deliberately distinct from MMFF's prefactor.
pub const COULOMB_KCAL_ANGSTROM_PER_MOL: f64 = 332.06;
/// Dimensionless scale of the logarithmic bond-order correction.
pub const BOND_ORDER_LAMBDA: f64 = 0.1332;
/// Effective C-N bond order for amide resonance.
pub const AMIDE_BOND_ORDER: f64 = 1.41;
/// Carbonyl inversion barrier in kcal/mol, shared among three permutations.
pub const CARBONYL_INVERSION_BARRIER: f64 = 50.0;
/// Trigonal inversion barrier in kcal/mol, shared among three permutations.
pub const TRIGONAL_INVERSION_BARRIER: f64 = 6.0;
/// Pyramidal inversion barrier in kcal/mol, shared among three permutations.
pub const PYRAMIDAL_INVERSION_BARRIER: f64 = 22.0;

/// Equilibrium pyramidal inversion angles `(atomic_number, degrees)`.
/// The supported centres are phosphorus, arsenic, antimony and bismuth.
pub const PYRAMIDAL_INVERSION_ANGLES: &[(u8, f64)] =
    &[(15, 84.4339), (33, 86.9735), (51, 87.7047), (83, 90.0)];
/// Conjugated torsion prefactor in UFF equation 17 (dimensionless).
pub const CONJUGATED_TORSION_PREFACTOR: f64 = 5.0;
/// Logarithmic bond-order multiplier in UFF equation 17 (dimensionless).
pub const TORSION_BOND_ORDER_SCALE: f64 = 4.18;
/// Oxygen's special sp3-sp3 chalcogen torsion barrier, in kcal/mol.
pub const OXYGEN_TORSION_BARRIER: f64 = 2.0;
/// Other chalcogens' special sp3-sp3 torsion barrier, in kcal/mol.
pub const HEAVY_CHALCOGEN_TORSION_BARRIER: f64 = 6.8;
/// Propene-like sp2-sp3 torsion barrier in kcal/mol.
pub const PROPENE_TORSION_BARRIER: f64 = 2.0;
/// Generic sixfold sp2-sp3 torsion barrier in kcal/mol.
pub const MIXED_TORSION_BARRIER: f64 = 1.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_have_unique_labels_and_positive_scales() {
        let mut labels = std::collections::HashSet::new();
        for row in UFF_TYPES {
            assert!(labels.insert(row.label));
            assert!(lookup(row.label).is_some());
            assert!(row.bond_radius > 0.0 && row.vdw_distance > 0.0);
            assert!(row.vdw_energy > 0.0 && row.electronegativity > 0.0);
            assert!((0.0..=180.0).contains(&row.angle));
        }
        assert!(lookup("unknown").is_none());
    }
}
