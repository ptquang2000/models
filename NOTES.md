# Notes

Scratchpad for preferences, decisions, and open questions. Not a progress log — that is
`PROGRESS.md`.

## Teaching preferences

- **TA only.** The human writes all model code and derivations. Review, explain, assign.
  Never implement a loss, gradient, split, update rule, or distance metric.
- **Starting from zero in linear algebra.** Build every idea from a 2D/3D picture and a tiny
  numeric example before notation. Do not skip to abstraction because the coding is strong.
- **Curated math, not exhaustive math.** The failure mode is doing every textbook problem.
  Scope each set to ~30–40% by concept, skip drills, and finish that subset. See
  `MATH-MAP.md`.
- **No deadlines.** Work continues gate to gate until the curriculum is done. Do not defer
  the next step to a future session or plan around time.
- **Pinpointed reading.** "Read chapter 4" is not an answer. Give section, purpose, and a
  few problem numbers.
- **Progress must persist.** Update `PROGRESS.md` at the end of every session; write
  learning records when understanding is demonstrated.
- **Direct feedback.** No praise padding, no hedging. Three review comments maximum.

## Decisions

| Date | Decision | Why |
| --- | --- | --- |
| 2026-11-17 | Repo is a TA workspace, not an implementation service | The goal is understanding, not shipped code |
| 2026-11-17 | Strang 5th ed is the math spine, not the only source | It is the book on hand; other high-trust sources are allowed and recorded in `RESOURCES.md` |
| 2026-11-17 | No deadline; progress is gate-driven | The learner works until each model and checkpoint is done, not until a date |
| 2026-11-17 | Stack: Rust, built and tested with cargo | The learner's choice; array library and support crates still to be decided |
| 2026-11-17 | A reference implementation is allowed as a test oracle only | Keeps verification cheap without creating library dependence |

## Baseline

_Recorded 2026-11-17, first session._

- **Linear algebra:** from zero. Start at §1.1 with pictures; notation comes after examples.
- **Coding:** strong. The bottleneck is math, not tooling.
- **Stack:** Rust. Array library and crate choices are still open.

## Open questions

- [ ] Array library: `ndarray`, `nalgebra`, or hand-rolled? (Learner's call.)
- [ ] Dataset source in Rust: checked-in CSV, generated synthetic data, or a crate?
- [ ] Dataset interests for the capstone: tabular, text, or something else?
- [ ] Does the user want the repo to be a git repo with commits per model?

## Working conventions

- One model per module; one `fit`/`predict` surface; explicit hyperparameters.
- Experiments print the seed and the command to reproduce them.
- Scratch binaries for exploration; `src/` for anything that must run again.
- When the human asks "show me", mark the item `demo` in `PROGRESS.md`; it must be redone
  from a blank file to count.
