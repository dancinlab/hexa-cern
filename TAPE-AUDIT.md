# TAPE-AUDIT — hexa-cern

## A. Audit-class ledgers

- `state/markers/*.marker` — **2201 markers**, mostly `adc_chain_*` (sigma-cascade benchtop accelerator ADC chain measurements). `_FAILED.marker` grade encoding present. Direct **Class-T** fit — `state/adc_chain.tape` collapses 2k+ markers into one append-only stream with `@T adc_chain <- @S run=NNN => @R ok|err == @D adc_count`.
- `state/hexa_cern_cli.log` — CLI session log.
- `proofs/`, `papers/` carry formula text; no JSONL.

## B. Identity surface

`AGENTS.md` + `LATTICE_POLICY.md` + `IMPORTED_FROM_CANON.md` + `LIMIT_BREAKTHROUGH.md`. As the sigma-cascade benchtop accelerator parent, this repo's identity is the **measurement instrument** itself. Excellent fit for `hexa-cern/identity.tape` — instrument calibration + axis pin + verifier roster.

## C. Domain.md files

Only 9 root MD files (lighter than chip/matter). Most domain weight lives in subdirs (higgs/, plasma-physics/, quantum-{computer,computing,gravity-sensor,network,oracle}/, tabletop-blackhole/, tachyon/, particle-cosmology/). These verb-dirs are themselves domain-shaped — promotion to root `<DOMAIN>.md` + sibling `.tape` is a recurring opportunity.

## D. Per-run/per-event history

`adc_chain_*` markers are physical measurement runs — exactly **measurement-pilot tape** shape per theme spec. Each `@T adc_chain` measurement run lifts to a Class-T event with grade. `bt-1176-nuclear-reactor-kinetics` + `bt-1386-standard-model` in `breakthroughs/` are formula-promotion events → Class-P.

## E. Promotion candidates

- **n6 atoms** — Standard Model from n6 (`higgs/standard-model-from-n6.md`); l11-l15 quantum-nuclear mapping (`higgs/`). Both are high-confidence n=6 law promotions.
- **hxc binaries** — ADC waveforms + Higgs spectra (binary detector frames).
- **n12 cube cells** — particle × interaction × scale (Higgs/quark/lepton × strong/weak/EM/grav × benchtop/lab/LHC).

## Verdict

**HEAVY** — 2201 measurement markers + canonical formula breakthroughs (Standard Model, nuclear kinetics). Top atom-promotion repo after hexa-physics; clearest measurement-pilot `.tape` case.
