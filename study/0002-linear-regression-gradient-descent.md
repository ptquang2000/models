# 0002 — Linear regression, gradient descent

**Status:** `todo`
**Primary source:** Boyd & Vandenberghe ch. 5, the gradient-descent part, stop at Newton's
method
**Prerequisite:** element 1. Calculus refresher (chain rule, partials).

## 1. Why this element exists

The closed form needs a matrix solve. It does not scale to large data. It also does not
generalize to logistic regression, which has no closed form. Gradient descent is the general
tool. This element builds it on a loss that already has a known answer, so the check is
cheap.

## 2. Diagnostic

The diagnostic tests the calculus prerequisite, not the new concept.

- **Question:** State the chain rule for `f(g(x))`. Differentiate `z²`, `log z`, and
  `3w₁ + 2w₂`. State the shape of the gradient of a scalar function of `w` (`d × 1`).
- **Pass:** the three derivatives are correct and the gradient shape is correct.
- **Fail:** add a prerequisite unit from the calculus refresher in `../reference/math-map.html` track B.

## 3. Concept lesson

- **Idea:** the gradient points uphill. Move against it. The step size controls the descent.
  The loss is convex, so a small enough step reaches the minimum.
- **Numeric example:** `f(w) = (w − 3)²`. Start at `w = 0`, step size `0.1`. The gradient is
  `2(w − 3)`. Compute the first three steps by hand.
- **Notation:** `∇f(w)` is a `d × 1` vector. `w ← w − η ∇f(w)`.
- **Source section:** Boyd ch. 5, the gradient-descent part.

## 4. Theory

The learner derives this.

- **Objective:** minimize `L(w) = ‖Xw − y‖²`.
- **Assumption:** `L` is convex and differentiable. A small enough step decreases it.
- **Derivation:** expand `L`, differentiate with respect to `w`, and use the chain rule. The
  result is `∇L(w) = 2Xᵀ(Xw − y)`. The step is `w ← w − η ∇L(w)`.
- **Failure mode:** a step that is too large increases the loss. Feature scaling changes the
  curvature and the safe step size.

## 5. Reference card

- **Definition:** gradient descent moves the parameters against the gradient of the loss.
- **Formula:** `∇L(w) = 2Xᵀ(Xw − y)`. `w ← w − η ∇L(w)`.
- **Failure mode:** a large step size diverges. Unscaled features make the loss surface
  narrow and the descent slow.

## 6. Build contract

- **Input:** `X` (`n × d`), `y` (`n × 1`), step size `η`, number of steps, tolerance.
- **Output:** `w` (`d × 1`), a loss history, a gradient checker.
- **Shapes:** the gradient has the same shape as `w`.
- **Error cases:** a non-finite loss must stop the run. A zero gradient must stop the run.
- **No ML library:** the gradient and the update are hand-written.

## 7. Acceptance test

- **Invariants:** the loss does not increase at a small enough step.
- **Oracle or closed form:** the gradient check agrees to relative error < 1e-6 in float64 on
  3–5 parameters. The final weights match element 1 on standardized data to 1e-6.
- **Degenerate cases:** a deliberately wrong gradient must fail the check. A step size that
  is too large must increase the loss.
- **Tolerance:** 1e-6 relative error for the gradient check.

## 8. Explain-back

- **Question:** Why does feature scaling change the number of steps? What is the gradient of
  `‖Xw − y‖²`?
- **Pass:** the answer gives the gradient and connects the curvature ratio to the step size.

## 9. Sources

- **Primary:** Boyd & Vandenberghe ch. 5.
- **Math:** `../reference/math-map.html` calculus refresher. Strang §4.3 for the normal equations.
- **Oracle:** element 1's closed form.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
