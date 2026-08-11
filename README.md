# chemistry_consts

Periodic table data and physical constants for Rust, as plain `const` arrays
indexed by atomic number.

There is no builder, no lazy static, no lookup at runtime. An element property
is an array index, so the compiler can fold most of it away and none of it
allocates. That is the whole design: the data has been the same since 1869 and
does not need a type system wrapped around it.

```toml
[dependencies]
chemistry_consts = "0.3"
```

## Two ways in

Reach for the tables when you already have an atomic number and want the number
back:

```rust
use chemistry_consts::*;

assert_eq!(ATOMIC_SYMBOLS[79], "Au");
assert_eq!(STANDARD_ATOMIC_WEIGHTS[1], Some(1.007975));
```

Or use the `ElementProperties` trait, which is implemented for `&str` and for
the unsigned integer types, so you can ask an element about itself:

```rust
use chemistry_consts::*;

assert_eq!(6u8.atomic_symbol(), Some("C"));
assert_eq!(6u8.monoisotopic_mass(), Some(12.0));
assert_eq!(8u8.electronegativity(), Some(350));      // Pauling, scaled by 100
assert_eq!(17u8.covalent_radius(), Some(1.02));      // angstroms
```

The properties that can have several values hand back an iterator rather than a
slice, so you get them in a meaningful order and nothing borrows:

```rust
use chemistry_consts::*;

// sulfur: 2, 4 and 6 are all real
assert_eq!(16u8.valencies().unwrap().collect::<Vec<_>>(), vec![2, 4, 6]);

// iron, common oxidation states only
assert_eq!(26u8.oxidation_states().unwrap().collect::<Vec<_>>(), vec![2, 3]);

// isotopes come out most abundant first
let chlorine: Vec<_> = 17u8.isotopes().unwrap().map(|i| (i.mass, i.abundance)).collect();
// [(34.968852682, 0.7576), (36.965902602, 0.2424)]
```

`Option` throughout, because plenty of these genuinely have no value: helium
has no electronegativity, iron has no sensible single valence, and the
transuranics have no standard atomic weight.

## What the numbers actually mean

Most of the tables are self-explanatory. These four are not, and getting them
wrong is quiet rather than loud, so they are worth stating plainly.

**`MONOISOTOPIC_MASSES` is the mass of the element's *most abundant* isotope.**
Not the lightest one. For 39 elements those differ, sometimes by a lot: helium's
lightest isotope is helium-3, which is 0.0001% of natural helium. This is the
number you want for an exact mass; use `STANDARD_ATOMIC_WEIGHTS` if you want an
average mass. The array is derived from `ISOTOPES` at compile time rather than
maintained separately, so the two cannot disagree.

For the 34 elements with no stable isotope the entry is the IUPAC conventional
nuclide — the longest-lived one, the same one `STANDARD_ATOMIC_WEIGHTS` names.
`most_abundant_isotope()` returns `None` for those, since there is no abundance
to be most of, while `monoisotopic_mass()` still answers:

```rust
use chemistry_consts::*;

assert_eq!(43u8.most_abundant_isotope(), None);            // technetium
assert_eq!(43u8.monoisotopic_mass(), Some(97.907216));     // Tc-98
```

**`ISOTOPES` is ordered by decreasing abundance and holds at most four per
element.** Entry 0 is therefore the most abundant one, which is what everything
else reads. A dozen elements have more than four natural isotopes and are cut
off, so the abundances of e.g. xenon do not add up to 1.

**`ELECTRONEGATIVITIES_PAULING` and `_ALLEN` are scaled by 100** and stored as
`u16`, so oxygen is `350` and not `3.50`. Integers compare and sort exactly,
which is usually what you want them for; divide when you want to print one.

**`VALENCIES` comes from RDKit's `atomic_data.cpp`,** not from a textbook. It is
what a cheminformatics valence model uses to decide implicit hydrogen counts,
which is why the transition metals are `None` rather than guessed.

## Physical constants

Named with their units, so there is nothing to look up:

```rust
use chemistry_consts::*;

let _ = GAS_CONSTANT_J_PER_MOL_K;   // 8.314462618153240
let _ = AVOGADRO_PER_MOL;           // 6.02214076e23
let _ = HARTREE_KJ_PER_MOL;         // 2625.4996394798254
let _ = BOHR_RADIUS_M;              // 5.29177210903e-11
```

Roughly forty of them, covering the physicochemical set (Avogadro, Boltzmann,
gas constant, molar volume), the atomic set (Hartree, Bohr radius, particle
masses in both kg and amu, fine structure, Rydberg), the fundamental set
(Planck, c, vacuum permittivity and permeability, elementary charge, Faraday),
and the everyday conversions (calorie, atmosphere, zero Celsius, angstrom).
Values from NIST CODATA.

## Symbols are case-insensitive

Files write `Fe`, `FE` and occasionally `fe`. All three work, and the symbol
the crate hands you goes straight back into it:

```rust
use chemistry_consts::*;

assert_eq!("Fe".atomic_number(), Some(26));
assert_eq!("FE".atomic_number(), Some(26));
assert_eq!("fe".atomic_number(), Some(26));

let symbol = 26u8.atomic_symbol().unwrap();          // "Fe"
assert_eq!(symbol.atomic_number(), Some(26));
```

Free, as far as anything is: a symbol is one or two ASCII letters, so the
uppercase form is built in two bytes on the stack and compared as a byte
string. Nothing allocates. Measure it with
`cargo run --release --example symbol_bench`.

## Also in the box

`get_gasteiger_params(atomic_number, Hybridization)` returns the (a, b, c)
parameters for Gasteiger partial charges, taken from RDKit, with hybridization
states down to the sulfoxide and sulfone special cases:

```rust
use chemistry_consts::*;

assert_eq!(get_gasteiger_params(6, Hybridization::Sp3), Some((7.98, 9.18, 1.88)));
```

## Sources

- Atomic weights, isotope masses and abundances: NIST, [Atomic Weights and
  Isotopic Compositions](https://www.nist.gov/pml/atomic-weights-and-isotopic-compositions-relative-atomic-masses)
- Physical constants: [NIST CODATA](https://physics.nist.gov/cuu/Constants/)
- Covalent radii: [Cordero et al., *Dalton Trans.* 2008](https://doi.org/10.1039/B801115J),
  single-bond radii, extended past curium from Wikipedia
- Electronegativities: Pauling (*The Nature of the Chemical Bond*, 3rd ed.) and
  Allen; elements 95–102 from Huheey, Keiter and Keiter
- Oxidation states: Wikipedia's list, in both a common and an exhaustive form
- Valencies and Gasteiger parameters: RDKit

## Testing

The tests are invariants rather than spot checks on values — every table is
indexed by atomic number and the same length, symbols round-trip, isotopes are
ordered by decreasing abundance, abundances are fractions that sum to at most
one, every nuclide mass is near a whole number of nucleons and never below the
proton count, and the two mass tables name the same nuclide for each element.

That is deliberate. A reference crate cannot be tested by re-typing its own
numbers; what it can be tested for is self-consistency, and each of the three
bugs fixed in 0.3.0 is caught by one of these.

## 0.3.0 changes values

If you compute exact masses, they will move. `MONOISOTOPIC_MASSES` previously
held each element's *lightest* isotope for 39 elements, thorium and uranium had
corrupt entries in `ISOTOPES`, and `most_abundant_isotope()` truncated where it
should have rounded, so it was wrong for 74 of the 84 elements it answers for —
oxygen returned 15. No API changed; the numbers did.

Symbol lookup also became case-insensitive, which can only turn a `None` into a
`Some`. If you were uppercasing before calling in, you can stop: it is now a
wasted allocation, and the lookup itself got about twice as fast in the
process.

## Not here yet

Electron affinities, ionisation energies, electron configurations, and pairwise
average bond dissociation energies. Contributions welcome, especially with a
source attached.

## Licence

MIT.
