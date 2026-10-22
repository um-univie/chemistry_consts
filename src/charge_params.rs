//! Charge-equilibration reference data, separate from solver settings.
//!
//! Rappe & Goddard (1991), as used by `cheq`'s `resources/qeq.data.toml`.
//! See `THIRD_PARTY.md` for provenance.

/// Scale factor used by cheq for the QEq Slater exponent (dimensionless).
/// This convention rounds the paper's approximately 0.4913 to 0.5.
pub const QEQ_SLATER_EXPONENT_SCALE: f64 = 0.5;

/// Linear fractional change in hydrogen hardness per elementary charge.
/// The hardness is `J(q) = J(0) * (1 + q * factor)`; this factor is
/// dimensionless when `q` is expressed in elementary charge units.
/// From cheq's `solver::implementation::H_CHARGE_DEPENDENCE_FACTOR`.
pub const QEQ_HYDROGEN_CHARGE_DEPENDENCE: f64 = 0.93475415965;

/// Electronegativity (`chi0`, eV), hardness (`J0`, eV), covalent radius
/// (angstrom) and principal quantum number, keyed by atomic number --
/// `cheq`'s default parameter table (`resources/qeq.data.toml`), itself
/// Rappe & Goddard (1991). Hydrogen's `chi0` is the later-corrected value
/// (4.528) the wider QEq literature settled on, not the original paper's
/// 7.17. The table covers H, B, C, N, O, F, Si, P, S, Cl, Se, Br and I.
pub const QEQ_PARAMETERS: &[(u8, f64, f64, f64, u8)] = &[
    (1, 4.528, 13.8904, 0.371, 1), // H
    (5, 5.11, 9.50, 0.85, 2),      // B
    (6, 5.343, 10.126, 0.759, 2),  // C
    (7, 6.899, 11.760, 0.715, 2),  // N
    (8, 8.741, 13.364, 0.669, 2),  // O
    (9, 10.874, 14.948, 0.706, 2), // F
    (14, 4.168, 6.974, 1.176, 3),  // Si
    (15, 5.463, 8.000, 1.102, 3),  // P
    (16, 6.928, 8.972, 1.047, 3),  // S
    (17, 8.564, 9.892, 0.994, 3),  // Cl
    (34, 6.428, 8.262, 1.20, 4),   // Se
    (35, 7.790, 8.850, 1.141, 4),  // Br
    (53, 6.822, 7.524, 1.333, 5),  // I
];

/// Returns `(electronegativity_eV, hardness_eV, radius_angstrom, principal_n)`.
/// Unsupported atomic numbers return `None`.
pub fn get_qeq_params(atomic_number: u8) -> Option<(f64, f64, f64, u8)> {
    QEQ_PARAMETERS
        .iter()
        .find(|&&(z, ..)| z == atomic_number)
        .map(|&(_, chi0, j0, radius, n)| (chi0, j0, radius, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ANGSTROM_M, BOHR_RADIUS_ANGSTROM, BOHR_RADIUS_M, BONDI_VDW_RADII_ANGSTROM};

    #[test]
    fn sparse_element_tables_are_unique_and_physical() {
        assert!(QEQ_PARAMETERS.windows(2).all(|w| w[0].0 < w[1].0));
        for &(z, chi, hardness, radius, n) in QEQ_PARAMETERS {
            assert!((1..=118).contains(&z));
            assert!(chi.is_finite() && chi > 0.0);
            assert!(hardness.is_finite() && hardness > 0.0);
            assert!(radius.is_finite() && radius > 0.0);
            assert!((1..=7).contains(&n));
            assert!(get_qeq_params(z).is_some());
        }
        assert!(get_qeq_params(0).is_none());
        assert!(get_qeq_params(u8::MAX).is_none());
        assert!(BONDI_VDW_RADII_ANGSTROM.windows(2).all(|w| w[0].0 < w[1].0));
        for &(z, radius) in BONDI_VDW_RADII_ANGSTROM {
            assert!((1..=118).contains(&z));
            assert!(radius.is_finite() && radius > 0.0);
        }
    }

    #[test]
    fn bohr_units_agree_to_floating_point_precision() {
        let converted = BOHR_RADIUS_M / ANGSTROM_M;
        assert!((converted - BOHR_RADIUS_ANGSTROM).abs() <= f64::EPSILON * converted);
    }
}
