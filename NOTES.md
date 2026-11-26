# Notes

Scratchpad for preferences, decisions, and open questions. Not a progress log — that is
`reference/progress.html`.

## Teaching preferences

- **TA only.** The human writes all model code and derivations. Review, explain, assign.
  Never implement a loss, gradient, split, update rule, or distance metric.
- **Starting from zero in linear algebra.** Build every idea from a 2D/3D picture and a tiny
  numeric example before notation. Do not skip to abstraction because the coding is strong.
- **Curated math, not exhaustive math.** The failure mode is doing every textbook problem.
  Scope each set to ~30–40% by concept, skip drills, and finish that subset. See
  `reference/math-map.html`.
- **No deadlines.** Work continues gate to gate until the curriculum is done. Do not defer
  the next step to a future session or plan around time.
- **Pinpointed reading.** "Read chapter 4" is not an answer. Give section, purpose, and a
  few problem numbers.
- **Progress must persist.** Update `reference/progress.html` at the end of every session; write
  learning records when understanding is demonstrated.
- **Direct feedback.** No praise padding, no hedging. Three review comments maximum.
- **Cases before gates.** A test written before the rule is understood turns into a guess, then into a copy of the implementation. Compute the rule's cases by hand first, and write the test from those cases.
- **Study element, not a chapter.** Each model has a diagnostic, a concept lesson, a reference card, a build, and an explain-back. Do not start the build before the diagnostic and the source section.

## Decisions

| Date | Decision | Why |
| --- | --- | --- |
| 2026-11-17 | Repo is a TA workspace, not an implementation service | The goal is understanding, not shipped code |
| 2026-11-17 | Strang 5th ed is the math spine, not the only source | It is the book on hand; other high-trust sources are allowed and recorded in `RESOURCES.md` |
| 2026-11-17 | No deadline; progress is gate-driven | The learner works until each model and checkpoint is done, not until a date |
| 2026-11-17 | Stack: Rust, built and tested with cargo | The learner's choice; array library and support crates still to be decided |
| 2026-11-17 | A reference implementation is allowed as a test oracle only | Keeps verification cheap without creating library dependence |
| 2026-11-23 | Array representation: hand-rolled `Mat` over an mmap arena (`NonNull<f64>`, row-major); supersedes the flat `Vec<f64>` plan | Learner's call — wants to own the storage and indexing |
| 2026-11-23 | Build of record is `build.sh` + `rustc`; `Cargo.toml` + `build.rs` exist only to feed rust-analyzer | Learner's decision; supersedes "built and tested with cargo". Consequence: tests are compiled by `build.sh` with `rustc --test`, not by `cargo test` |
| 2026-11-23 | Metric oracle: hand-computed fixtures, no reference crate | Consequence of dropping cargo; keeps the oracle visible in the test |
| 2026-11-23 | Input checks use `debug_assert!` | Learner's decision; active at `-C opt-level=0`, compiled out above it |
| 2026-11-23 | Each build has a pinned, section-level primary reading; a build does not start until that reading is done and its terms are defined | Learner asked to study each concept before coding it; the reading track lives in `reference/curriculum.html` |
| 2026-11-23 | Platform/model split: OS-level failure handling (mmap, file I/O) is platform code; model code assumes valid memory | Learner's call — mirrors Handmade Hero's platform/game separation |
| 2026-11-23 | Checks run manually (`./build/linux_models_test`), not from `build.sh` | Learner's call — build and check stay separate |
| 2026-11-24 | Required core is builds 0, 1, 2, 5, 8, 11, 13; builds 3, 4, 6, 7, 9, 10, 12 are optional depth | The mission asks for four starter models and an explanation of the upgrades. The full 13-build path exceeded that goal. The upgrade explain-backs stay required. |
| 2026-11-25 | Retrieval items carry a `source` and are askable only when that source is `verified` or later | A check-in on 2026-11-24 tested the §2.1 column-picture idea while §2.1 was still `todo`. The learner had not studied it, so the failure was mine. The queue now gates every item on its source. |
| 2026-11-25 | Build 0 order: hand cases first, then the acceptance test | The learner reported that test-first blocked his understanding of the split. The test now comes from cases he computes by hand, and never from the output of his code. |
| 2026-11-26 | Core path is builds 0, 1, 2, 5, 8, 11, 13. The study element is the unit of work | The mission asks for four starter models and a capstone. The full 13-build path exceeded that goal. The upgrades stay explain-only. |
| 2026-11-26 | Programming is a tested prerequisite, not a taught module | The learner is a strong coder. The path tests the contract once and adds a repair unit only on failure. |
| 2026-11-26 | Markdown study elements now; HTML lessons per element | The plan must be stable before the teaching. Lessons start when an element starts, so they stay in the zone of proximal development. |
| 2026-11-26 | A failed diagnostic or gate adds a prerequisite study element | The path must fill a knowledge gap. The main element waits. The learner does not grind. |

## Baseline

_Recorded 2026-11-17, first session._

- **Linear algebra:** from zero. Start at §1.1 with pictures; notation comes after examples.
- **Coding:** strong. The bottleneck is math, not tooling.
- **Stack:** Rust. Array library and crate choices are still open.

## Open questions

- [x] Array library: hand-rolled `Mat` over an mmap arena, row-major (decided 2026-11-23; flat `Vec<f64>` plan superseded same day)
- [ ] Dataset source in Rust: checked-in CSV, generated synthetic data, or a crate?
- [ ] Dataset interests for the capstone: tabular, text, or something else?
- [ ] Does the user want the repo to be a git repo with commits per model?

## Working conventions

- One model per module; one `fit`/`predict` surface; explicit hyperparameters.
- Experiments print the seed and the command to reproduce them.
- Scratch binaries for exploration; `src/` for anything that must run again.
- When the human asks "show me", mark the item `demo` in `reference/progress.html`; it must be redone
  from a blank file to count.
