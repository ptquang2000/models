# 0001 — Matrix assembly: rows vs columns

**Date:** 2026-11-23
**Context:** Strang §1.3, problem 1.

Given `s₁ = (1,1,1)`, `s₂ = (0,1,1)`, `s₃ = (0,0,1)` and the instruction that they are the
columns of `S`, the matrix was assembled with those vectors as *rows*, and
`x₁(1,0,0) + x₂(1,1,0) + x₃(1,1,1)` was written as `Sx`. That expression is `Sᵀx`; at
`x = (3,4,5)` it gives `(12,9,5)`, not `b = (3,7,12)`. The error survived one round of
"it matches b" without computation. A 2×2 worked example (`a₁ = (1,0)`, `a₂ = (2,1)`) plus
the rule `A[i][j]` = `i`-th component of column `j` fixed it; the rows were then produced
correctly in one attempt.

**Why it changes what to teach next:** orientation of a matrix is now a known weak point,
not a hypothetical one. Every later artifact with columns as features (`X` in `Xw`, the
design matrix for the normal equations, the vocabulary matrix for TF-IDF) inherits the same
risk. From here, any `fit(X, y)` review checks the shape and the column meaning of `X`
before the math. The habit to build is: state the shape and say what each axis means
*before* multiplying.

A second, related observation: answers were verified by assertion rather than by
computation ("match b", "independent"). Cheap numeric checks — substitute two points and
compare — are the correction, and should be demanded rather than offered.
