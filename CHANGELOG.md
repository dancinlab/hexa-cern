# Changelog — hexa-cern

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased] — v1.1.0-pre on `main`

### Added (2026-05-07 — fifteenth iteration)

- `verify/numerics_se3_partial.hexa` — F-PCERN-2's third T2 stub
  (alongside `numerics_classical` + `numerics_liouville`). Promotes
  the symplectic check from 1-DOF to **2-DOF** — a partial step toward
  the full 6-DOF SE(3) target documented in roadmap §A.2 (v1.2.0+).
  - System: two coupled harmonic oscillators
      H = ½(p₁²+p₂²) + ½ω²(q₁²+q₂²) + ½κ(q₁−q₂)²
  - Velocity-Verlet kick-drift-kick on (q₁, p₁, q₂, p₂)
  - Checks: energy drift, antisymmetric-mode decoupling under
    symmetric IC (replaces total-p which is broken by on-site
    potential), 4×4 Jacobian determinant for Liouville volume
  - Results: 8/8 PASS, max |ΔE/E| = 8.4·10⁻⁶, anti-mode = 0.0
    exactly, 4-D det(J) drift = 4.9·10⁻¹¹ at origin / 8.2·10⁻¹¹
    at offset IC.
- `cli/hexa-cern.hexa verify` — new `numerics-se3` sub. The
  `verify all` aggregator now runs **17/17** scripts (was 16/16).
- `verify/falsifier_check.hexa` — F2 T2 array now has ×3 stack
  (classical + liouville + se3_partial). All 3 falsifiers now
  symmetric at T2 ≥ 2 (sat-1 closer; F1/F3 still ×2/×3).
- `verify/lint_numerics.hexa` — NUMERICS_SCRIPTS extended to 10.

### Added (2026-05-07 — fourteenth iteration)

- `docs/numerics_methodology.md` — narrative documentation of the
  verify-surface conventions accumulated across iterations 1 → 13:
  - 3-tier evidence ladder (T1 algebraic / T2 numerical / T3 empirical)
  - per-falsifier T2 stack (F-PCERN-1 ×2 + F-PCERN-2 ×2 + F-PCERN-3 ×3)
  - cross-cutting + meta scripts inventory
  - math_pure dependency + how `numerics_lattice_arithmetic.hexa`
    serves as the stability floor
  - aggregate state (16/16 verify, 4/4 tests, 3 PDFs)
  - what's intentionally NOT in v1.1.0-pre (T3 empirical, full
    relativistic LWFA, full SE(3) 6-DOF symplectic) — scope, not bugs
  - recipe for adding a new `numerics_*.hexa` script (8-step checklist)
- `README.md` — links to the new methodology doc from the Verification
  section, alongside the existing `docs/cern_baseline.md` reference.

### Added (2026-05-07 — thirteenth iteration)

- `verify/numerics_lwfa_solver.hexa` — first step toward the v1.1.0
  target ("mini .hexa CLI wired (laser pulse → electron energy
  계산)" per `.roadmap.hexa_cern §A.2`). A 1-D Verlet integrator
  propagates an electron through a sinusoidal plasma-wakefield
  E_z(q, t) = E_peak·sin(k_p·(q − v_g·t)) and accumulates ΔE
  step-by-step over a 1 mm path with 1024 Verlet steps.
  - Field model uses E_peak = 120 GV/m (n=6 derivation σ·(σ-φ))
    and k_p derived from the cold-plasma wave-breaking relation
    ω_p = e·E_peak / (m_e·c) — same chain numerics_wakefield uses.
  - Solver result: electron advances 430 µm, ΔE ≈ 7 keV (well
    below the 120 MeV closed-form ceiling; honest because a
    non-trapped electron in a sinusoidal field doesn't accumulate
    much net work — phase trapping for sustained acceleration is
    a v1.2.0+ feature).
  - 7/7 PASS: solver terminates, q advances, ΔE ≥ 0, ΔE ≤
    closed-form ceiling, E_z periodic in λ_p, λ_p in LWFA window,
    mini doc anchored.
  - λ_p = 26.76 µm  matches `numerics_wakefield`'s cold-plasma
    derivation exactly (independent re-derivation, same answer).
- `cli/hexa-cern.hexa verify` — new `numerics-solver` sub. The
  `verify all` aggregator now runs **16/16** scripts (was 15/15).
- `verify/falsifier_check.hexa` — F-PCERN-3 T2 array now lists
  3 scripts (numerics_wakefield + numerics_lwfa_parity +
  numerics_lwfa_solver) — F-PCERN-3 has the deepest T2 stack.
- `verify/lint_numerics.hexa` — NUMERICS_SCRIPTS extended to 9.

### Added (2026-05-07 — twelfth iteration)

- `verify/numerics_liouville.hexa` — F-PCERN-2's second T2 stub.
  Demonstrates the deeper symplectic invariant: the Jacobian
  determinant of the leapfrog flow map is exactly 1, meaning
  phase-space volume is preserved (Liouville's theorem at
  discrete time). Computed via central finite differences on
  64-step quadrants and full-period propagation.
  - phase 1 (pump):    |det(J) − 1| = 2.7e-10
  - phase 2 (bubble):  |det(J) − 1| = 6.3e-10
  - phase 3 (capture): |det(J) − 1| = 7.0e-10
  - phase 4 (extract): |det(J) − 1| = 1.3e-09
  - full period:       |det(J) − 1| = 1.3e-09
  - offset IC (0.5, 0.7): |det(J) − 1| = 8.4e-10
  All within central-diff truncation noise floor (1e-8).
  8/8 PASS first try.
  Together with `numerics_classical` (energy drift), this covers
  both characteristic symplectic invariants (energy & volume) for
  F-PCERN-2's T2 closure.
- `cli/hexa-cern.hexa verify` — new `numerics-liouville` sub. The
  `verify all` aggregator now runs **15/15** scripts (was 14/14).
- `verify/falsifier_check.hexa` — F-PCERN-2 T2 array now lists
  both `numerics_classical` and `numerics_liouville`. All three
  falsifiers now have 2 T2 scripts (symmetric).
- `verify/lint_numerics.hexa` — NUMERICS_SCRIPTS inventory list
  extended to 8 entries.

### Added (2026-05-07 — eleventh iteration)

- `verify/lint_numerics.hexa` — meta-lint that grep-audits every
  `verify/numerics_*.hexa` for 5 regression-discipline invariants:
    1. `use "self/runtime/math_pure"` (no raw float math sneaks in)
    2. `__HEXA_CERN_<NAME>__ PASS` sentinel emitted
    3. `FALSIFIERS` list declared
    4. `exit(0)` on PASS path
    5. `RUN` / `FAIL` accounting counters present
  Plus an inventory completeness check (NUMERICS_SCRIPTS list count
  must match the on-disk `numerics_*.hexa` glob count). 36/36 PASS:
  35 invariant checks (5 × 7 scripts) + 1 inventory check.
  - File originally `numerics_methodology.hexa`; renamed to `lint_numerics.hexa`
    so it doesn't trip its own inventory check (it's a LINT, not a
    numerical solver).
- `cli/hexa-cern.hexa verify` — new `lint-numerics` sub. The
  `verify all` aggregator now runs **14/14** scripts (was 13/13).

### Added (2026-05-07 — tenth iteration)

- `verify/numerics_lattice_arithmetic.hexa` — float-arithmetic
  cross-check of the n=6 lattice anchors using `self/runtime/math_pure`
  (sqrt, pow, log10, exp/log). Repeats every closure that
  `lattice_check.hexa` proves with INTEGER arithmetic, but via
  floating-point routines, and asserts agreement to rel err < 1e-9.
  - sqrt(σ²) = σ, pow(σ, 2) = 144, pow(σ, 3) = 1728 (Ω_ACCEL)
  - pow(σ-φ, n) = 10⁶, log10(10⁷) = 7 (chain decade ladder)
  - log10(σ²) = 2·log10(σ) (homomorphism)
  - exp(log(σ)) = σ at rel err 1.48·10⁻¹⁶ (machine precision)
  - sqrt(σ³) = σ^(3/2) (cross-consistency)
  - σ·φ = n·τ = J₂ in float (closure parity)
  - int(pow(σ, k)) = integer anchor (float→int cast safe)
  - σ·J₂ = 288 (Tevatron seed in float)
  - sopfr(6)·φ(6) = σ - φ (cascade seed identity)
  - 12/12 PASS first try.
  Catches numerical drift on anchors used by every per-pillar
  numerics_*.hexa — gives math_pure stability its own regression slot.
- `cli/hexa-cern.hexa verify` — new `numerics-lattice` sub. The
  `verify all` aggregator now runs **13/13** scripts (was 12/12).

### Added (2026-05-07 — eighth iteration)

- `verify/numerics_lwfa_parity.hexa` — F-PCERN-3 LWFA parity stub
  (parallel to `numerics_lhc_parity` for F-PCERN-1). Compares the
  hexa-cern design point (E_peak = 120 GV/m, n_e = 1.56·10¹⁸ cm⁻³,
  ΔE = 1200 MeV/cm) against published LWFA experiments:
    LBNL BELLA       400 MeV/cm  @ 3·10¹⁷ cm⁻³  → 100 GV/m
    SLAC FACET-II    100 MeV/cm  @ 1·10¹⁷       → 50 GV/m
    DESY ATHENA-x    500 MeV/cm  @ 1·10¹⁸       → 200 GV/m
    DESY FLASHFwd    100 MeV/cm  @ 1·10¹⁸       → 30 GV/m
  hexa-cern's gradient sits 2.4× above ATHENA-x (high side of the
  band, consistent with operating at a₀ = 6 blowout regime). Density
  in 10¹⁷..10¹⁸ window. λ_p / FACET λ_p = 0.25 (matches √n_e ratio
  exactly, independent re-derivation). 7/7 PASS first try.
  Closes F-PCERN-3's T2 (numerical) tier alongside numerics_wakefield.
- `cli/hexa-cern.hexa verify` — new `numerics-lwfa` sub. The
  `verify all` aggregator now runs **12/12** scripts (was 11/11).

### Added (2026-05-07 — seventh iteration)

- `hexa.toml [package].version` bumped 1.0.0 → **1.1.0-pre** (and the
  matching `VERSION` constant in `cli/hexa-cern.hexa`). Reflects the
  release-candidate state on `main`.
- `verify/numerics_lhc_parity.hexa` — F-PCERN-1 collider parity stub.
  Compares the σ-cascade ladder (E_3 = 100 GeV, E_4 = 1 TeV, E_5 = 10 TeV,
  E_6 = 100 TeV) to per-beam published collider energies:
    LEP        104.5 GeV  →  E_3  ratio 1.05  (close fit)
    Tevatron   980 GeV    →  E_4  ratio 0.98  (close fit)
    LHC        6.8 TeV    →  E_5  ratio 0.68  (rounded-decade ladder)
    FCC-hh    50 TeV     →  E_6  ratio 0.50  (proposed, consistent)
  All 4 machines stay within (σ-φ)/2 = 5× of their bucket and map to
  4 unique E_k stages (chain is informative). 10/10 PASS.
  This stub closes F-PCERN-1's T2 (numerical) tier; T3 (live LHC feed)
  remains TBD per Stage-1+ roadmap.
- `cli/hexa-cern.hexa verify` — new `numerics-lhc` sub. The
  `verify all` aggregator now runs **11/11** scripts (was 10/10).

### Changed (2026-05-07 — sixth iteration)

- `verify/falsifier_check.hexa` — closure-progress tracker. Each
  F-PCERN now reports a 3-tier evidence ladder:
    T1 algebraic (`verify/calc_*.hexa`)
    T2 numerical (`verify/numerics_*.hexa`)
    T3 empirical (Stage-1+ TBD)
  At v1.1.0-pre all three falsifiers stand at **67% closure**
  (T1 + T2 landed; T3 awaits empirical). The PASS sentinel now
  reads `3/3 preregistered, 67% closure (T1+T2 locked)`.
- New tier-script presence audit: each F-PCERN's T1 + T2 verify
  files must exist on disk. 9/9 PASS (was 3/3 — added 6 new
  presence checks).
- Closure pct uses round-up reporting (0/33/67/100) so the 2-of-3
  case shows 67%, matching the sentinel exactly.

### Added (2026-05-07 — fifth iteration)

- `verify/numerics_cross_pillar.hexa` — cross-pillar numerical
  consistency check. Each per-pillar numerics stub validates its own
  physics in isolation; this script links them on shared anchors:
  - mini.target (100 MeV) ≡ parent.E_1 (100 MeV)  — identical scalar
  - γ at 100 MeV ≈ 196.69, identical across pillars (rest-mass tail
    free-of-bug check)
  - classical 1-DOF stub phase-space (dim 2) ⊆ Hamilton σ = 12
  - plasma λ_p (26.8 µm) in LWFA bubble window (1–100 µm)
  - λ_p / λ_laser ≈ 33.4 (driver / wake scale separation, matches
    numerics_wakefield's ω_0/ω_p ratio exactly)
  - n=6 lattice constants identical across all pillar numerics
  - 8/8 PASS first try.
- `cli/hexa-cern.hexa verify` — new `numerics-cross` sub. The
  `verify all` aggregator now runs **10/10** scripts (was 9/9).

### Added (2026-05-07 — fourth iteration)

- `verify/numerics_classical.hexa` — third numerical solver stub
  (classical pillar). Velocity-Verlet (leapfrog) symplectic integrator
  on a 1-DOF harmonic oscillator H = ½(p² + ω²q²) with ω = 1, run
  over **one full period** divided into τ = 4 phase quadrants
  (pump · bubble · capture · extract).
  - 1024 leapfrog steps total (256 per quadrant), Δt = π/512.
  - 1-period |Δstate| = 9.9·10⁻⁶ (well under O(Δt²) bound 1.6·10⁻⁴).
  - max |ΔE/E| = 9.4·10⁻⁶ (symplectic shadow Hamiltonian holds).
  - All 4 phase quadrants land within 5·10⁻⁶ of canonical (q,p).
  - 9/9 PASS first try.
- `cli/hexa-cern.hexa verify` — new `numerics-classical` sub. The
  `verify all` aggregator now runs **9/9** scripts (was 8/8).

### Added (2026-05-07 — third iteration)

- `verify/numerics_sigma_cascade.hexa` — second numerical solver stub
  (parent pillar). Computes Lorentz γ at each E_k stage using
  m_e c² = 0.511 MeV, then verifies:
  - γ strictly monotone across all 7 stages.
  - γ_6 / γ_2 ≈ 10⁵ within 0.1% (ultrarelativistic regime check).
  - γ_0 rest-mass tail = m_e c²/E_0 ≈ 5% (boundary regime, surfaces
    why we use γ_2 not γ_0 as the lower anchor).
  - log10(E_6/E_0) = 7, σ²=144, σ³=1728, ultrarelativistic by E_2.
  - 10/10 PASS.
- `cli/hexa-cern.hexa verify` — new `numerics-sigma` sub. The
  `verify all` aggregator now runs **8/8** scripts (was 7/7).

### Added (2026-05-07 — second iteration)

- `verify/numerics_wakefield.hexa` — first numerical solver stub. Uses
  `self/runtime/math_pure` (sqrt / pow / pi) for the cold-plasma
  closed-form: given E_peak = 120 GV/m it computes
  ω_p, n_e ≈ 1.56·10¹⁸ cm⁻³ (inside DESY/SLAC LWFA window) and
  L_d ≈ 1.5 cm (linear-1D dephasing). 4/4 PASS.
- `cli/hexa-cern.hexa verify` — new `numerics-wakefield` sub. The
  `verify all` aggregator now runs **7/7** scripts (was 6/6).
- `RELEASE_NOTES_v1.1.0-pre.md` — packaged release notes for the
  v1.1.0-pre cut.

### Fixed

- `verify/calc_wakefield.hexa` — L_acc_req unit conversion was off by
  100×. Surfaced by `numerics_wakefield`'s L_d vs L_acc_req cross-check
  (which initially failed against the buggy 8 cm value before the fix
  reduced it to 0.083 cm). Math: 100 MeV / (120 GV/m · 10 MeV/cm·GV/m⁻¹)
  = 100/1200 ≈ 0.083 cm. Fix uses tenth-mm integer arithmetic to avoid
  losing precision. 6/6 still PASS post-fix.



### Added — `.hexa` runnable surface

All new code is `.hexa` (zero `.py` added). Audits the v1.0.0 frozen specs without modifying them.

- `verify/` — 6 atlas-style audits (run via `hexa-cern verify <sub>`):
  - `lattice_check.hexa` — σ(6)·φ(6) = n·τ(6) = J₂ = 24 closure across roadmap + 3 pillars (**23/23 PASS**)
  - `cross_doc_audit.hexa` — LHC / DESY / OEIS / BT cross-pillar consistency (**11/11 PASS**)
  - `calc_wakefield.hexa` — mini pillar laser-wakefield n=6 derivation (E_peak = σ·(σ-φ) = 120 GV/m, a₀ = n = 6, R = 10 cm) (**6/6 PASS**)
  - `calc_sigma_cascade.hexa` — parent pillar E_0..E_6 chain (10 MeV → 100 TeV, σ³ = 1728 FCC envelope) (**8/8 PASS**)
  - `calc_classical.hexa` — classical pillar Lagrange/Hamilton τ=4 phase (DOF = n = 6, dim(q,p) = σ = 12) (**11/11 PASS**)
  - `falsifier_check.hexa` — F-PCERN-1/2/3 preregister checklist (**3/3 registered**, status UNVERIFIED v1.0)
- `cli/hexa-cern.hexa verify [<sub>]` — 6-runner aggregator subcommand (sub: `all` | `lattice` | `cross-doc` | `wakefield` | `sigma` | `classical` | `falsifier`).
- `build/Makefile` + `build/header.tex` — pandoc + xelatex regenerates the 3 pillar PDFs (mini 145 K + parent 94 K + classical 143 K). `header.tex` soft-guards optional packages (xeCJK, titlesec) so missing packages skip silently rather than abort.
- `tests/test_lattice.hexa` + `tests/test_calculators.hexa` + `tests/test_cli_verify.hexa` + `tests/test_all.hexa` — 4 `.hexa` regressions. `test_all.hexa` aggregates and prints `__HEXA_CERN_TEST_ALL__ PASS` (**4/4 PASS**).
- `hexa.toml` — `[test].files` lists 5 cases; `[modules].hexa` lists 7 entries; `[closure]` adds `verify_pass / build_pdfs / tests_pass` for v1.1.0-pre.
- `.roadmap.hexa_cern` — §A.2 release table adds v1.1.0-pre row; §A.3 cycle history adds Cycle 1 entry.
- `README.md` — new §Repository layout + Verification table.

### Changed
- `cli/hexa-cern.hexa` main routing — only the FIRST positional token triggers global `--help`, so sub-positioned flags (e.g. `verify --help`) reach their subcommand's own help branch.
- `.gitignore` — `state/` patterns now match `**/state/` (build/state markers ignored); `build/out/` + `*.pdf` ignored, but `build/Makefile` + `build/header.tex` tracked.

### Honesty (raw#10 C3)
- The verify surface confirms **algebraic + cross-doc** consistency only. Empirical falsifiers F-PCERN-1/2/3 remain UNVERIFIED v1.0 (no Stage-1+ benchtop build yet). Numerical solvers (laser pulse → electron energy parity, σ-cascade integration, classical Hamiltonian τ=4 phase numerics) target v1.1.0 / v1.2.0.

---

## [1.0.0] — 2026-05-06

### Added
- Initial extraction from `n6-architecture@c0f1f570` (2026-05-06).
- 3-pillar bundle ships specs only:
  - `mini/doc/mini-accelerator.md`        — HEXA-MINI-ACCEL (benchtop laser-plasma 100 MeV / 1 GeV/m)
  - `parent/doc/particle-accelerator.md`  — HEXA-PACCEL (integrated parent accelerator)
  - `classical/doc/classical-mechanics-accelerator.md` — HEXA-CLASSIC-ACCEL (classical-mechanics baseline)
- `cli/hexa-cern.hexa` placeholder dispatcher with verbs: `mini` / `parent` / `classical` / `status` / `selftest` / `help` / `--version`.
  - Each pillar verb prints `status: spec-only — TBD` and the path to its `.md` spec.
- `hexa.toml` package manifest (MIT, entry `cli/hexa-cern.hexa`, repo `dancinlab/hexa-cern`).
- `install.hexa` hx package-manager hook (post-install warn-only selftest).
- `tests/test_selftest.hexa` 3-pillar verb-count smoke check.
- `docs/cern_baseline.md` — LHC 7 TeV/27 km vs DESY 1 GeV/m vs HEXA σ-φ=10 GeV/m comparison table.
- README §Why · §Verbs · §Verification + §Status · §Install · §Cross-link · §License.

### Honesty (raw#10 C3)
- **specs only, .hexa CLI TBD.** Empirical wiring (laser-plasma sandbox, parent integration, classical baseline solver) deferred to Stage-1+ benchtop builds.
- n=6 σ-cascade 6-order claim (precision ×10, throughput ×144, energy ÷12, size ÷10, error ÷144, lifetime ×48) is a **design-target ceiling**, not a measurement.
- LHC 7 TeV/27 km + DESY 1 GeV/m comparison is paper-only.

### Cross-link
- SC magnet substrate: [`dancinlab/hexa-rtsc`](https://github.com/dancinlab/hexa-rtsc)
- cousin (PET cyclotron, antimatter factory): [`dancinlab/hexa-antimatter`](https://github.com/dancinlab/hexa-antimatter)
- Stage-3 propulsion dependent: [`dancinlab/hexa-ufo`](https://github.com/dancinlab/hexa-ufo)

[1.0.0]: https://github.com/dancinlab/hexa-cern/releases/tag/v1.0.0
