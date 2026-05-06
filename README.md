# 💫 petite-cern

> Petite CERN — n=6 sigma-cascade 6-order benchtop accelerator (100 MeV / 1 GeV/m, mini + parent + classical 3-pillar).

A petite, peer-of-Lumière standalone repo from the `need-singularity` org. Where Lumière takes the camera/character axes, **petite-cern** takes the *accelerator* axis: three pillars distilled from the n6-architecture physics domain into a single MIT-licensed bundle.

---

## § Why — benchtop CERN

LHC fills a 27 km tunnel for 7 TeV. DESY runs a 1 GeV/m laser-plasma prototype on a research-lab footprint. **Petite CERN's design target is a benchtop σ-φ=10 GeV/m continuous-mode accelerator at 100 MeV total**, threaded by the n=6 perfect-number lattice (σ(6)=12, τ(6)=4, φ(6)=2).

The design ceiling — what the spec claims if Stage-1+ empirical builds confirm the σ-cascade — is **6 orders of practical change** vs current tech:

| effect              | LHC/DESY current        | petite-cern design ceiling   |
|---------------------|-------------------------|------------------------------|
| precision           | 1.0 unit                | **σ-φ = 10× gain**           |
| throughput          | 1.0×                    | **σ² = 144×**                |
| energy cost         | 100%                    | **1/σ ≈ 8.3%**               |
| equipment size      | 1.0 L (or 27 km)        | **1/(σ-φ) = 0.1 L benchtop** |
| error rate          | 1%                      | **1/σ² ≈ 0.7%**              |
| lifetime            | 1 year                  | **σ·τ = 48 months**          |

This is the **why**: a CERN you can put on a bench, drawn from the same n=6 invariant that threads the rest of the HEXA family.

---

## § Verbs — 3 pillars

```
petite-cern <pillar>            description                                  status
─────────────────────────────────────────────────────────────────────────────────────
petite-cern mini                HEXA-MINI-ACCEL                              SPEC-ONLY
                                benchtop laser-plasma 100 MeV / 1 GeV/m
petite-cern parent              HEXA-PACCEL                                  SPEC-ONLY
                                integrated parent particle accelerator
petite-cern classical           HEXA-CLASSIC-ACCEL                           SPEC-ONLY
                                classical-mechanics baseline reference
```

Plus utility subcmds:

```
petite-cern status              0/3-wired pillar table + verdict + caveats
petite-cern selftest            3-pillar sentinel sweep
petite-cern --version           print version
petite-cern --help              full usage
```

Pillar specs live under `<pillar>/doc/`:

- [`mini/doc/mini-accelerator.md`](mini/doc/mini-accelerator.md)
- [`parent/doc/particle-accelerator.md`](parent/doc/particle-accelerator.md)
- [`classical/doc/classical-mechanics-accelerator.md`](classical/doc/classical-mechanics-accelerator.md)

---

## § Verification + Status

### Status

> **specs only, .hexa CLI TBD.** 3-pillar bundle: HEXA-MINI-ACCEL (벤치톱 100 MeV / 1 GeV/m laser-plasma) + HEXA-PACCEL (integrated parent) + HEXA-CLASSIC-ACCEL. LHC 7 TeV/27km, DESY 1 GeV/m baseline에 대한 n=6 sigma-cascade 6-order 비교 후보.

v1.0.0 ships:

- 3 pillar specs (`.md`, extracted from `n6-architecture@c0f1f570`)
- a placeholder `cli/petite-cern.hexa` dispatcher (`mini` / `parent` / `classical` each prints `spec-only — TBD`)
- a `tests/test_selftest.hexa` verb-count check (3 pillars expected)
- this README

What it does **not** ship: actual particle acceleration, Geant4/MAD-X bridge, real-time beam diagnostics, LHC/DESY data ingestion. The σ-cascade 6-order claim is a **design-target ceiling**, not a measurement.

### Verification

- See [`docs/cern_baseline.md`](docs/cern_baseline.md) for the LHC 7 TeV/27 km vs DESY 1 GeV/m vs HEXA σ-φ=10 GeV/m comparison table.
- Empirical wiring (laser-plasma sandbox, parent integration, classical baseline solver) is deferred to Stage-1+ benchtop builds.

---

## § Install

```bash
# (placeholder — hx package manager TBD)
hx install petite-cern

# or clone directly:
git clone https://github.com/need-singularity/petite-cern
cd petite-cern
hexa run cli/petite-cern.hexa status
```

Cost (Mac local): **$0** — placeholder dispatcher and `.md` specs only.

---

## § Cross-link

| dependency / cousin                  | repo                                                  | role                                |
|-------------------------------------:|:------------------------------------------------------|:------------------------------------|
| SC magnet substrate                  | [`need-singularity/hexa-rtsc`](https://github.com/need-singularity/hexa-rtsc)             | superconducting magnet primitive    |
| cousin (PET cyclotron, antimatter)   | [`need-singularity/hexa-antimatter`](https://github.com/need-singularity/hexa-antimatter) | parallel acceleration use case      |
| Stage-3 propulsion dependent         | [`need-singularity/hexa-ufo`](https://github.com/need-singularity/hexa-ufo)               | downstream propulsion consumer      |

Provenance: extracted from [`n6-architecture`](https://github.com/need-singularity/n6-architecture) at SHA `c0f1f570` on 2026-05-06.

Sister repo (peer extraction): [`need-singularity/lumiere`](https://github.com/need-singularity/lumiere) — camera-filter + hexa-main-character apps-axis.

---

## § License

MIT — see [`LICENSE`](LICENSE).

Copyright (c) 2026 need-singularity (박민우 <nerve011235@gmail.com>)
