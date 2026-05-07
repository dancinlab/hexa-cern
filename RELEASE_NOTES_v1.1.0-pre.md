# hexa-cern v1.1.0-pre — `.hexa` runnable surface 💫

**Cut:** 2026-05-07 (on `main`, untagged — release tag lands when v1.1.0 numerical solver completes the chunk)
**Provenance:** `hexa-cern@v1.0.0` (`78f5f72`) → `hexa-cern@main`
**License:** MIT
**Verdict:** SPECS_ONLY (3/3 pillar specs unchanged — runnable surface AUDITS them, doesn't replace them)

---

## What's new

The v1.0.0 cut shipped 3 pillar specs + a placeholder CLI. v1.1.0-pre adds the **runnable surface** that audits those specs end-to-end without modifying them. Every new artifact is `.hexa` (zero `.py` added).

### `verify/` — 6 atlas-style audits

| script                                | check                                                        | result        |
|:--------------------------------------|:-------------------------------------------------------------|:--------------|
| `verify/lattice_check.hexa`           | σ(6)·φ(6) = n·τ(6) = J₂ = 24 closure across roadmap + 3 pillars | **23/23 PASS** |
| `verify/cross_doc_audit.hexa`         | LHC / DESY / OEIS / BT cross-pillar consistency             | **11/11 PASS** |
| `verify/calc_wakefield.hexa`          | mini — E_peak = σ·(σ-φ) = 120 GV/m, a₀ = n = 6, R = σ-φ = 10 cm | **6/6 PASS** |
| `verify/calc_sigma_cascade.hexa`      | parent — E₀..E₆ chain (10 MeV → 100 TeV, σ³ = 1728 envelope) | **8/8 PASS** |
| `verify/calc_classical.hexa`          | classical — DOF = n = 6, dim(q,p) = σ = 12, conserved = sopfr+φ = 7 | **11/11 PASS** |
| `verify/falsifier_check.hexa`         | F-PCERN-1/2/3 preregister checklist                          | **3/3 registered** (UNVERIFIED v1.0) |

Run them all:
```bash
hexa-cern verify all      # 6/6 PASS expected, exit 0
```

### `cli/hexa-cern.hexa` — pillar verbs wired

The 3 pillar verbs (`mini` / `parent` / `classical`) now emit a one-shot
n=6 derivation table on top of the existing spec presence check:

```
$ hexa-cern mini
hexa-cern mini — HEXA-MINI-ACCEL (benchtop laser-plasma 100 MeV / 1 GeV/m)
  status: spec frozen v1.0 · .hexa calc lands v1.1.0-pre · empirical Stage-1+ TBD
  spec:   /Users/ghost/.hx/packages/hexa-cern/mini/doc/mini-accelerator.md

  n=6 derivation (verify with: hexa-cern verify wakefield)
    E_peak    = σ·(σ-φ) = 12·10  = 120 GV/m   (peak wakefield)
    a_0       = n               = 6           (normalized vec. pot.)
    R_chamber = σ-φ             = 10 cm       (benchtop radius)
    target    = 100 MeV reachable in ~ 8 cm of L_acc
```

Plus a new top-level `verify [<sub>]` subcommand routes the 6 verifiers.
Main routing fix: only the FIRST positional token triggers global
`--help`, so sub-positioned flags reach their subcommand's own help.

### `build/` — pandoc + xelatex 3-PDF rebuild

`make -C build all` regenerates the 3 pillar PDFs (mini 145 K + parent
94 K + classical 143 K) into `build/out/` (gitignored). `header.tex`
soft-guards optional packages (xeCJK, titlesec) so build hosts without
those installed get a clean ASCII-only PDF rather than an abort.

### `tests/` — 4 `.hexa` regressions

- `tests/test_lattice.hexa` — exit 0 + sentinel from `verify/lattice_check.hexa`
- `tests/test_calculators.hexa` — all 5 calc/audit/falsifier scripts close
- `tests/test_cli_verify.hexa` — `hexa-cern verify all` aggregates 6/6 PASS
- `tests/test_all.hexa` — top-level aggregator (4/4 PASS)

```bash
hexa run tests/test_all.hexa   # 4/4 PASS, exit 0
```

---

## Honest scope (raw#10 C3)

The verify surface confirms **algebraic + cross-doc** consistency only.
Empirical falsifiers F-PCERN-1/2/3 remain UNVERIFIED v1.0 — there is
no Stage-1+ benchtop build, no laser-plasma simulation parity, and no
LHC/DESY data ingestion. v1.1.0-pre intentionally lands the audit
surface BEFORE the numerical solver so that future Stage-1 results
have a regression target.

| component                          | v1.0.0 | v1.1.0-pre | v1.1.0 (target) |
|:-----------------------------------|:------:|:----------:|:---------------:|
| 3 pillar specs                     |   ✅   |     ✅     |        ✅       |
| placeholder CLI dispatcher         |   ✅   |     ✅     |        ✅       |
| n=6 algebraic audit                |   ✗    |     ✅     |        ✅       |
| cross-doc consistency check        |   ✗    |     ✅     |        ✅       |
| 3-PDF rebuild                      |   ✗    |     ✅     |        ✅       |
| pillar n=6 derivation in CLI       |   ✗    |     ✅     |        ✅       |
| **numerical wakefield solver**     |   ✗    |     ✗      |        🎯       |
| **σ-cascade integration**          |   ✗    |     ✗      |        🎯       |
| **classical Hamiltonian numerics** |   ✗    |     ✗      |        🎯       |
| **F-PCERN-1/2/3 closure**          |   ✗    |     ✗      |  empirical TBD  |

---

## Diff stats

```
2 commits, 17 + 2 files changed, 1888 insertions(+)

0a74c21 v1.1.0-pre: .hexa runnable surface (verify + build + tests + cli verify)
        17 files, +1833 / -20
5aded4e cli: wire mini/parent/classical verbs to emit n=6 derivation summaries
         2 files, +55 / -6
```

---

## What's next (toward v1.1.0)

1. First numerical solver stub (`verify/numerics_wakefield.hexa`) — uses
   `math_pure` to compute n_e from E_peak via plasma frequency, then
   dephasing length L_d. Pure algebra check: closed-form ratio against
   the n=6 derivation.
2. `mini/` empirical sandbox (laser pulse → electron energy parity vs
   DESY/SLAC reference data — closes F-PCERN-3 if it converges).
3. `parent/` σ-cascade chain integrator (closes F-PCERN-1).
4. `classical/` Lagrangian/Hamiltonian τ=4 phase numerics (closes F-PCERN-2).

— need-singularity (박민우 <nerve011235@gmail.com>)
