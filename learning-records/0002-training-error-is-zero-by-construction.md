# 0002 — Training error is zero by construction, not evidence of fit

**Date:** 2026-11-24
**Context:** ISL §2.2.1–2.2.2 (ISLP 2023, printed pp. 27–33), Phase 0 reading gate.

Three attempts at "why is error on the fitted data useless as an estimate of new-data
performance" came back as restatements of ISL's *goal* — "minimize bias and variance" — rather
than the mechanism. The mechanism arrived only after a computed micro-example: fit a line
through `(0,1)`, `(1,3)`, then through `(0,5)`, `(1,−2)`. Training MSE was 0 both times, and
the learner then predicted, correctly, that it would be 0 before seeing the data. Stated in
their words: the estimate equals the computed value, so the number is fixed by the setup.

A second error surfaced on the same gate: asked where `f̂` comes from when evaluating
`Ave(y0 − f̂(x0))²`, the answer was "test data". Corrected to: `f̂` is estimated on the
training observations; test observations only supply `(x0, y0)`. The learner restated (2.6)
correctly in one sentence afterwards.

**Why it changes what to teach next:** two habits, both now demonstrated rather than
hypothetical. (1) Abstract definitions do not land; a two-point computation does. Any gate
about an estimator is to be asked with numbers first, words second. (2) The
fit-on-train / measure-on-test boundary is not yet automatic, so every harness artifact that
touches data must be reviewed for what it was allowed to see — the scaler and the split
tests are the first chance, and the leak check is not optional there.
