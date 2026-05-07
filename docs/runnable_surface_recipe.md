# runnable surface recipe — hexa-* 프로젝트 공통

> hexa-cern 에서 13 iteration 동안 검증된 패턴. 다른 hexa-* 프로젝트
> (`hexa-bio`, `hexa-rtsc`, `hexa-fusion`, `hexa-antimatter`,
> `hexa-ufo`, `hexa-chip`, `hexa-codex` …) 에 그대로 적용 가능.

본 문서가 정의하는 작업의 정식 명칭:

**Runnable Surface Construction** (RSC) — 또는 **봉쇄 심화 (closure-depth accumulation)**

설명:
- 새 verify/build/tests 코드를 하나씩 추가하며,
- 사전 등록된 falsifier 의 closure 를 T1 (algebraic) → T2 (numerical) →
  T3 (empirical) 사다리로 깊게 만든다.
- T3 (실험 하드웨어) 는 미루고, T2 를 **수평으로** 누적해 닫힌-form 의
  교차 검증 깊이를 쌓는다.

---

## §0 전제 조건 (대상 프로젝트가 갖춰야 하는 것)

대상 프로젝트가 **이미 가지고 있어야** 본 레시피가 즉시 적용 가능:

1. `hexa.toml` ([package].entry, [test], [closure] 섹션 포함)
2. `.roadmap.<project>` (§A.1 invariant lattice + §A.4 falsifier preregister)
3. `cli/<project>.hexa` 플레이스홀더 dispatcher (3+ pillar verb)
4. `<pillar>/doc/*.md` spec 문서 (n=6 lattice 명시)
5. `tests/test_selftest.hexa` 정도 한 개

이 5개가 안 갖춰져 있으면 먼저 그것부터 만들어야 한다.
hexa-sscb 와 hexa-cern 은 둘 다 갖춰져 있고, 본 레시피는 이런
"specs-only v1.0.0" 상태에서 시작한다고 가정한다.

---

## §1 산출물 — verify/ 디렉토리 16-script 표준 인벤토리

각 hexa-* 프로젝트는 본 16-script 골격을 자신의 도메인에 맞게 채운다.

### Algebraic tier (T1) — 4 scripts

| # | 파일                          | 역할 |
|:--|:-----------------------------|:-----|
| 1 | `lattice_check.hexa`         | n=6 closure (σ·φ = n·τ = J₂ = 24) across roadmap + 모든 pillar |
| 2 | `cross_doc_audit.hexa`       | pillar 간 anchor (baseline collider/실험명, OEIS, BT-link) 일치 |
| 3 | `calc_<pillar1>.hexa`        | pillar 1 의 n=6 derivation (정수 산술) |
| 4 | `calc_<pillar2>.hexa`        | pillar 2 의 n=6 derivation |
|   | …                            | pillar 갯수만큼 calc_*.hexa |

### Numerical tier (T2) — 9 scripts (대표)

| # | 파일                                  | 역할 |
|:--|:-------------------------------------|:-----|
| 5 | `numerics_<pillar1>.hexa`            | pillar 1 closed-form 을 math_pure 로 재유도 |
| 6 | `numerics_<pillar1>_parity.hexa`     | pillar 1 vs published 실험 비교 (e.g., 콜라이더 4종, LWFA 4종) |
| 7 | `numerics_<pillar1>_solver.hexa`     | pillar 1 의 미니 ODE solver (Verlet/leapfrog) |
| 8 | `numerics_<pillar2>.hexa`            | pillar 2 numerics |
| 9 | `numerics_<pillar2>_parity.hexa`     | pillar 2 published-ref parity |
|10 | `numerics_<pillar3>.hexa`            | pillar 3 numerics (e.g., symplectic) |
|11 | `numerics_<pillar3>_<X>.hexa`        | pillar 3 second T2 (e.g., Liouville volume) |
|12 | `numerics_cross_pillar.hexa`         | pillar 간 numerical anchor 일치 |
|13 | `numerics_lattice_arithmetic.hexa`   | n=6 lattice 의 float ↔ int 정밀도 cross-check |

### Meta tier — 2 scripts

| #  | 파일                          | 역할 |
|:---|:-----------------------------|:-----|
| 14 | `falsifier_check.hexa`       | F-<PROJECT>-1/2/3 closure-progress tracker (3-tier ladder, %) |
| 15 | `lint_numerics.hexa`         | 모든 `numerics_*.hexa` 가 5 invariant 따르는지 grep-lint |

### Build + Tests

| 위치                                  | 역할 |
|:-------------------------------------|:-----|
| `build/Makefile` + `build/header.tex` | pandoc + xelatex 로 모든 pillar PDF rebuild |
| `tests/test_lattice.hexa`             | verify/lattice_check 회귀 |
| `tests/test_calculators.hexa`         | 모든 numerics_* + calc_* 회귀 |
| `tests/test_cli_verify.hexa`          | `<project> verify all` aggregate 회귀 |
| `tests/test_all.hexa`                 | 위 4개 + selftest aggregator |

### CLI 확장

`cli/<project>.hexa` 에 `verify [<sub>]` 서브커맨드 추가:

```
<project> verify all       # 모든 verify/* 일괄 실행, exit 코드 합산
<project> verify lattice
<project> verify cross-doc
<project> verify <pillar1>
<project> verify numerics-<pillar1>
<project> verify numerics-<pillar1>-parity
<project> verify numerics-<pillar1>-solver
<project> verify <pillar2>
…
<project> verify falsifier
<project> verify lint-numerics
```

---

## §2 한 iteration 의 7-step 사이클

각 commit/push 사이클은 다음 7단계:

1. **Chunk 선택** — 위 인벤토리에서 아직 미작성된 슬롯 1개, 또는 기존
   파일 중 보강 가치 있는 1곳. (한 commit = 한 chunk = 한 PR 단위.)
2. **Write** — `.hexa` 파일 작성 (math_pure import + RUN/FAIL counters
   + FALSIFIERS list + `__HEXA_<PROJECT>_<NAME>__ PASS` sentinel +
   `exit(0)`). lint_numerics 가 enforced.
3. **Run** — `hexa run verify/<new>.hexa` 로 단독 실행, n/n PASS 확인.
4. **Wire** — `cli/<project>.hexa` `VERIFY_SUBS` + dispatcher + help 갱신.
   `tests/test_calculators.hexa` 에 (filename, sentinel) 한 줄 추가.
   `tests/test_cli_verify.hexa` 의 expected aggregate count bump.
   필요시 `verify/falsifier_check.hexa` `Fk_T2_SCRIPTS` 배열 + `verify/lint_numerics.hexa` `NUMERICS_SCRIPTS` 배열 갱신.
5. **Regression** — `<project> verify all` (모두 N+1/N+1 PASS),
   `hexa run tests/test_all.hexa` (4/4 PASS) 둘 다 실행.
6. **Doc** — `CHANGELOG.md` 에 `### Added (날짜 — Nth iteration)` 블록.
   필요시 `README.md` verify 표 + 인벤토리 트리 갱신.
7. **Commit + push** — 단일 commit, descriptive message, origin/main push.

이 7 step 을 한 chunk 당 반복.

---

## §3 closure pct 계산 (falsifier_check 의 sentinel)

각 falsifier 는 3-tier ladder 로 closure 를 추적:

```
closure_pct(t1_ok, t2_ok, t3_ok):
  n = sum(t1_ok, t2_ok, t3_ok)
  if n == 0: return 0
  if n == 1: return 33
  if n == 2: return 67
  if n == 3: return 100
```

- T1: `verify/calc_*.hexa` (algebraic) 가 disk 에 존재
- T2: `verify/numerics_*.hexa` (numerical) 모든 항목이 disk 에 존재
- T3: 실험 하드웨어 / 라이브 데이터 피드 — 항상 ✗ (Stage-1+ 후 닫힘)

T2 는 **배열** 로 저장 — 한 falsifier 가 여러 numerics 스크립트로 검증 가능.
hexa-cern 의 F-PCERN-3 는 numerics_wakefield + numerics_lwfa_parity + numerics_lwfa_solver = 3 T2 stack.

---

## §4 5 invariants enforced by lint_numerics.hexa

모든 `verify/numerics_*.hexa` 가 만족해야:

1. `use "self/runtime/math_pure"` import (raw float math 사용 금지)
2. `__HEXA_<PROJECT>_<NAME>__` sentinel prefix + `__ PASS` suffix
3. `FALSIFIERS` 배열 선언
4. `exit(0)` on PASS path
5. `let mut RUN = 0` + `let mut FAIL = 0` 카운터

추가: `NUMERICS_SCRIPTS` 인벤토리 배열 항목 수 == `verify/numerics_*.hexa`
on-disk glob count.

---

## §5 한 세션에서 진행한 결과 (hexa-cern v1.1.0-pre 사례)

- **15 iterations**, 16 commits
- **16 verify scripts** (4 algebraic + 9 numerical + 3 meta)
- **4 tests**, 3 PDFs
- **5500+ 라인 추가**, 0 .py
- F-PCERN-1/2/3 모두 67% closure (T1 ✓ + T2 ×2~3 ✓, T3 TBD)

각 iteration 은 ~30 분 (chunk 작성 + 회귀 + commit + push) 정도 소요.

---

## §6 다른 hexa-* 프로젝트로 옮길 때 주의점

1. `n=6 lattice` 는 모든 hexa-* 가 공유 (σ(6)=12, τ(6)=4, φ(6)=2, J₂=24).
   대상 프로젝트의 `.roadmap.<project> §A.1` 에 등재되어 있어야 함.
2. **falsifier 명칭 prefix** 는 프로젝트 별로 다름:
   - hexa-cern: `F-PCERN-*`
   - hexa-bio:  `F-BIO-*`
   - hexa-rtsc: `F-RTSC-*`
   - 등등. `.roadmap.<project> §A.4` 가 SSOT.
3. **published reference points** 도 프로젝트별:
   - hexa-cern: LEP / Tevatron / LHC / FCC + BELLA / FACET / ATHENA / FLASHFwd
   - hexa-bio: 알맞은 bio 실험 (CRISPR / MEA / fMRI / 등)
   - hexa-rtsc: MgB₂ / Nb₃Sn / NbTi / YBCO 등 SC 데이터
   - 각 도메인의 4-machine 류 비교 셋을 numerics_*_parity.hexa 가 사용.
4. **sentinel namespace** 는 `__HEXA_<PROJECT>_*__` 로 통일.
   hexa-cern 은 `__HEXA_CERN_*__`, hexa-bio 는 `__HEXA_BIO_*__` 등.
5. **numerics_lattice_arithmetic** 는 모든 프로젝트가 공유 — math_pure
   stability floor 는 도메인 무관.

---

## §7 새 hexa-* 세션 kickoff (paste-ready)

새 세션에 다음을 입력하면 본 레시피로 즉시 시작:

```
hexa-<프로젝트> 의 runnable surface 를
docs/runnable_surface_recipe.md 패턴대로 .hexa 로 구축한다.

1. 현재 verify/build/tests 인벤토리 점검 (이미 있는 것 확인)
2. 16-script 표준 인벤토리에서 미작성 슬롯 한 개씩 7-step 사이클로
   1 chunk = 1 commit 으로 추가
3. 각 iteration 끝에 verify all + test_all 회귀 실행 + push
4. CHANGELOG iteration 항목 + falsifier_check closure pct 추적
5. T3 (empirical) 은 미루고 T2 깊이 (closure-depth accumulation)
   에 집중

목표: F-<PROJECT>-1/2/3 모두 67% closure (T1 + T2 stack ≥ 1).
참고 SSOT: .roadmap.<project> §A.4 falsifier preregister.
```

---

— 본 레시피는 hexa-cern v1.1.0-pre 에서 검증됨.
   `git log --oneline` 0a74c21..main 의 16 commit 이 한 적용 사례.
