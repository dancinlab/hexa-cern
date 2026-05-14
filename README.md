<p align="center">
  <img src="docs/logo.svg" width="140" alt="hexa-cern">
</p>

<h1 align="center">🔬 hexa-cern</h1>

<p align="center"><strong>HEXA-CERN family</strong> — particle physics · benchtop accelerator · σ-cascade · n=6 substrate</p>

<p align="center">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue"></a>
  <a href="https://doi.org/10.5281/zenodo.20102596"><img alt="DOI" src="https://zenodo.org/badge/DOI/10.5281/zenodo.20102596.svg"></a>
  <img alt="Spec" src="https://img.shields.io/badge/spec-v1.1.0--pre-success">
  <img alt="Pillars" src="https://img.shields.io/badge/pillars-3-informational">
  <img alt="Verify" src="https://img.shields.io/badge/verify-29%2F29%20PASS-brightgreen">
  <img alt="Checks" src="https://img.shields.io/badge/checks-353%2F353%20PASS-brightgreen">
  <img alt="Family" src="https://img.shields.io/badge/family-HEXA--rtsc%20·%20HEXA--antimatter%20·%20HEXA--ufo-blueviolet">
</p>

<p align="center">particle-physics · accelerator · laser-plasma · LWFA · σ-cascade · LHC · DESY · n=6 lattice · benchtop</p>

---

# 💫 hexa-cern

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.20102596.svg)](https://doi.org/10.5281/zenodo.20102596)

> hexa-cern — n=6 sigma-cascade 6-order benchtop accelerator (100 MeV / 1 GeV/m, mini + parent + classical 3-pillar).

A petite, peer-of-Lumière standalone repo from the `dancinlab` org. Where Lumière takes the camera/character axes, **hexa-cern** takes the *accelerator* axis: three pillars distilled from the canon physics domain into a single MIT-licensed bundle.

---

## Why — benchtop CERN

LHC fills a 27 km tunnel for 7 TeV. DESY runs a 1 GeV/m laser-plasma prototype on a research-lab footprint. **hexa-cern's design target is a benchtop σ-φ=10 GeV/m continuous-mode accelerator at 100 MeV total**, threaded by the n=6 perfect-number lattice (σ(6)=12, τ(6)=4, φ(6)=2).

The design ceiling — what the spec claims if Stage-1+ empirical builds confirm the σ-cascade — is **6 orders of practical change** vs current tech:

| effect              | LHC/DESY current        | hexa-cern design ceiling   |
|---------------------|-------------------------|------------------------------|
| precision           | 1.0 unit                | **σ-φ = 10× gain**           |
| throughput          | 1.0×                    | **σ² = 144×**                |
| energy cost         | 100%                    | **1/σ ≈ 8.3%**               |
| equipment size      | 1.0 L (or 27 km)        | **1/(σ-φ) = 0.1 L benchtop** |
| error rate          | 1%                      | **1/σ² ≈ 0.7%**              |
| lifetime            | 1 year                  | **σ·τ = 48 months**          |

This is the **why**: a CERN you can put on a bench, drawn from the same n=6 invariant that threads the rest of the HEXA family.

---

## Verification + Status

### Status

> **v1.1.0 RSC code-layer FINAL on `main` (2026-05-13): 29/29 verify
> scripts PASS · 353/353 row-level checks PASS · 4/4 tests PASS ·
> 100% bookkeeping closure across F-PCERN-1/2/3 (T1+T2+T3 locked) ·
> `__HEXA_CERN_RSC_SATURATED__ STOP` signal active.**
>
> 3-pillar bundle: HEXA-MINI-ACCEL (벤치톱 100 MeV / 1 GeV/m laser-plasma)
> + HEXA-PACCEL (integrated parent) + HEXA-CLASSIC-ACCEL. LHC 7 TeV/27km
> & DESY 1 GeV/m 기준선에 대한 n=6 σ-cascade 6-order 비교가 paper-only로
> 명시. **Empirical raw-data fit (Stage-1+ benchtop) UNVERIFIED until
> hardware lands.** 100% closure here is *bookkeeping* — closed-form
> algebra + closed-form numerics + archival paper-existence — not bench
> measurement. Real-limits authority is `LIMIT_BREAKTHROUGH.md` (CERN
> public TDR/CDR values for L1..L12; no n=6 lattice fit on CERN

v1.0.0 ships (frozen 2026-05-06):

- 3 pillar specs (`.md`, extracted from `canon@c0f1f570`)
- a `cli/hexa-cern.hexa` dispatcher with 3 pillar verbs + status / selftest

v1.1.0-pre adds (on `main`, 2026-05-07):

- `verify/` — 12 `.hexa` scripts auditing n=6 lattice + per-pillar derivations + numerical solvers
- `build/` — pandoc + xelatex Makefile that regenerates 3 pillar PDFs (clean)
- `tests/` — 4 `.hexa` test cases (+ `test_all.hexa` aggregator); 4/4 PASS
- `cli/hexa-cern.hexa verify [<sub>]` — 6-runner aggregator subcommand
- `hexa.toml` v1.1.0-pre [closure] block: `verify_pass: 6/6`, `tests: 4/4`

**Zero `.py` was added** — the runnable surface is 100% `.hexa`. This is
deliberate: hexa-cern is a hexa-family member, and the migration target
across `dancinlab` repos is .hexa-native tooling.

What it does **not** ship: actual particle acceleration, Geant4/MAD-X bridge, real-time beam diagnostics, LHC/DESY data ingestion. The σ-cascade 6-order claim is a **design-target ceiling**, not a measurement.

### Verification

The `verify/` surface (all `.hexa`) audits n=6 closure + per-pillar derivations.
**Current state (2026-05-13)**: **29/29 scripts PASS, 353/353 row-level
checks PASS — 100% bookkeeping closure.** F-PCERN-1/2/3 all at T1+T2+T3
locked. `__HEXA_CERN_RSC_SATURATED__ STOP` signal active (17/17 saturation
conditions met).

| script | check | result |
|---|---|---|
| `verify/lattice_check.hexa`           | σ(6)·φ(6) = n·τ(6) = J₂ = 24 across roadmap + 3 pillars | 23/23 PASS |
| `verify/cross_doc_audit.hexa`         | LHC / DESY / OEIS / BT cross-link + firmware HDL/MCU skeleton | 15/15 PASS |
| `verify/calc_wakefield.hexa`          | mini — E_peak = σ·(σ-φ) = 120 GV/m, a₀ = n = 6, R = 10 cm | 6/6 PASS |
| `verify/numerics_wakefield.hexa`      | mini — closed-form plasma numerics (n_e, L_d via math_pure) | 4/4 PASS |
| `verify/numerics_lwfa_solver.hexa`    | mini — Verlet LWFA ODE solver | 7/7 PASS |
| `verify/numerics_lwfa_relativistic.hexa` | mini — KDK leapfrog relativistic LWFA (γ → 200) | 9/9 PASS |
| `verify/numerics_beam_dynamics.hexa`  | mini — LWFA betatron + emittance preservation | 8/8 PASS |
| `verify/numerics_lwfa_scaling.hexa`   | mini — Lu/Esarey/TD scaling-law parity | 10/10 PASS |
| `verify/calc_sigma_cascade.hexa`      | parent — E_0..E_6 chain (10 MeV → 100 TeV) | 8/8 PASS |
| `verify/numerics_sigma_cascade.hexa`  | parent — relativistic γ progression numerics | 10/10 PASS |
| `verify/numerics_bending.hexa`        | parent — collider bending-radius R = E/(ecB), LHC/Tev/FCC | 8/8 PASS |
| `verify/calc_classical.hexa`          | classical — DOF = n = 6, phase-space dim = σ = 12 | 11/11 PASS |
| `verify/numerics_classical.hexa`      | classical — symplectic leapfrog (τ=4 quadrants) | 9/9 PASS |
| `verify/numerics_liouville.hexa`      | classical — Liouville volume preservation (det(J)=1) | 8/8 PASS |
| `verify/numerics_se3_partial.hexa`    | classical — 2-DOF symplectic (1→2→6-DOF) | 8/8 PASS |
| `verify/numerics_cross_pillar.hexa`   | cross-pillar consistency (mini ↔ parent ↔ classical) | 8/8 PASS |
| `verify/numerics_lhc_parity.hexa`     | F-PCERN-1 collider parity (LEP/Tevatron/LHC/FCC) | 10/10 PASS |
| `verify/numerics_lwfa_parity.hexa`    | F-PCERN-3 LWFA parity (BELLA/FACET/ATHENA/FLASHFwd) | 7/7 PASS |
| `verify/numerics_lattice_arithmetic.hexa` | n=6 lattice float arithmetic (math_pure stability) | 12/12 PASS |
| `verify/lint_numerics.hexa`           | meta-lint: every numerics_*.hexa follows conventions | 71/71 PASS |
| `verify/saturation_check.hexa`        | RSC saturation (sat-1 + sat-2 + sat-3 → STOP) | 17/17 PASS |
| `verify/empirical_lhc_api.hexa`       | F-PCERN-1 T3 — CERN Open Data API archival feed | 7/7 PASS |
| `verify/empirical_lwfa_hepdata.hexa`  | F-PCERN-3 T3 — Inspire-HEP LWFA paper milestones | 10/10 PASS |
| `verify/empirical_classical_arxiv.hexa` | F-PCERN-2 T3 — arXiv classical-mechanics milestones | 6/6 PASS |
| `verify/falsifier_check.hexa`         | F-PCERN-1/2/3 preregister + closure progress (100% bookkeeping) | 21/21 PASS |
| `firmware/sim/timing_chain.hexa`      | §A.6.1 step C — master clock + trigger sim | 10/10 PASS |
| `firmware/sim/dac_chain.hexa`         | §A.6.1 step C — 16-bit DAC chain | 10/10 PASS |
| `firmware/sim/adc_chain.hexa`         | §A.6.1 step C — ADC chain (BPM/diamond) | 9/9 PASS |
| `firmware/sim/control_loop.hexa`      | §A.6.1 step C — closed PID loop | 9/9 PASS |

Run them all with the unified CLI subcommand or the standalone orchestrator:

```bash
HEXA_LANG=$HOME/core/hexa-lang hexa-cern verify all       # 29/29 expected
HEXA_LANG=$HOME/core/hexa-lang hexa run verify/run_all.hexa  # same; orchestrator (.hexa)
```

> `HEXA_LANG` points the interpreter at the hexa-lang stdlib root that
> ships `self/runtime/math_pure`. Required when numerics scripts run
> outside the `hx` package shim.

Or build the 3 pillar PDFs:

```bash
make -C build check       # verify pandoc + xelatex + hexa available
make -C build all         # rebuild all 3 pillar PDFs into build/out/
```

Empirical wiring (laser-plasma sandbox, parent integration, classical
baseline solver) is deferred to Stage-1+ benchtop builds. See
[`docs/cern_baseline.md`](docs/cern_baseline.md) for the LHC 7 TeV/27 km
vs DESY 1 GeV/m vs HEXA σ-φ=10 GeV/m comparison table, and
[`docs/numerics_methodology.md`](docs/numerics_methodology.md) for how
the verify surface is structured (3-tier evidence ladder, F-PCERN
closure pct, math_pure conventions, recipe for adding new numerics).

---

## Install

```bash
# 1. Install hexa-lang (gives you `hexa` + `hx` package manager)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/dancinlab/hexa-lang/main/install.sh)"

# 2. Install hexa-cern
hx install hexa-cern
```

## Run

```bash
hexa-cern status          # 0/3-wired pillar table + verdict + caveats
hexa-cern selftest        # full 3-pillar sentinel sweep
hexa-cern verify          # n=6 invariant + per-pillar calculators (29/29)
hexa-cern --version       # show version
hexa-cern --help          # full usage

# orchestrator (.hexa, no CLI wrapper):
HEXA_LANG=$HOME/core/hexa-lang hexa run verify/run_all.hexa
```

---
## Repo layout

```
hexa-cern/
├── README.md                     ← this file
├── LICENSE                       ← MIT
├── hexa.toml                     ← package manifest (hx install hexa-cern)
├── install.hexa                  ← hx install hook (post-install selftest)
├── .roadmap.hexa_cern            ← cross-cutting state (lattice / cycles / falsifiers)
├── CHANGELOG.md                  ← release history
├── RELEASE_NOTES_v1.0.0.md       ← v1.0.0 cut notes
│
├── mini/doc/mini-accelerator.md                            ← pillar 1 (47 KB)
├── parent/doc/particle-accelerator.md                      ← pillar 2 (14 KB)
├── classical/doc/classical-mechanics-accelerator.md        ← pillar 3 (47 KB)
│
├── cli/
│   └── hexa-cern.hexa            ← CLI router (status/selftest/verify/mini/parent/classical)
├── verify/                       ← v1.1.0-pre — n=6 audit surface (.hexa)
│   ├── lattice_check.hexa            ← σ·φ = n·τ = J₂ = 24 closure   (23/23)
│   ├── cross_doc_audit.hexa          ← LHC/DESY/OEIS/BT cross-pillar (11/11)
│   ├── calc_wakefield.hexa           ← mini — laser-wakefield n=6   ( 6/ 6)
│   ├── numerics_wakefield.hexa       ← mini — closed-form n_e/L_d   ( 4/ 4)
│   ├── numerics_lwfa_parity.hexa     ← mini — vs DESY/SLAC LWFA refs ( 7/ 7)
│   ├── calc_sigma_cascade.hexa       ← parent — E_0..E_6 chain      ( 8/ 8)
│   ├── numerics_sigma_cascade.hexa   ← parent — relativistic γ      (10/10)
│   ├── numerics_lhc_parity.hexa      ← parent — vs LEP/Tev/LHC/FCC  (10/10)
│   ├── calc_classical.hexa           ← classical — Lagrange/Hamilton(11/11)
│   ├── numerics_classical.hexa       ← classical — symplectic leap. ( 9/ 9)
│   ├── numerics_cross_pillar.hexa    ← mini ↔ parent ↔ classical    ( 8/ 8)
│   └── falsifier_check.hexa          ← F-PCERN-1/2/3 + closure %    (11/11)
├── build/
│   ├── Makefile                  ← pandoc + xelatex 3-PDF rebuild
│   ├── header.tex                ← LaTeX include (CJK + monospace; soft-guarded)
│   └── out/*.pdf                 ← generated, .gitignore'd
├── tests/                        ← v1.1.0-pre — regression suite (.hexa)
│   ├── test_selftest.hexa
│   ├── test_lattice.hexa
│   ├── test_calculators.hexa
│   ├── test_cli_verify.hexa
│   └── test_all.hexa             ← runs everything above (4/4 PASS)
└── docs/cern_baseline.md         ← LHC vs DESY vs hexa-cern comparison
```

The `verify/ + build/ + tests/` triad is the canonical runnable
surface for hexa-cern: every audit + every PDF + every test is `.hexa`
end-to-end (no Python, no shell-only logic).

---

## Cross-link

| dependency / cousin                  | repo                                                  | role                                |
|-------------------------------------:|:------------------------------------------------------|:------------------------------------|
| SC magnet substrate                  | [`dancinlab/hexa-rtsc`](https://github.com/dancinlab/hexa-rtsc)             | superconducting magnet primitive    |
| cousin (PET cyclotron, antimatter)   | [`dancinlab/hexa-antimatter`](https://github.com/dancinlab/hexa-antimatter) | parallel acceleration use case      |
| Stage-3 propulsion dependent         | [`dancinlab/hexa-ufo`](https://github.com/dancinlab/hexa-ufo)               | downstream propulsion consumer      |

Provenance: extracted from [`canon`](https://github.com/dancinlab/echoes) at SHA `c0f1f570` on 2026-05-06.

Sister repo (peer extraction): [`dancinlab/lumiere`](https://github.com/dancinlab/lumiere) — camera-filter + hexa-main-character apps-axis.

---

## License

MIT — see [`LICENSE`](LICENSE).

Copyright (c) 2026 dancinlab (박민우 <nerve011235@gmail.com>)
