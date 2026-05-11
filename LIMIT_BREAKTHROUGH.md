# LIMIT_BREAKTHROUGH.md — hexa-cern real-limits audit (Wave M)

> Universal real-limits audit per `LATTICE_POLICY.md §1.2`.
> Scope: accelerator physics — beam dynamics, RF cavity gradient, magnet B-field, luminosity, detector resolution.
> SI units. Sources: **PDG 2024**, **CERN public data (LHC Run 3 / FCC CDR / HL-LHC TDR)**, **NIST CODATA 2022**.

---

## §1 Domain identification

`hexa-cern` covers CERN-style high-energy accelerator physics:
classical mechanics, particle accelerators, mini-accelerator (PWFA/LPA), beam dynamics, RF cavity, magnet, detector, firmware control.

**Distinct from `hexa-physics`** in that the limits here are **engineering-dominated**: most are SOFT_WALL or BREAKABLE_WITH_TECH. The *outputs* (energy reach, luminosity) are bounded by hard physical walls (synchrotron radiation, Lorentz force, c) but the *machines* operate far below those.

---

## §2 Real limits applicable

| # | Limit | Class | Formula / current value | Source |
|---|-------|-------|------------------------|--------|
| L1 | Speed of light c | Physical / HARD | 299 792 458 m/s | NIST |
| L2 | RF cavity gradient (SRF, Nb) | Engineering / SOFT | ~50 MV/m (sustainable); ~70 MV/m peak | LHC TDR; XFEL TDR |
| L3 | Laser-plasma wakefield gradient | Engineering / BREAKABLE | 1–10 GV/m demonstrated; 1 TV/m theoretical | AWAKE Run 2; Esarey 2009 |
| L4 | Dipole magnet field (NbTi/Nb₃Sn) | Engineering / SOFT | LHC NbTi: 8.33 T; HL-LHC Nb₃Sn: 11 T; FCC target: 16 T | HL-LHC TDR; FCC CDR |
| L5 | Synchrotron radiation loss | Physical / HARD | ΔE/turn ∝ E⁴/ρ | Schwinger 1949 |
| L6 | Luminosity ℒ | Engineering / SOFT | LHC Run 3: ~2×10³⁴ cm⁻²s⁻¹; HL-LHC: 5–7×10³⁴; FCC-hh: 30×10³⁴ | CERN reports |
| L7 | Detector spatial resolution (Si tracker) | Engineering / SOFT | ~10 µm (ATLAS ITk); ~5 µm (timing layers) | ATLAS ITk TDR |
| L8 | Detector timing resolution | Engineering / SOFT | ~30 ps (HL-LHC HGTD/MTD) | HGTD TDR |
| L9 | Beam emittance (normalized) | Engineering / SOFT | LHC: ~2.5 µm·rad; FCC-ee Z-pole: ~0.27 nm·rad | FCC CDR |
| L10 | Accelerator radius (bending) | Engineering / SOFT | LHC: 4.3 km; FCC: 14 km design; PWFA: ~m-scale | CERN reports |
| L11 | Planck energy (theoretical ceiling) | Physical / HARD | 1.22×10¹⁹ GeV (~10¹⁵× LHC) | NIST |
| L12 | Heisenberg (vertex resolution) | Physical / HARD | ~ℏc/E ≈ 0.2 fm at 1 TeV | NIST |

---

## §3 Per-limit breakthrough assessment

### L1 — Speed of light — **HARD_WALL**

Particles approach c asymptotically. LHC protons γ ≈ 7461 → v ≈ (1 − 9×10⁻⁹)·c. **HARD_WALL.**

### L2 — RF cavity gradient ~50 MV/m — **SOFT_WALL**

| Item | Value |
|------|-------|
| Current best (SRF Nb, 9-cell TESLA) | ~35–40 MV/m operational, 50–60 MV/m max measured |
| Theoretical ceiling (Nb critical field) | ~50 MV/m sustained (RF surface field ~200 mT) |
| Path to higher | Nb₃Sn (T_c = 18 K, H_sh ~400 mT) → potential 90 MV/m |
| Honest verdict | **SOFT_WALL** — Nb is near ceiling; Nb₃Sn is R&D frontier |

### L3 — Laser-plasma wakefield 1–10 GV/m — **BREAKABLE_WITH_TECH**

| Item | Value |
|------|-------|
| Demonstrated (LBNL BELLA, 2019) | 7.8 GeV in 20 cm → ~40 GV/m average |
| AWAKE (CERN proton-driven PWFA) | 2 GV/m demonstrated |
| Theoretical ceiling | E_wb = m_e·c·ω_p/e; at n_e = 10¹⁹ cm⁻³, E_wb ≈ 100 GV/m |
| Engineering gap to deployable collider | beam quality, staging, energy efficiency (currently <1% wall-plug → particle) |
| Honest verdict | **BREAKABLE_WITH_TECH** — 100–1000× over RF; demonstrated but not yet collider-ready |

### L4 — Dipole magnet 8.33 T → 16 T — **SOFT_WALL (Nb₃Sn ceiling ~16 T)**

| Magnet | B (T) | Conductor |
|--------|-------|-----------|
| LHC | 8.33 | NbTi @ 1.9 K |
| HL-LHC | 11 | Nb₃Sn @ 1.9 K |
| FCC-hh target | 16 | Nb₃Sn @ 1.9 K |
| Future R&D (HTS) | 20–24 | REBCO / Bi-2212 |
| Material ceiling (Nb₃Sn) | ~16–17 T | Critical-field limit |

**SOFT_WALL** at 16 T for Nb₃Sn; **BREAKABLE_WITH_TECH** with HTS (REBCO) where lab solenoids reach 32 T (NHMFL 2019) — but HTS dipoles for collider use are R&D-stage.

### L5 — Synchrotron radiation loss — **HARD_WALL**

ΔE/turn = (4π/3) · α·ℏc · (γ⁴/ρ) per electron.
For LEP-2 (104 GeV, 4.3 km): ~3 GeV/turn → ultimate ceiling for circular e⁺e⁻ at LEP scale.
FCC-ee at 182.5 GeV (tt̄ threshold) needs 14 km radius to stay tractable.
**HARD_WALL** — physical inevitability; engineering choice is *radius* trade.

### L6 — Luminosity ℒ — **SOFT_WALL**

ℒ = (f_rev·n_b·N²) / (4π·σ_x·σ_y) · F (geometric).
HL-LHC: 5–7×10³⁴ cm⁻²s⁻¹. Bounded by:
- beam-beam tune-shift (ξ ≲ 0.01 hadrons, 0.1 leptons)
- detector pileup tolerance (~200 pp at HL-LHC)
- power (~1 kW/m beam-induced heating)

**SOFT_WALL** — engineering trade-space; FCC-hh aims 30×10³⁴ via lower β* + crab cavities.

### L7 — Si tracker resolution ~10 µm — **SOFT_WALL**

| Detector | Pitch | Resolution |
|----------|-------|------------|
| LHC ATLAS Pixel (current) | 50×400 µm | ~10 µm |
| ATLAS ITk (HL-LHC) | 50×50 µm | ~5 µm |
| Monolithic CMOS (R&D) | 25×25 µm | ~3 µm |
| Theoretical (Heisenberg, 1 TeV) | ~0.2 fm | unreachable |

**SOFT_WALL** — 10⁻¹³ orders headroom before Heisenberg matters.

### L8 — Timing resolution ~30 ps — **SOFT_WALL**

Path: LGAD (Low-Gain Avalanche Detector) → ~30 ps (HL-LHC HGTD). R&D → 10 ps (DRD3, EIC). **SOFT_WALL.**

### L9 — Beam emittance — **SOFT_WALL**

LHC normalized emittance ε_n ≈ 2.5 µm·rad. Theoretical floor: quantum-limited photon emission in damping rings ~ µeV-scale. **SOFT_WALL** — engineering frontier.

### L10 — Accelerator radius — **SOFT_WALL via PWFA**

LHC: 4.3 km. FCC: 14 km. PWFA: ~10–100 m for same energy. Replacing km with m **is the real breakthrough**. **BREAKABLE_WITH_TECH** — see L3.

### L11 — Planck energy 10¹⁹ GeV — **HARD_WALL**

LHC = 1.4×10⁻¹⁵·E_P. FCC-hh = 10⁻¹⁴·E_P. Even cosmic-ray observation peaks ~10²⁰ eV = 10⁻⁸·E_P. **HARD_WALL.**

### L12 — Heisenberg vertex resolution — **HARD_WALL but irrelevant**

Δx ≥ ℏc/E. At 13.6 TeV: Δx ≈ 10⁻²⁰ m. Detectors at ~10⁻⁵ m. 15 orders headroom. **HARD_WALL** but not the operational limit.

---

## §4 Top-3 breakthrough opportunities (honest)

### #1 — Laser/plasma-wakefield acceleration (real, demonstrated)

- **Limit type**: BREAKABLE_WITH_TECH on cost-per-GeV; SOFT_WALL on wave-breaking field
- **Current**: 7.8 GeV/20 cm (BELLA), 2 GV/m (AWAKE)
- **Path**: staging, beam-quality, wall-plug efficiency (<1% today → ~10% needed)
- **Theoretical ceiling**: ~100 GV/m (wave-breaking field at 10¹⁹ cm⁻³)
- **Honest verdict**: real ~100× gradient improvement; **but full TeV-class lepton collider requires solving staging, beam-quality, repetition rate — engineering decade away**.

### #2 — HTS dipole magnets toward 20+ T (real, R&D)

- **Limit type**: SOFT_WALL on Nb₃Sn (16 T); BREAKABLE with REBCO/HTS
- **Current**: 32 T HTS solenoid (NHMFL 2019, all-superconducting); 14 T HTS dipole prototypes
- **Path**: stress-management, quench protection, conductor cost
- **Honest verdict**: BREAKABLE_WITH_TECH — FCC-hh upgrade path or muon-collider enabler.

### #3 — Muon collider (real but distant)

- **Limit type**: SOFT_WALL on muon cooling (ionization cooling demonstrated by MICE, 2020)
- **Current**: no muon collider exists
- **Path**: rapid acceleration (μ lifetime 2.2 µs), MAP/IMCC R&D
- **Honest verdict**: would deliver 10 TeV pp-equivalent in ~10 km ring (vs FCC 100 km). Engineering not physics. Real but ~30-year timeline.

### Not in top-3 (over-hyped)

| Claim | Reality |
|-------|---------|
| "Tabletop LHC equivalent" | Energy ≠ luminosity; PWFA stages still bulk for full integrated lumi. HARD_WALL on integrated cross-section physics. |
| "Reach Planck scale on Earth" | LHC at 10⁻¹⁵·E_P; need 10¹⁵× more energy. **HARD_WALL.** |
| "Negative-mass particles" | No experimental evidence; LHC searches null. **HARD_WALL.** |

---

## §5 Honest caveats

1. **CERN/HL-LHC/FCC numbers are public** — TDRs and CDRs cited.
2. **Engineering frontier is real here.** Unlike `hexa-physics` (mostly HARD walls), `hexa-cern` is a domain where most operating limits are *engineering-bounded* — gradient, B-field, emittance, luminosity all have 10×–1000× headroom before hitting physics walls.
3. **Synchrotron radiation (L5) IS a hard wall** for circular e⁺e⁻ — this is why FCC-ee uses 14 km ring and why future lepton colliders consider linear (ILC) or PWFA paths.
4. **Energy ceiling (L11) is unbreakable for terrestrial accelerators.** Planck-scale physics is observational (UHECR, GW) not collider-accessible.
5. **Lattice disclaimer (LATTICE_POLICY §1.2)**: accelerator parameters (8.33 T LHC, 14 km FCC, 50 MV/m SRF) are CERN-specified, not n=6 projections.
6. **Soft-wall count: 7/12** — accelerators are mostly engineering, mostly improvable, but capped by physical bounds at the top.

---

## §6 References

- **PDG 2024 Review of Particle Physics**, R.L. Workman et al., PTEP (2024)
- **FCC Conceptual Design Report**, EPJ ST 228 (2019)
- **HL-LHC Technical Design Report**, CERN-2017-007-M
- **ATLAS ITk Strip TDR**, CERN-LHCC-2017-005
- **HGTD TDR**, CERN-LHCC-2020-007
- **AWAKE Run 2 status**, CERN-PBC-Notes-2022
- Esarey E. et al., Rev. Mod. Phys. 81, 1229 (2009) — LPA review
- Gonsalves A. et al., *PRL* 122, 084801 (2019) — BELLA 7.8 GeV
- **NHMFL** 32 T all-superconducting magnet, Markiewicz et al. 2019
- Bottura L. et al., Annu. Rev. Nucl. Part. Sci. 62, 109 (2012) — HTS for accelerators
- **MICE Collaboration**, *Nature* 578, 53 (2020) — muon ionization cooling
- Schwinger J., Phys. Rev. 75, 1912 (1949) — synchrotron radiation
- **NIST CODATA 2022**, https://physics.nist.gov/cuu/Constants/

---

*Audit wave: M. Authored by 박민우 <nerve011235@gmail.com>. No n=6 lattice anchoring of CERN engineering values (per LATTICE_POLICY §1.2). All operational parameters from CERN public TDRs/CDRs; physics constants from PDG 2024 + NIST CODATA 2022.*
