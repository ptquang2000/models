# Curriculum

Four problem types, thirteen builds. Each model is one module, one `fit` / `predict`
surface, and one acceptance gate. The order is deliberate: every model reuses a
fundamental from the one before it. Start the next build only when the current one passes
its gate; there is no deadline.

Status lives in `PROGRESS.md`. Math assignments live in `MATH-MAP.md`. This file only says
what to build and what must be true when it is done.

## The map

| Problem type | Starter | Upgrade |
| --- | --- | --- |
| Predict a number | Linear regression | Random forest / XGBoost regressor |
| Sort into categories | Logistic regression | Random forest / XGBoost classifier |
| Group unlabeled data | k-means | Hierarchical clustering / DBSCAN |
| Text classification | Naive Bayes | Logistic regression with TF-IDF |

## Path to the mission

The mission sets the final goal. This table gives the shortest path to that goal. Each row
is one gate. A build does not start before its prerequisites pass.

The math gates are in `MATH-MAP.md`. Checkpoints A2 and B3 cover the two derivations in
the mission.

F1 is the harmonic mean of precision and recall.

| Build | Step | Gate |
| --- | --- | --- |
| 0 | Lab harness: split, scaler, metrics | The tests pass. The leak test fails. |
| 1 | Linear regression, closed form | The normal equations work. The residual is orthogonal to the columns. |
| 2 | Linear regression, gradient descent | The loss does not increase. The gradient check passes. |
| 5 | Logistic regression | The gradient check passes. Probabilities are in (0, 1). F1 beats the baseline. |
| 8 | k-means | Inertia does not increase. The code recovers three blobs. |
| 11 | Multinomial naive Bayes | Probabilities sum to 1. F1 beats the baseline. |
| 13 | Capstone | The report passes its checks. One command reproduces every number. |

Builds 3, 4, 6, 7, 9, 10, and 12 are optional depth. The mission requires an explanation
of each upgrade, not its code. Read the upgrade source and answer when it beats the
starter. The upgrade explain-backs are required. Build 3 and build 6 hold the tree code
that builds 4 and 7 reuse.

## Reading track

Every build has one primary reading, pinned to a section, read before the code and only to
the depth the build needs. This is the same discipline as `MATH-MAP.md`: a fixed subset is
the unit of work, and finishing a chapter is not a goal. Derivations stay in `MATH-MAP.md`;
this table is the "why it works" reading.

| # | Build | Read before coding | Stop at | What it answers |
| --- | --- | --- | --- | --- |
| 0 | Lab harness | ISL §2.2, *Assessing Model Accuracy* | the Python lab | why held-out error, not training error, estimates performance |
| 1 | Linear regression, closed form | ISL §3.1–3.2 | §3.3 | what least squares estimates, and what R² does and does not say |
| 2 | Linear regression, gradient descent | Boyd & Vandenberghe ch. 5, the gradient-descent part | Newton's method | step size, convergence, why scaling changes the path |
| 3 | Random forest regressor | ISL §8.1–8.2 | the ch. 8 lab | greedy splitting, bagging, why averaging cuts variance |
| 4 | XGBoost regressor | Chen & Guestrin (2016), introduction and regularized objective | the system-design section | the gain formula, and why the objective is second-order |
| 5 | Logistic regression | ISL §4.1–4.3, §4.4.2 pp. 155–156, §6.2.1 (ridge) | §4.4.3 | log-odds, sigmoid, why log-loss and not squared error |
| 6 | Random forest classifier | ISL §8.1, the classification parts | §8.2 | Gini vs. entropy, probability leaves |
| 7 | XGBoost classifier | Chen & Guestrin (2016), the logistic objective | the system-design section | base score from the prior, calibrated probabilities |
| 8 | k-means | ISL §12.4.1 | §12.4.2 | the within-cluster objective, and why Lloyd cannot increase it |
| 9 | Hierarchical clustering | ISL §12.4.2 | §12.4.3 | linkage choice, reading a dendrogram |
| 10 | DBSCAN | Ester et al. (1996), the definitions | the experiments | core, border, noise, density reachability |
| 11 | Naive Bayes | ISL §4.4, the naive Bayes part | §4.5 | the independence assumption, and why it still works |
| 12 | TF-IDF + logistic regression | Jurafsky & Martin, *SLP3* ch. 6 | the embeddings part | tf-idf weighting, cosine, sparsity |
| 13 | Capstone | ISL ch. 5, *Resampling Methods* | the lab | validation vs. cross-validation, reading a learning curve |

Rules:

1. **One primary source per build.** The rest of the list stays shut until that build is
   done.
2. **Section-level and capped.** The "stop at" column is part of the assignment, not a
   suggestion.
3. **Verify the section when the build starts.** Chapter numbers here are safe; section
   numbers get checked against the PDF, and the source wins if they disagree.
4. **No build starts until three things are true:** the reading is done, every term in the
   build assignment has been defined in words (in the session or in `GLOSSARY.md`), and
   the acceptance test is written down before the code.
5. **Reading is not progress.** A build moves to `verified` on tests, never on pages read.
   A term enters `GLOSSARY.md` only after it is used correctly.
6. **A starter model counts as implemented only from a blank file.** Close the reference
   and the previous code. Write the model again. A model copied from a reference is a
   `demo`, not `verified`.
7. **Every build has two gates.** Write the acceptance test and one explain-back question
   before the code. The build passes when the tests pass and the answer is correct without
   notes.
8. **The test set is read once.** Fit the scaler on train. Make every choice on train or
   validation. Touch the test set only for the final number.

An **acceptance test** is the written statement of what must be true for a build to count
as done. Write it before any code exists. It pins observable behavior: outputs, invariants,
and error cases. It does not pin the internal steps. The implementation cannot define its
own success.

A test enforces a rule. It cannot invent one. When a build contains a free choice — a size
rule for remainders, a tie-break, a default tolerance — write the choice in prose beside the
test, then let the test enforce it. Test-first does not remove the decision.

## Phase 0 — Lab harness

Not a model, but nothing else is measurable without it. Build the three pieces in this
order. Each piece is the tool that measures the next piece.

1. **Seeded three-way split.** Divide the observation indices `0..n` into a train set, a
   validation set, and a test set. The split is a function of `(n, seed, train_share,
   validation_share)` and nothing else. The seed makes an arbitrary choice reproducible.
   The same four inputs must always give the same three sets. The split may not read any
   feature or label value. Determinism is not tidiness. If two runs split differently,
   every comparison mixes a model difference with a split difference.
2. **Standardization, fitted on train only.** Compute one mean and one standard deviation
   per feature on the train rows. Apply `(x − mean) / std` to all three sets. If the
   statistics come from all rows, the test rows change the scaling of the train rows. That
   is leakage.
3. **Metrics.** MSE only. It is the one metric §2.2 uses, and the only metric that means
   anything before a model exists. R² enters with build 1, where §3.2 defines it. RMSE and
   MAE are one-line definitions in the session when build 1 needs them. Accuracy,
   precision, recall, the confusion matrix, and F1 enter with build 5 (§4.4.2, pp. 155–156),
   the first classifier.

The finite-difference gradient checker belongs to build 2. Build 2 is the first build with
a gradient to check.

**Acceptance:** the split is reproducible from its seed, disjoint, and exhaustive. The
three sizes follow one stated rule, including when the shares do not divide evenly. Each
implemented metric matches a hand-computed fixture to 1e-12. A deliberately leaky scaler
test fails. Make every choice on train or validation. Do not touch the test set before the
final number.

## Phase 1 — Predict a number

### 1. Linear regression, closed form

- **Math:** projection onto a column space, normal equations, why AᵀA is invertible for
  full column rank (Strang 4.1–4.3).
- **Build:** `fit(X, y)` solves AᵀAx = Aᵀb; `predict`; intercept handling. L2 is deferred
  to build 5, where the reading defines it.
- **Fundamentals:** least squares as orthogonal projection, residual orthogonality,
  rank and multicollinearity, R² vs. RMSE.
- **Acceptance:** agrees with the gradient-descent version to 1e-6; residual `e = y − ŷ` is
  orthogonal to the columns of X to 1e-10; R² matches a hand-computed `1 − SSE/SST` fixture
  and is greater than 0 on the validation set; singular X raises a clear error instead of
  returning garbage.
- **Stretch:** weighted least squares; compare `solve` vs. explicit inverse for
  conditioning.

### 2. Linear regression, gradient descent

- **Math:** derivative of ‖Xw − y‖², learning rate, convexity.
- **Build:** batch gradient descent with a loss history; optional mini-batch; the
  finite-difference gradient checker (central difference against the analytic gradient).
- **Fundamentals:** why GD is needed for models without a closed form, convergence vs.
  step size, epoch vs. iteration, and feature scaling. Conditioning is the ratio between
  the largest and smallest curvature of the loss surface. A large ratio makes the steps
  slow.
- **Acceptance:** loss is non-increasing at a small enough step; the gradient checker
  agrees to relative error < 1e-6 in float64 on 3–5 parameters and flags a deliberately
  wrong gradient; final weights match the closed form on standardized data.
- **Stretch:** momentum; compare iterations to convergence with and without scaling.

### 3. Random forest regressor

- **Math:** no new linear algebra. Variance reduction from averaging; recursive
  partitioning. Entropy/Gini are for classification, MSE reduction for regression.
- **Build:** CART regression tree (best split by SSE reduction, min samples, max depth),
  bootstrap sampling, feature subsampling, prediction by averaging.
- **Fundamentals:** greedy splitting, overfitting control by depth and leaf size, bagging,
  out-of-bag error, why trees do not need scaling.
- **Acceptance:** a depth-1 tree equals the best single split found by brute force; a deep
  tree fits training data to ~0 error; forest test RMSE is lower than a single tree on a
  nonlinear dataset.
- **Stretch:** permutation importance; compare with and without feature subsampling.

### 4. XGBoost regressor (stretch)

- **Math:** second-order Taylor expansion of the loss, regularized objective, gain formula.
- **Build:** gradient boosting with regression trees, shrinkage, row and column
  subsampling, L2 leaf regularization.
- **Fundamentals:** boosting vs. bagging, why second-order information speeds convergence,
  shrinkage and early stopping, the bias–variance trade-off across rounds.
- **Acceptance:** gain formula implemented from the derivation; test loss decreases then
  rises with too many rounds; on the same data, beats or matches the random forest within
  5% and you can explain why.

## Phase 2 — Sort into categories

### 5. Logistic regression

- **Math:** log-odds, sigmoid, maximum likelihood, log-loss, gradient and Hessian;
  probability basics (Strang 12.1–12.2); chain rule.
- **Build:** binary logistic regression by gradient descent; L2 regularization; class
  weights optional; stable sigmoid; precision, recall, and F1 from the confusion matrix.
- **Fundamentals:** MLE vs. least squares, cross-entropy, why the loss is convex, decision
  threshold vs. ranking, calibration, imbalanced classes.
- **Acceptance:** the gradient check passes; the loss decreases at every step; each row
  gets one probability in (0, 1); two runs from different starting points agree to 1e-6;
  F1 on the validation set beats the majority-class baseline; the decision boundary plot
  exists for two features.
- **Stretch:** Newton/IRLS; multiclass softmax; ROC and AUC from scratch.

### 6. Random forest classifier

- **Math:** Gini impurity and entropy; information gain.
- **Build:** reuse the tree code with a classification criterion, class-probability leaves,
  bootstrap and feature subsampling, majority/probability averaging.
- **Fundamentals:** impurity vs. misclassification error, why probability averaging beats
  voting, class imbalance, OOB estimate.
- **Acceptance:** Gini and entropy splits agree on a hand-checked 3-point example; a
  single tree overfits (train accuracy 1.0) and the forest closes most of the difference;
  F1 on held-out data beats logistic regression on a nonlinear dataset.
- **Stretch:** compare impurity criteria; add `max_features` tuning without a library
  search.

### 7. XGBoost classifier (stretch)

- **Math:** logistic loss with the second-order approximation; base score from the
  log-odds of the prior.
- **Build:** gradient boosting for classification with the same tree core.
- **Fundamentals:** why the base score is not zero, log-loss vs. accuracy, probability
  calibration, early stopping on a validation set.
- **Acceptance:** log-loss decreases over rounds before overfitting; predicted
  probabilities are calibrated within 0.05 on a reliability plot; beats the random forest
  on at least one dataset and loses on another, with an explanation.

## Phase 3 — Group unlabeled data

### 8. k-means

- **Math:** squared Euclidean distance, the within-cluster objective, coordinate-descent
  view of Lloyd's algorithm.
- **Build:** Lloyd's algorithm, k-means++ initialization, multiple restarts, inertia
  history; elbow and silhouette.
- **Fundamentals:** why the objective never increases, local minima, choosing k, scaling
  before clustering, spherical-cluster assumption.
- **Acceptance:** inertia is non-increasing every iteration; two restarts on the same seed
  agree exactly; recovers 3 well-separated blobs with adjusted Rand index > 0.99; empty
  cluster handled explicitly.
- **Stretch:** mini-batch k-means; compare k-means++ with random init over 20 seeds.

### 9. Hierarchical agglomerative clustering

- **Math:** linkage definitions (single, complete, average), distance matrices, the
  Lance–Williams update.
- **Build:** naive O(n³) agglomerative clustering, linkage options, dendrogram with a
  plotting crate of your choice, flat cut at k clusters.
- **Fundamentals:** linkage choice changes the geometry, single linkage chaining,
  monotonicity of merge heights, when a dendrogram is more informative than a label.
- **Acceptance:** merge heights are non-decreasing for the three linkages; single linkage
  finds two moons that k-means cannot; produces the same labels as a hand-computed
  5-point example.
- **Stretch:** compare against a reference implementation on random points; complexity
  measurement as n grows.

### 10. DBSCAN

- **Math:** ε-neighborhoods, density reachability, core/border/noise points.
- **Build:** region queries (brute force; optional grid index), cluster expansion, noise
  labeling, `eps`/`min_samples` handling.
- **Fundamentals:** density vs. centroid assumptions, why it finds arbitrary shapes, the
  curse of dimensionality for distance, sensitivity to ε, no need to choose k.
- **Acceptance:** labels match a hand-worked 10-point example; noise points handled;
  increasing ε never splits a cluster; finds concentric circles where k-means fails.
- **Stretch:** k-distance plot for choosing ε; compare runtime with a spatial index.

## Phase 4 — Text classification

### 11. Multinomial naive Bayes

- **Math:** Bayes' theorem, conditional independence, multinomial likelihood, Laplace
  smoothing, log-space computation (Strang 12.2).
- **Build:** tokenizer, vocabulary with minimum frequency, class priors, conditional
  log-probabilities, Laplace/Lidstone smoothing, `predict_proba` in log-space.
- **Fundamentals:** the independence assumption and why it still works, prior vs.
  likelihood, smoothing as a prior, unseen words, why accuracy is misleading on imbalanced
  text.
- **Acceptance:** the class probabilities sum to 1; adding a constant to every count does
  not change the predicted class; F1 on the validation set beats the majority-class
  baseline; a hand-computed 2-word example matches.
- **Stretch:** complement NB; binary vs. multinomial event model; effect of stop words.

### 12. TF-IDF + logistic regression

- **Math:** vector space model, term frequency, inverse document frequency, cosine
  similarity, sparsity, L2 regularization.
- **Build:** TF-IDF transformer fitted on train only, sparse-friendly logistic regression
  (or dense on a reduced vocabulary), cosine nearest-neighbour baseline.
- **Fundamentals:** why IDF downweights common words, document length normalization, high
  dimensionality and sparsity, linear separability of text, feature leakage through
  vocabulary.
- **Acceptance:** TF-IDF matrix shape and row norms checked on a 3-document example;
  vocabulary fitted on train only; beats naive Bayes on the same split, or the difference
  is explained with evidence.
- **Stretch:** bigrams; hashing trick; compare L1 vs. L2 sparsity.

## Phase 5 — Capstone

One dataset, one report, no new model code.

- Baseline (mean or majority class), starter model, upgrade model, same split and metric.
- Cross-validation numbers for the final comparison.
- Learning curves and one failure case per model.
- One paragraph per upgrade: when it beats the starter, and the source for the claim.
- A short written defence: why this model, what it assumes, where it breaks, what you
  would do with 10× more data.
- One reproducibility command that regenerates every number.

**Acceptance:** the report's claims are checkable with the command. No metric comes from
training data alone. The model choice follows a validation number, not a preference.

## Datasets

Use small, local, fast data. Loading a checked-in CSV or generating synthetic data is fine;
using an ML crate's estimators inside model code is not.

Choose the capstone dataset and its problem type before build 1. Record the choice in
`NOTES.md`. Choose one small set for each other phase.

| Phase | Suggested data |
| --- | --- |
| Regression | California housing (subset), or a synthetic nonlinear function |
| Classification | Breast cancer, wine, or a synthetic two-moons set |
| Clustering | Iris without labels, synthetic blobs, moons, concentric circles |
| Text | SMS spam collection, 20 newsgroups (2 categories) |

## Rules for upgrades

- Never start the upgrade until the starter is `verified` and has a recorded baseline.
- If you implement the upgrade, use the same interface. The comparison must be fair.
- One new idea per upgrade. If you cannot name the single idea, the scope is too big.
- The upgrade must earn its complexity with a measured gain, or be documented as a
  negative result.
- For an upgrade that you explain only, cite the source. Do not report a number that you
  did not measure.
