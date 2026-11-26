# 0013 — Capstone

**Status:** `todo`
**Primary source:** ISL ch. 5, *Resampling Methods*, stop at the lab
**Prerequisite:** elements 0, 1, 2, 5, 8, 11

## 1. Why this element exists

The mission asks for one dataset end to end: split, baseline, train, evaluate, diagnose,
compare, and defend the choice with numbers. This element proves the whole path on one
problem. It adds no new model code.

## 2. Diagnostic

The diagnostic tests the evaluation prerequisite, not the new concept.

- **Question:** Name the metric for your problem type and the baseline. State why the test
  set is read once. State the difference between a validation set and cross-validation.
- **Pass:** the metric, the baseline, and the one-read rule are correct.
- **Fail:** add a prerequisite unit from ISL ch. 5 and element 0.

## 3. Concept lesson

- **Idea:** a model choice is a decision under uncertainty. The validation number is the
  evidence. The test number is the final report, not a tuning signal.
- **Numeric example:** one dataset, two models, one split. Show that the model with the
  lower training error can have the higher validation error.
- **Notation:** `MSE`, `R²`, `accuracy`, `precision`, `recall`, `F1`, `ROC-AUC`, `inertia`,
  `silhouette`.
- **Source section:** ISL ch. 5.

## 4. Theory

The learner states this.

- **Objective:** choose one model and defend it with a validation number.
- **Assumption:** the split is random and the rows are independent.
- **Derivation:** no new derivation. The report connects the metric to the problem type and
  the model assumption.
- **Failure mode:** test-set peeking. A model choice made on the test set is not a held-out
  result.

## 5. Reference card

- **Definition:** the capstone is one dataset, one split, one baseline, one starter model,
  one upgrade, and one report.
- **Formula:** the report uses the metric that matches the problem type.
- **Failure mode:** test-set peeking, a baseline that is not measured, a claim with no
  command.

## 6. Build contract

- **Input:** one dataset and one problem type.
- **Output:** a report, the code, and one reproducibility command.
- **Shapes:** the report states the number of rows, the number of features, and the split
  sizes.
- **Error cases:** a missing seed must stop the run. A missing baseline must stop the report.
- **No ML library:** the models are the ones from the core path.

## 7. Acceptance test

- **Invariants:** every number in the report comes from the command. No metric comes from
  training data alone.
- **Oracle or closed form:** the baseline is measured, not assumed.
- **Degenerate cases:** a small dataset. An imbalanced class. A feature with missing values.
- **Tolerance:** the report states the tolerance for each comparison.

## 8. Explain-back

- **Question:** Defend the model choice with a validation number. State one failure mode and
  what you would do with 10× more data.
- **Pass:** the answer uses a measured number and names a real failure mode.

## 9. Sources

- **Primary:** ISL ch. 5.
- **Math:** the checkpoints for the chosen model.
- **Oracle:** the baseline and the validation split.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
