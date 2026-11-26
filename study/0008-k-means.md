# 0008 — k-means

**Status:** `todo`
**Primary source:** ISL §12.4.1, stop at §12.4.2
**Prerequisite:** element 0. Distance and mean. Checkpoint C4.

## 1. Why this element exists

This is the first unsupervised model. It groups points without labels. It has no gradient
and no likelihood. It minimizes a distance objective by alternating two steps.

## 2. Diagnostic

The diagnostic tests the prerequisite, not the new concept.

- **Question:** Compute the squared Euclidean distance between `(1,2)` and `(4,6)`. Compute
  the mean of `(1,1)`, `(3,5)`, and `(5,0)`. State why the mean is the point that minimizes
  the sum of squared distances.
- **Pass:** the distance and the mean are correct, and the mean is named as the minimizer.
- **Fail:** add a prerequisite unit on distance and mean from `../reference/math-map.html` track A, §1.2.

## 3. Concept lesson

- **Idea:** k-means minimizes the sum of squared distances from each point to its assigned
  centroid. Assignment and centroid update each decrease the objective. The pair of steps
  reaches a local minimum.
- **Numeric example:** six points on a line, `k = 2`. Start with centroids at 0 and 10.
  Compute one assignment step and one update step by hand.
- **Notation:** `C₁, …, Cₖ` are the clusters. `μⱼ` is the centroid of `Cⱼ`.
  `J = Σⱼ Σ_{x ∈ Cⱼ} ‖x − μⱼ‖²`.
- **Source section:** ISL §12.4.1.

## 4. Theory

The learner derives this.

- **Objective:** minimize `J`.
- **Assumption:** the clusters are roughly spherical and have similar size. The features are
  scaled.
- **Derivation:** the assignment step minimizes `J` for fixed centroids. The update step sets
  each centroid to the mean of its cluster. Both steps cannot increase `J`. The sequence is
  finite, so it terminates.
- **Failure mode:** a bad initialization gives a bad local minimum. An empty cluster has no
  mean. The result depends on the scale of the features.

## 5. Reference card

- **Definition:** k-means partitions points into `k` clusters and represents each cluster by
  its mean.
- **Formula:** `J = Σⱼ Σ_{x ∈ Cⱼ} ‖x − μⱼ‖²`. `μⱼ = mean(Cⱼ)`.
- **Failure mode:** local minima, empty clusters, unscaled features.

## 6. Build contract

- **Input:** `X` (`n × d`), `k`, seed, number of restarts.
- **Output:** labels (`n × 1`), centroids (`k × d`), inertia history.
- **Shapes:** one label per row. One centroid per cluster.
- **Error cases:** `k > n` must return an error. An empty cluster must be handled explicitly.
- **No ML library:** the distance, the assignment, the update, and the initialization are
  hand-written.

## 7. Acceptance test

- **Invariants:** inertia does not increase at any iteration. Two restarts on the same seed
  agree exactly.
- **Oracle or closed form:** recovers three well-separated blobs with adjusted Rand index >
  0.99.
- **Degenerate cases:** duplicate points. `k = 1`. `k = n`. An empty cluster.
- **Tolerance:** exact agreement for the same seed. 0.99 for the blob check.

## 8. Explain-back

- **Question:** Which objective does k-means minimize, and why does Lloyd's algorithm never
  increase it?
- **Pass:** the answer names the objective and gives the two-step argument.

## 9. Sources

- **Primary:** ISL §12.4.1.
- **Math:** `../reference/math-map.html` track C, checkpoint C4.
- **Oracle:** hand-computed 6-point example. Optional reference implementation in tests only.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
