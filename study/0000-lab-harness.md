# 0000 — Lab harness

**Status:** `reading`
**Primary source:** ISL §2.2, *Assessing Model Accuracy* (ISLP 2023, printed pp. 27–39)
**Prerequisite:** programming diagnostic

## 1. Why this element exists

Nothing is measurable without a split, a scaler, and a metric. This element builds the three
tools that measure every later model. It is not a model. It is the lab.

## 2. Diagnostic

The diagnostic tests the programming contract. It does not test the harness concepts.

- **Question:** Write a Rust function `fn mean(xs: &[f64]) -> f64`. Write a `#[test]` for it.
  Read a file with one number per line and return the sum. State the shape of a 3×2 matrix
  stored as `Vec<Vec<f64>>` and the meaning of each index.
- **Pass:** the function compiles, the test passes, the file input works, and the shape
  statement names rows and columns.
- **Fail:** add a prerequisite unit from *The Rust Programming Language*, chapters 3–4, 8,
  and 11. Name the missing fundamental.

## 3. Concept lesson

- **Idea:** training error is not an estimate of performance on new data. A model can fit
  the training rows exactly and still fail on a new row. The fix is a held-out set.
- **Numeric example:** fit a line through `(0,1)` and `(1,3)`. The training MSE is 0. Fit a
  line through `(0,5)` and `(1,−2)`. The training MSE is 0 again. The number is fixed by the
  setup, not by the data.
- **Notation:** `f̂` comes from the training set. The test set supplies `(x₀, y₀)` only.
- **Source section:** ISL §2.2.1–2.2.2, stop at the Python lab.

## 4. Theory

The learner states the rule and computes the cases.

- **Objective:** estimate the error on unseen data.
- **Assumption:** the training rows and the test rows come from the same distribution.
- **Derivation:** the split is a function of `(n, seed, train_share, validation_share)`. The
  seed makes an arbitrary choice reproducible. The scaler uses train statistics only.
  MSE is `(1/n) Σ (yᵢ − ŷᵢ)²`.
- **Failure mode:** leakage. If the scaler sees all rows, the test rows change the train
  scaling. The measurement is no longer held out.

## 5. Reference card

- **Definition:** a split divides the observation indices into train, validation, and test
  sets. A scaler maps each feature to zero mean and unit variance. MSE is the average squared
  error.
- **Formula:** `z = (x − μ_train) / σ_train`. `MSE = (1/n) Σ (yᵢ − ŷᵢ)²`.
- **Failure mode:** fit the scaler on all rows. The test set leaks into training.

## 6. Build contract

- **Input:** `n`, `seed`, `train_share`, `validation_share`. A feature matrix `X`. A label
  vector `y`.
- **Output:** three index sets. A scaler with `fit` and `transform`. An MSE function.
- **Shapes:** `X` is `n × d`. `y` is `n × 1`. The split returns index vectors.
- **Error cases:** an empty set must return an error. A share that leaves no test row must
  return an error.
- **No ML library:** the split, the scaler, and MSE are hand-written.

## 7. Acceptance test

Write this before the code. Compute the cases by hand first.

- **Invariants:** the split is reproducible from its seed, disjoint, and exhaustive. The
  three sizes follow one stated rule, including when the shares do not divide evenly.
- **Oracle or closed form:** each metric matches a hand-computed fixture to 1e-12.
- **Degenerate cases:** `n = 3` at 60/20/20. `n = 10`. `n = 11`. `n = 8`.
- **Leak test:** a deliberately leaky scaler test must fail.
- **Tolerance:** 1e-12 for the fixtures.

## 8. Explain-back

- **Question:** Why is training error not an estimate of performance on new data? Why does
  the scaler use train statistics only?
- **Pass:** the answer names the mechanism, not the goal. The fit-on-train boundary is
  stated.

## 9. Sources

- **Primary:** ISL §2.2, ISLP 2023, printed pp. 27–39, stop at the Python lab.
- **Math:** none new. `../reference/math-map.html` track A starts with element 1.
- **Oracle:** hand-computed fixtures. No reference crate.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| 2026-11-27 | Extent confused with index (`m.len()` answered as `i`) | `0000a-shape-vs-index.md` | `verified` 2026-11-27 |
