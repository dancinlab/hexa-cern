# Changelog — hexa-cern

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased] — v1.1.0-pre on `main`

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
- `hexa.toml` package manifest (MIT, entry `cli/hexa-cern.hexa`, repo `need-singularity/hexa-cern`).
- `install.hexa` hx package-manager hook (post-install warn-only selftest).
- `tests/test_selftest.hexa` 3-pillar verb-count smoke check.
- `docs/cern_baseline.md` — LHC 7 TeV/27 km vs DESY 1 GeV/m vs HEXA σ-φ=10 GeV/m comparison table.
- README §Why · §Verbs · §Verification + §Status · §Install · §Cross-link · §License.

### Honesty (raw#10 C3)
- **specs only, .hexa CLI TBD.** Empirical wiring (laser-plasma sandbox, parent integration, classical baseline solver) deferred to Stage-1+ benchtop builds.
- n=6 σ-cascade 6-order claim (precision ×10, throughput ×144, energy ÷12, size ÷10, error ÷144, lifetime ×48) is a **design-target ceiling**, not a measurement.
- LHC 7 TeV/27 km + DESY 1 GeV/m comparison is paper-only.

### Cross-link
- SC magnet substrate: [`need-singularity/hexa-rtsc`](https://github.com/need-singularity/hexa-rtsc)
- cousin (PET cyclotron, antimatter factory): [`need-singularity/hexa-antimatter`](https://github.com/need-singularity/hexa-antimatter)
- Stage-3 propulsion dependent: [`need-singularity/hexa-ufo`](https://github.com/need-singularity/hexa-ufo)

[1.0.0]: https://github.com/need-singularity/hexa-cern/releases/tag/v1.0.0
