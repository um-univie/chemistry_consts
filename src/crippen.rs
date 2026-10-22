//! Wildman-Crippen atomic contributions to log P and molar refractivity.
//!
//! Reference contributions from the 1999 model, as distributed with RDKit.
//! Log P is dimensionless; molar refractivity is in cm³/mol. Atom typing is
//! the caller's responsibility. See `THIRD_PARTY.md` for attribution.

/// One of the Wildman-Crippen atom classes.
///
/// [`CrippenType::None`] is not a class but the absence of one: an element the
/// model was never fitted for -- a noble gas, an actinide -- contributes
/// nothing, which is a statement about the model's reach rather than about the
/// atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum CrippenType {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    CS,
    H1,
    H2,
    H3,
    H4,
    HS,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    N11,
    N12,
    N13,
    N14,
    NS,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
    OS,
    F,
    Cl,
    Br,
    I,
    Hal,
    P,
    S1,
    S2,
    S3,
    Me1,
    Me2,
    None,
}

impl CrippenType {
    /// The class's name, as the published table gives it.
    pub fn label(self) -> &'static str {
        self.entry().0
    }

    /// What one atom of this class contributes: `(log P, molar refractivity)`.
    ///
    /// A few classes were published without a refractivity, and contribute
    /// nothing to it.
    pub fn contribution(self) -> (f64, f64) {
        let entry = self.entry();
        (entry.1, entry.2)
    }

    fn entry(self) -> (&'static str, f64, f64) {
        use CrippenType::*;
        match self {
            C1 => ("C1", 0.1441, 2.503),
            C2 => ("C2", 0.0, 2.433),
            C3 => ("C3", -0.2035, 2.753),
            C4 => ("C4", -0.2051, 2.731),
            C5 => ("C5", -0.2783, 5.007),
            C6 => ("C6", 0.1551, 3.513),
            C7 => ("C7", 0.0017, 3.888),
            C8 => ("C8", 0.08452, 2.464),
            C9 => ("C9", -0.1444, 2.412),
            C10 => ("C10", -0.0516, 2.488),
            C11 => ("C11", 0.1193, 2.582),
            C12 => ("C12", -0.0967, 2.576),
            C13 => ("C13", -0.5443, 4.041),
            C14 => ("C14", 0.0, 3.257),
            C15 => ("C15", 0.245, 3.564),
            C16 => ("C16", 0.198, 3.18),
            C17 => ("C17", 0.0, 3.104),
            C18 => ("C18", 0.1581, 3.35),
            C19 => ("C19", 0.2955, 4.346),
            C20 => ("C20", 0.2713, 3.904),
            C21 => ("C21", 0.136, 3.509),
            C22 => ("C22", 0.4619, 4.067),
            C23 => ("C23", 0.5437, 3.853),
            C24 => ("C24", 0.1893, 2.673),
            C25 => ("C25", -0.8186, 3.135),
            C26 => ("C26", 0.264, 4.305),
            C27 => ("C27", 0.2148, 2.693),
            CS => ("CS", 0.08129, 3.243),
            H1 => ("H1", 0.123, 1.057),
            H2 => ("H2", -0.2677, 1.395),
            H3 => ("H3", 0.2142, 0.9627),
            H4 => ("H4", 0.298, 1.805),
            HS => ("HS", 0.1125, 1.112),
            N1 => ("N1", -1.019, 2.262),
            N2 => ("N2", -0.7096, 2.173),
            N3 => ("N3", -1.027, 2.827),
            N4 => ("N4", -0.5188, 3.0),
            N5 => ("N5", 0.08387, 1.757),
            N6 => ("N6", 0.1836, 2.428),
            N7 => ("N7", -0.3187, 1.839),
            N8 => ("N8", -0.4458, 2.819),
            N9 => ("N9", 0.01508, 1.725),
            N10 => ("N10", -1.95, 0.0),
            N11 => ("N11", -0.3239, 2.202),
            N12 => ("N12", -1.119, 0.0),
            N13 => ("N13", -0.3396, 0.2604),
            N14 => ("N14", 0.2887, 3.359),
            NS => ("NS", -0.4806, 2.134),
            O1 => ("O1", 0.1552, 1.08),
            O2 => ("O2", -0.2893, 0.8238),
            O3 => ("O3", -0.0684, 1.085),
            O4 => ("O4", -0.4195, 1.182),
            O5 => ("O5", 0.0335, 3.367),
            O6 => ("O6", -0.3339, 0.7774),
            O7 => ("O7", -1.189, 0.0),
            O8 => ("O8", 0.1788, 3.135),
            O9 => ("O9", -0.1526, 0.0),
            O10 => ("O10", 0.1129, 0.2215),
            O11 => ("O11", 0.4833, 0.389),
            O12 => ("O12", -1.326, 0.0),
            OS => ("OS", -0.1188, 0.6865),
            F => ("F", 0.4202, 1.108),
            Cl => ("Cl", 0.6895, 5.853),
            Br => ("Br", 0.8456, 8.927),
            I => ("I", 0.8857, 14.02),
            Hal => ("Hal", -2.996, 0.0),
            P => ("P", 0.8612, 6.92),
            S1 => ("S1", 0.6482, 7.591),
            S2 => ("S2", -0.0024, 7.365),
            S3 => ("S3", 0.6237, 6.691),
            Me1 => ("Me1", -0.3808, 5.754),
            Me2 => ("Me2", -0.0025, 0.0),
            None => ("", 0.0, 0.0),
        }
    }
}
