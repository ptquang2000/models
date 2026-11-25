# Progress

Single source of truth for what is proven. The TA updates this **before** a session ends.
The human may read it at any time; only evidence moves a status.

**No deadlines.** This is a gate-driven plan: work continues until every model is
`verified` + `explained`, every checkpoint is answered, and the capstone passes. Time
elapsed is not a progress metric.

## Status legend

| Status | Meaning |
| --- | --- |
| `todo` | Not started |
| `reading` | Read the theory, no working code yet |
| `implemented` | Code runs end to end, nothing verified |
| `verified` | Tests pass, including an oracle or closed-form check |
| `explained` | Can state objective, update rule, assumptions, failure mode, without notes |
| `demo` | The TA showed the solution; must be redone from a blank file |
| `blocked` | Two attempts produced no new information; needs a prerequisite |

A model is **done** only at `verified` + `explained`. Coverage is not progress.

## Current focus

_Set 2026-11-23._

- Track: A. Least squares
- Model: Phase 0 lab harness (Rust, hand-rolled `Mat`)
- Reading: ISL §2.2, *Assessing Model Accuracy* (stop at the Python lab)
- Next action: settle the three-way split's return shape and size rule, write its acceptance test against the Phase 0 gate, then the split code
- Last session: 2026-11-24 (ISL §2.2 gate: test error vs. training error, LR-0002)

## Models

| # | Model | Status | Code | Check that proves it | Explain-back | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | Lab harness (split, scaler, metrics) | todo | `models.rs` | — | — | `Mat` (arena, row-major) + `test_mat` done 2026-11-23; split, scaler, metrics pending. Gradient checker moved to build 2 (no gradient exists yet) |
| 1 | Linear regression, closed form | todo | — | — | — | — |
| 2 | Linear regression, gradient descent | todo | — | — | — | — |
| 3 | Random forest regressor | todo | — | — | — | — |
| 4 | XGBoost regressor | todo | — | — | — | stretch |
| 5 | Logistic regression | todo | — | — | — | — |
| 6 | Random forest classifier | todo | — | — | — | — |
| 7 | XGBoost classifier | todo | — | — | — | stretch |
| 8 | k-means | todo | — | — | — | — |
| 9 | Hierarchical clustering | todo | — | — | — | — |
| 10 | DBSCAN | todo | — | — | — | — |
| 11 | Multinomial naive Bayes | todo | — | — | — | — |
| 12 | TF-IDF + logistic regression | todo | — | — | — | — |
| 13 | Capstone report | todo | — | — | — | — |

Required for the mission: 0, 1, 2, 5, 8, 11, 13. Builds 3, 4, 6, 7, 9, 10, and 12 are
optional depth. Their explain-backs are still required.

## Math track

Sections not listed are SKIP. Assignments and problem numbers are in `MATH-MAP.md`.

### Track A — least squares

| § | Topic | Status | Date | Notes |
| --- | --- | --- | --- | --- |
| 1.1 | Vectors and linear combinations | verified | 2026-11-20 | 1, 19, 25 correct after prompting. Weakness: drops conditions from the question. Explain-back pending |
| 1.2 | Lengths and dot products | verified | 2026-11-21 | 1, 2, 3, 8, 12 correct. Weak spots: producing a vector at a given angle took 3 attempts; counterexample was checked against the hypothesis but not the conclusion. Explain-back pending |
| 1.3 | Matrices and `Ax = b` | verified | 2026-11-23 | 1, 4, 6 done. Failure caught: assembled `S` with the given vectors as rows, i.e. computed `Sᵀx`, and asserted the result matched `b`. p4 combination `(1,−2,1)`, plane `b₁−2b₂+b₃=0`, `(1,0,0)` unreachable. p6: `c=3`, `c=−1`, independent for `c≠0` by components, `c=0` → `x=(−3,1,1)`. Explain-back pending |
| 2.1 | Row and column pictures | todo | — | Assigned 2026-11-24: read §2.1, problems 2, 5, 7. Acts as the prerequisite for the queue item "why does the column picture give the same `b`" |
| 2.2 | Elimination and pivots | todo | — | — |
| 2.5 | Inverse matrices | todo | — | — |
| 2.6 | `A = LU` | todo | — | — |
| 3.1 | Spaces of vectors | todo | — | — |
| 3.2 | Nullspace and free variables | todo | — | — |
| 3.3 | Complete solution | todo | — | — |
| 3.4 | Independence, basis, dimension | todo | — | — |
| 3.5 | Four subspaces | todo | — | — |
| 4.1 | Orthogonality of four subspaces | todo | — | — |
| 4.2 | Projections | todo | — | — |
| 4.3 | Least squares | todo | — | — |

### Track B — likelihood

| § | Topic | Status | Date | Notes |
| --- | --- | --- | --- | --- |
| 12.1 | Mean, variance, probability | todo | — | — |
| 12.2 | Covariance and Bayes | todo | — | — |
| — | Calculus refresher (chain rule, partials, log) | todo | — | — |

### Track C — eigen, SVD, PCA

| § | Topic | Status | Date | Notes |
| --- | --- | --- | --- | --- |
| 6.1 | Eigenvalues and eigenvectors | todo | — | — |
| 6.2 | Diagonalization | todo | — | — |
| 6.4 | Symmetric matrices | todo | — | — |
| 6.5 | Positive definite matrices | todo | — | — |
| 7.1 | SVD and image compression | todo | — | — |
| 7.2 | Bases and matrices in the SVD | todo | — | — |
| 7.3 | PCA by the SVD | todo | — | — |

### Track D — text (review)

| § | Topic | Status | Date | Notes |
| --- | --- | --- | --- | --- |
| 1.2 | Cosine similarity | todo | — | — |
| 12.2 | Bayes for naive Bayes | todo | — | — |
| 3.4 | Independence in high dimensions | todo | — | — |

## Checkpoints

Answer without notes. The TA asks one per session, not all at once.

| ID | Question | Status | Date |
| --- | --- | --- | --- |
| A1 | Why is `AᵀA` invertible when `A` has independent columns? | todo | — |
| A2 | Derive `AᵀAx = Aᵀb` from perpendicular residuals | todo | — |
| A3 | What does `I − P` project onto? | todo | — |
| A4 | Two identical columns: what breaks, and what should the code do? | todo | — |
| B1 | Why does Bernoulli likelihood maximization give log-loss? | todo | — |
| B2 | State Bayes' theorem and name naive Bayes' assumption | todo | — |
| B3 | Differentiate the logistic log-loss. Write the gradient in matrix form. | todo | — |
| C1 | Why are symmetric-matrix eigenvalues real and eigenvectors orthogonal? | todo | — |
| C2 | Why is `AᵀA` positive definite, and what does it buy? | todo | — |
| C3 | What does the first singular vector of centered data point along? | todo | — |
| C4 | Which objective does k-means minimize, and why does Lloyd never increase it? | todo | — |

## Concept spine

Fundamentals that must be explainable by the end, independent of any single model.
Tick only after an explain-back.

### Data and evaluation

- [ ] Train/validation/test split, and why test is touched once
- [ ] Leakage: scalers, vocabularies, target encoding, time order
- [ ] Bias–variance decomposition, in your own words
- [ ] Cross-validation, and when it is worth the cost
- [ ] Regression metrics: MSE, RMSE, MAE, R² — and when each misleads
- [ ] Classification metrics: accuracy, precision, recall, F1, ROC-AUC, PR curve
- [ ] Clustering metrics: inertia, silhouette, adjusted Rand index
- [ ] Baseline-first discipline

### Learning

- [ ] Loss vs. metric: why they differ
- [ ] Maximum likelihood → loss function (at least for squared and log loss)
- [ ] Gradient, learning rate, convexity, convergence
- [ ] Batch vs. stochastic vs. mini-batch
- [ ] Feature scaling and conditioning
- [ ] Regularization: L1 vs. L2, and what each does to weights
- [ ] Overfitting, early stopping, model capacity
- [ ] Hyperparameter tuning without test-set peeking
- [ ] Reproducibility: seeds, determinism, version pinning

### Models

- [ ] Least squares as projection; normal equations; rank condition
- [ ] Logistic regression as a linear log-odds model; sigmoid; log-loss
- [ ] Decision trees: greedy split, impurity, pruning
- [ ] Bagging vs. boosting, and why each reduces a different error term
- [ ] k-means: Lloyd's algorithm, initialization, choosing k, assumptions
- [ ] Hierarchical clustering: linkage, dendrogram, when it beats k-means
- [ ] DBSCAN: density, core/border/noise, ε sensitivity
- [ ] Naive Bayes: conditional independence, smoothing, log-space
- [ ] TF-IDF: term weighting, document normalization, sparsity, cosine similarity
- [ ] SVD and PCA: variance maximization, centering, explained variance (optional)

## Retrieval queue

Spaced review, not a schedule. Ask the next item whenever a session starts, regardless of
dates. Wrong answers go back one interval (1 → 3 → 7 → 21 days). Retrieval never blocks
forward progress; if an item is weak, drill it, then continue the phase.

**Gate:** an item is askable only when its `source` is `verified` or later. A row with
`waiting` in the result column sits out until its source gets there. Rule and exceptions:
`AGENTS.md`, session loop step 1.

| Item | Source | Last asked | Suggested next | Result |
| --- | --- | --- | --- | --- |
| What is a linear combination? What does the set of all combinations of two vectors look like? | §1.1 | 2026-11-23 | 2026-11-26 | partial — pass on the cone; gave the notation instead of the geometry for the unrestricted case, so the line-vs-plane condition was dropped again |
| `w` at 0°, 90°, 180° to `v`: what is it, and what test proves it? | §1.2 | 2026-11-24 | 2026-12-01 | pass, vectors first try. First proof wrote `v·w = cos θ` (dropped `|v||w|`); corrected to the ratio and produced 1 / 0 / −1 in one line each. Side condition `v ≠ 0` still had to be supplied |
| Row `i` of `A` dotted with `x`: which entry of `b` does it produce? | §1.3 | 2026-11-24 | 2026-11-25 | pass. The narrow claim only: the entry. It took two asks, 2026-11-23 and 2026-11-24 |
| Why does the column picture produce the same `b`? | §2.1 | — | — | waiting. Asked 2026-11-24 ahead of the reading; that was my error, not a failed retrieval. Activates after §2.1 is `verified` |
| Why estimate performance on data the model hasn't seen? | ISL §2.2 | 2026-11-24 | 2026-11-27 | pass. Mechanism reached via a 2-point line (training MSE 0 for any pair of points, predictable without data). Correction absorbed: `f̂` from training, test observations only supply `(x0, y0)`. LR-0002 |

## Session log

Newest first. One line each: date, type, focus, evidence, next action.

| Date | Type | Focus | Evidence | Next |
| --- | --- | --- | --- | --- |
| 2026-11-25 | process | Retrieval queue gate | A check-in on 2026-11-24 tested the §2.1 column picture while §2.1 was `todo`. The queue gained a `source` column and a gate rule in `AGENTS.md`; the broken item split into an askable §1.3 claim (pass) and a `waiting` §2.1 item | Split return shape and size rule |
| 2026-11-24 | plan | Path review against the mission and the teach skill | The path now starts from the mission. Required core: builds 0, 1, 2, 5, 8, 11, 13. Upgrades keep their builds as optional depth, and their explain-backs stay required. Add a validation set, the L2 reading, checkpoint B3, and the dataset gate. Mark track C optional. | Write the three-way split test |
| 2026-11-24 | reading | ISL §2.2, *Assessing Model Accuracy* | Test-error item passed after a 2-point micro-example: training MSE is 0 for any pair of points and was predicted before computing. `f̂`-comes-from-train correction absorbed. Training set / test set / test MSE entered the glossary. LR-0002 | Write the split acceptance test |
| 2026-11-23 | harness | Phase 0: `Mat` + layout check | `Mat` over the arena with `test_mat` pinning `set`/`get` and the row-major stride; the first draft's out-of-bounds `copy_nonoverlapping` was removed after review; a transposed stride fails the check | Answer the ISL §2.2 question, then write the split acceptance test |
| 2026-11-23 | math | §1.3 problems 1, 4, 6 | p1 stalled on row-vs-column assembly until a 2×2 micro-example; p4 and p6 correct after component-level probing, not before | Phase 0 harness: choose array library, then split + scaler |
| 2026-11-20 | math | §1.1 problems 1, 19, 25 | all correct after 3 rounds of prompts; recurring error: dropping one condition from the question | §1.2 problems 1, 2, 3, 8, 12 |
| 2026-11-17 | setup | Mission, stack, baseline | Stack: Rust; math from zero; coding strong | §1.1 problems 1, 19, 25 |
