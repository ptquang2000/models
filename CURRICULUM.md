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

## Phase 0 — Lab harness

Not a model, but nothing else is measurable without it.

- Deterministic train/test split, seeded.
- Standardization with `fit` on train only and `transform` applied to both splits.
- Metrics: MSE, RMSE, MAE, R², accuracy, precision, recall, F1, confusion matrix.
- Finite-difference gradient checker.

**Acceptance:** every metric matches a reference library on a fixed vector to 1e-12; a
deliberately leaky scaler test fails; the gradient checker flags a wrong gradient.

## Phase 1 — Predict a number

### 1. Linear regression, closed form

- **Math:** projection onto a column space, normal equations, why AᵀA is invertible for
  full column rank (Strang 4.1–4.3).
- **Build:** `fit(X, y)` solving AᵀAx = Aᵀb; `predict`; intercept handling; optional L2 via
  a modified normal-equations system.
- **Fundamentals:** least squares as orthogonal projection, residual orthogonality,
  rank and multicollinearity, R² vs. RMSE.
- **Acceptance:** agrees with the gradient-descent version to 1e-6; residual `e = y − ŷ` is
  orthogonal to the columns of X to 1e-10; singular X raises a clear error instead of
  returning garbage.
- **Stretch:** weighted least squares; compare `solve` vs. explicit inverse for
  conditioning.

### 2. Linear regression, gradient descent

- **Math:** derivative of ‖Xw − y‖², learning rate, convexity.
- **Build:** batch gradient descent with a loss history; optional mini-batch.
- **Fundamentals:** why GD is needed for models without a closed form, convergence vs.
  step size, feature scaling's effect on conditioning, epoch vs. iteration.
- **Acceptance:** loss is non-increasing at a small enough step; gradient check passes;
  final weights match the closed form on standardized data.
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
  weights optional; stable sigmoid.
- **Fundamentals:** MLE vs. least squares, cross-entropy, why the loss is convex, decision
  threshold vs. ranking, calibration, imbalanced classes.
- **Acceptance:** gradient check passes; loss decreases monotonically; predicted
  probabilities sum correctly per row and lie in (0, 1); accuracy matches a reference
  implementation within 1% on a small standardized dataset; decision boundary plotted for
  two features.
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
- **Acceptance:** probabilities sum to 1; adding a constant to every count does not change
  predictions; beats the majority-class baseline by a wide margin on SMS spam; a
  hand-computed 2-word example matches.
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
- Learning curves and one failure case per model.
- A short written defence: why this model, what it assumes, where it breaks, what you
  would do with 10× more data.
- One reproducibility command that regenerates every number.

**Acceptance:** the report's claims can be checked by running the command; no metric is
reported on training data alone; the model choice is justified with a validation number,
not a preference.

## Datasets

Use small, local, fast data. Loading a checked-in CSV or generating synthetic data is fine;
using an ML crate's estimators inside model code is not.

| Phase | Suggested data |
| --- | --- |
| Regression | California housing (subset), or a synthetic nonlinear function |
| Classification | Breast cancer, wine, or a synthetic two-moons set |
| Clustering | Iris without labels, synthetic blobs, moons, concentric circles |
| Text | SMS spam collection, 20 newsgroups (2 categories) |

## Rules for upgrades

- Never start the upgrade until the starter is `verified` and has a recorded baseline.
- Implement the upgrade from the same interface, so the comparison is fair.
- One new idea per upgrade. If you cannot name the single idea, the scope is too big.
- The upgrade must earn its complexity with a measured gain, or be documented as a
  negative result.
