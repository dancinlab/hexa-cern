# Changelog — hexa-cern

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
