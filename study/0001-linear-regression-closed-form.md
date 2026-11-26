# 0001 — Linear regression, closed form

**Status:** `todo`
**Primary source:** ISL §3.1–3.2, stop at §3.3
**Prerequisite:** element 0. MATH-MAP track A through §4.3

## 1. Why this element exists

This is the first model. It predicts a number. It is also the cleanest picture of least
squares: the fitted values are the projection of `y` onto the column space of `X`.

## 2. Diagnostic

- **Question:** State the shape of `X`, `w`, and `y` in `Xw = y`. What does each column of
  `X` mean? What does the residual `e = y − Xw` measure?
- **Pass:** the shapes are correct, the column meaning is correct, and the residual is
  described as the part of `y` outside the column space.
- **Fail:** add a prerequisite unit from MATH-MAP track A, §1.3 and §3.1.

## 3. Concept lesson

- **Idea:** least squares chooses the `w` that makes `Xw` the closest point to `y` in the
  column space. The closest point leaves a residual perpendicular to that space.
- **Numeric example:** three points, one feature. Compute the mean of `x` and `y`, the slope,
  and the intercept by hand.
- **Notation:** `X` is `n × d`, `w` is `d × 1`, `y` is `n × 1`.
- **Source section:** ISL §3.1–3.2.

## 4. Theory

The learner derives this three ways: geometry, calculus, algebra.

- **Objective:** minimize `‖Xw − y‖²`.
- **Assumption:** the columns of `X` are independent. If not, `XᵀX` is singular.
- **Derivation:** set the residual perpendicular to every column: `Xᵀ(y − Xw) = 0`. This
  gives `XᵀXw = Xᵀy`. The calculus route sets the gradient to zero. The algebra route uses
  the normal equations.
- **Failure mode:** two identical columns. `XᵀX` is singular. The code must raise a clear
  error, not return garbage.

## 5. Reference card

- **Definition:** the least-squares fit is the projection of `y` onto the column space of
  `X`.
- **Formula:** `w = (XᵀX)⁻¹Xᵀy`. Residual `e = y − Xw`. `Xᵀe = 0`.
- **Failure mode:** rank-deficient `X`.

## 6. Build contract

- **Input:** `X` (`n × d`), `y` (`n × 1`).
- **Output:** `w` (`d × 1`), `predict(X) -> ŷ`, `R²`.
- **Shapes:** add an intercept column explicitly. State the shape before the multiply.
- **Error cases:** singular `XᵀX` must return an error. `n < d` must return an error or use a
  stated rule.
- **No ML library:** solve the normal equations with your own elimination or a stated
  decomposition. Do not invert the matrix.

## 7. Acceptance test

- **Invariants:** `Xᵀe ≈ 0` to 1e-10. `R²` matches a hand-computed `1 − SSE/SST` fixture.
- **Oracle or closed form:** agrees with element 2's gradient descent to 1e-6 on standardized
  data.
- **Degenerate cases:** two identical columns. One feature. Constant feature.
- **Tolerance:** 1e-6 for the cross-check, 1e-10 for orthogonality.

## 8. Explain-back

- **Question:** Why is the residual orthogonal to the column space? Derive `XᵀXw = Xᵀy`.
- **Pass:** the derivation uses the projection picture and the normal equations.

## 9. Sources

- **Primary:** ISL §3.1–3.2.
- **Math:** Strang §4.1–4.3. Checkpoints A1–A4.
- **Oracle:** hand-computed fixture. Optional reference implementation in tests only.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
