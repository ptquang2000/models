# 0005 — Logistic regression

**Status:** `todo`
**Primary source:** ISL §4.1–4.3, §4.4.2 pp. 155–156, §6.2.1, stop at §4.4.3
**Prerequisite:** element 2. MATH-MAP track B. Checkpoints B1–B3.

## 1. Why this element exists

This is the first classifier. It predicts a probability, not a number. It replaces the
squared loss with the likelihood of the labels. The model is still linear in the parameters,
but the output passes through a sigmoid.

## 2. Diagnostic

The diagnostic tests the probability and calculus prerequisites, not the new concept.

- **Question:** State Bayes' theorem for two classes. Differentiate `log z` and `e^z`. State
  the chain rule for `log σ(z)`, where `σ(z) = 1 / (1 + e⁻ᶻ)`.
- **Pass:** the theorem is correct and the two derivatives are correct.
- **Fail:** add a prerequisite unit from `../reference/math-map.html` track B, §12.1–12.2 and the calculus
  refresher.

## 3. Concept lesson

- **Idea:** model the log-odds as a linear function of `x`. The sigmoid turns the linear
  score into a probability. Fit the weights by maximum likelihood, not by least squares.
- **Numeric example:** one feature, two points. Compute the probability for `w = 0` and for
  `w = 1`. Show that the log-loss punishes a confident wrong answer.
- **Notation:** `p = σ(wᵀx)`. `log-loss = −[y log p + (1 − y) log(1 − p)]`.
- **Source section:** ISL §4.1–4.3.

## 4. Theory

The learner derives this.

- **Objective:** maximize the Bernoulli likelihood of the labels.
- **Assumption:** the log-odds are linear in `x`. The rows are independent.
- **Derivation:** write the likelihood, take the log, and differentiate. The gradient is
  `Xᵀ(σ(Xw) − y)`. The loss is convex.
- **Failure mode:** perfect separation drives the weights to infinity. L2 regularization
  controls this. A threshold is not a probability.

## 5. Reference card

- **Definition:** logistic regression models the log-odds of the positive class as a linear
  function of the features.
- **Formula:** `p = σ(wᵀx)`. `∇L(w) = Xᵀ(σ(Xw) − y)`.
- **Failure mode:** separation, unscaled features, a threshold read as a probability.

## 6. Build contract

- **Input:** `X` (`n × d`), `y` (`n × 1`, values 0 or 1), step size, L2 strength.
- **Output:** `w` (`d × 1`), `predict_proba(X) -> p`, `predict(X, threshold) -> label`,
  precision, recall, F1.
- **Shapes:** `p` is `n × 1`. The gradient has the same shape as `w`.
- **Error cases:** a label outside `{0, 1}` must return an error. A non-finite probability
  must return an error.
- **No ML library:** the sigmoid, the loss, the gradient, and the metrics are hand-written.

## 7. Acceptance test

- **Invariants:** every probability is in `(0, 1)`. The loss does not increase.
- **Oracle or closed form:** the gradient check passes to relative error < 1e-6. Two runs
  from different starting points agree to 1e-6.
- **Degenerate cases:** one class only. A constant feature. A feature with zero variance.
- **Tolerance:** 1e-6 for the gradient check and the restart check.
- **Baseline:** F1 on the validation set beats the majority-class baseline.

## 8. Explain-back

- **Question:** Derive the logistic log-loss from the Bernoulli likelihood. Write the
  gradient in matrix form.
- **Pass:** the derivation is correct and the gradient is `Xᵀ(σ(Xw) − y)`.

## 9. Sources

- **Primary:** ISL §4.1–4.3, §4.4.2 pp. 155–156, §6.2.1.
- **Math:** Strang §12.1–12.2. Checkpoint B1–B3.
- **Oracle:** hand-computed fixture. Optional reference implementation in tests only.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
