# Math map — Strang, *Introduction to Linear Algebra*, 5th ed.

Source of record: `docs/Ed 5, Gilbert Strang - Introduction to Linear Algebra (2016,
Wellesley-Cambridge Press).pdf`
Answers: `docs/Solutions Manual for Introduction to Linear Algebra Fifth Edition by
Gilbert Strang.pdf`

Every section and problem number below was checked against those PDFs. If a number here
disagrees with the book, the book wins — fix this file.

## Rules of engagement

1. **Tiers are decisions, not suggestions.** MUST = read and solve. SKIM = read the
   summary and one worked example. SKIP = do not open it.
2. **Work the assigned subset to completion** — roughly 30–40% of each problem set, chosen
   by concept. That subset is the definition of done; the unassigned problems do not exist.
   This is a scope limit, not a time limit.
3. **Pick in this order:** conceptual first problems → problems marked
   *Recommended*/*Important* → one problem that connects to the model you are building.
   Ignore the rest.
4. **Skip drills.** Long elimination by hand, cofactor expansions, Cramer's rule, and
   determinant arithmetic teach typing, not structure. Do one example by hand, then verify
   in code.
5. **Solutions manual only after a real attempt.** If you read the answer first, the
   problem does not count. Mark it `demo` in `PROGRESS.md`.
6. **One section at a time.** Read the summary at the top, then the examples, then the
   assigned problems. Do not read the whole chapter "to be safe".
7. **Interleave.** Alternate math and coding on the model that needs it. Math without a use
   decays fast.

## Tracks

| Track | Start it when | Sections | Done when |
| --- | --- | --- | --- |
| A. Least squares | Before linear regression | 1.1–1.3, 2.1–2.2, 2.5–2.6, 3.1–3.5, 4.1–4.3 | checkpoints A1–A4 answered |
| B. Likelihood | Before logistic regression | 12.1–12.2 + calculus refresher | checkpoints B1–B2 answered |
| C. Eigen and SVD | Before k-means + PCA | 6.1–6.2, 6.4–6.5, 7.1–7.3 | checkpoints C1–C4 answered |
| D. Text | Before naive Bayes | 1.2 (cosine), 12.2 (Bayes), 3.4 (sparsity) | text model verified + explained |
| E. Rigor (optional) | After the starter models work | 4.4, 5.1, 7.4, 11.1–11.2 | as needed |

Tracks A and B can overlap: do the probability sections between the math and the coding
for logistic regression.

## Track A — least squares

| § | Read | Problems | Why it matters |
| --- | --- | --- | --- |
| 1.1 | all | 1, 19, 25 | A feature matrix is a set of column vectors; predictions are linear combinations. 19 shows the cone of nonnegative combinations (convexity); 25 shows span collapse (rank). |
| 1.2 | all | 1, 2, 3, 8, 12 | Dot product = similarity; length = norm; 12 is the projection formula in disguise. |
| 1.3 | all | 1, 4, 6 | `Ax` as a combination of columns; dependence via `Wx = 0`; which `c` makes columns dependent. |
| 2.1 | all | 2, 5, 7 | Row picture vs. column picture. 5–7 are the singular/no-solution cases behind rank deficiency. |
| 2.2 | all | 4, 5, 7 | Pivots, permanent vs. temporary breakdown, singular systems. Skip the 2×2 drilling in 1–3. |
| 2.5 | concept only | 3 problems: 2×2 inverse formula, invertible vs. singular, one Gauss–Jordan by hand | Inverses exist only for full rank; you will almost never invert a matrix in code. |
| 2.6 | summary + one example | 1, 3 | `A = LU` is what `solve` does. Know it exists; do not practice 3×3 factorization. |
| 3.1 | 9–18 only | 4 of 9–18 | Subspace tests. **Skip 1–8** (axiom checking). Column space and nullspace are the point. |
| 3.2 | all | 2, 5, 10 | Nullspace, free variables, rank. This is why one-hot columns and collinear features cause trouble. |
| 3.3 | all | 1, 5 | Complete solution `x = x_p + x_n`; solvability conditions on `b`. |
| 3.4 | all | 2, 5, 7 | Independence, basis, dimension. 7 (differences of independent vectors are dependent) builds rank intuition. |
| 3.5 | all | 1, 2, 4 | Dimensions of the four subspaces; the Fundamental Theorem. Read this twice. |
| 4.1 | all | 1, 7, 9, 10 | Orthogonality of the four subspaces. 9 proves `AᵀA` has the same nullspace as `A` — the reason normal equations work. |
| 4.2 | all | 1, 9, 13, 17, 21, 27 | Projections onto lines and subspaces, `P = A(AᵀA)⁻¹Aᵀ`, `P² = P`, and `I − P` as the residual maker. This is linear regression. |
| 4.3 | all | 1, 4, 12, 14, 21, 23 | Normal equations, the calculus derivation, the mean/variance link, covariance of the estimate, and which subspace the error lives in. **The core of the track.** |

After 4.2–4.3 you should be able to derive `AᵀAx = Aᵀb` three ways: geometry
(perpendicular residual), calculus (set the gradient to zero), and algebra (least squares
normal equations).

## Track B — likelihood and probability

| § | Read | Problems | Why it matters |
| --- | --- | --- | --- |
| 12.1 | all | 1, 6, 7 | Mean, variance, sample vs. expected, why the sample variance divides by N − 1. 6 drills it; 7 proves the identity. |
| 12.2 | all | 2, 4, 8, 9 | Covariance, independence, product rule, and Bayes' theorem. Problem 9 is naive Bayes in one line. |
| Calculus refresher | derivative rules, chain rule, partial derivatives, `d/dx log x` | 10–15 short exercises from an external source | Logistic regression is impossible without the chain rule. See Beyond this book. |

You do **not** need measure theory, moment generating functions, or the multivariate
Gaussian (12.3) for the four starter models.

## Track C — eigenvalues, SVD, PCA

| § | Read | Problems | Why it matters |
| --- | --- | --- | --- |
| 6.1 | all | 1, 2, 3, 8 | `Ax = λx`, trace and determinant checks, how to find λ given `x` and vice versa. |
| 6.2 | all | 3–4 problems: diagonalization, powers, why `Aᵏ → 0` needs `|λ| < 1` | Diagonalization is the cleanest view of repeated matrix application. |
| 6.4 | all | 4, 6, 13, 14 | Symmetric matrices, orthogonal diagonalization, spectral theorem, projection decomposition. 4 (`AᵀCA` symmetric) is the covariance-matrix pattern. |
| 6.5 | all | 1, 3, 4, 7, 10 | Positive definiteness, `xᵀSx`, pivots vs. eigenvalues, and why `AᵀA` is positive definite for independent columns. 7 is directly about `AᵀA`. |
| 7.1 | all | 2–3 problems on the image-compression example | The SVD as rank-ordered importance. Read the pictures, not the proofs. |
| 7.2 | all | 2, 6, 7, 13 | `A = UΣVᵀ`, `AᵀA` vs. `AAᵀ`, singular values, and the inverse via SVD. |
| 7.3 | all | 1, 5, 7 | Centering, covariance matrix, PCA by SVD, correlation vs. covariance. This is the clustering/dimensionality bridge. |

Skip 6.3 (differential equations) and 7.4 (pseudoinverse) on the first pass. 7.4 is worth
one separate pass when you care about rank-deficient least squares.

## Track D — text

| § | Read | Problems | Why it matters |
| --- | --- | --- | --- |
| 1.2 | review | 3, 7 | Cosine similarity is normalized dot product. |
| 12.2 | review | 8, 9 | Bayes and conditional probability, in the notation naive Bayes uses. |
| 3.4 | review | 5 | Independence in high dimensions; most token columns are nearly dependent. |

Naive Bayes needs no new linear algebra. TF-IDF + logistic regression needs only
normalization and sparse dot products.

## Track E — optional rigor

| § | Why later |
| --- | --- |
| 4.4 Gram–Schmidt / QR | Numerical stability of least squares; read after you have felt `AᵀA` lose precision. |
| 5.1 Determinants | Only `det A = 0` ⟺ singular, and volume scaling. Skip 5.2–5.3 entirely. |
| 7.4 Pseudoinverse | Minimum-norm solution for rank-deficient least squares. |
| 11.1–11.2 Norms and condition numbers | Why your normal equations are unstable with correlated features. |
| 11.3 Iterative methods | Only if you implement large-scale solvers. |

## Skip list

Do not open these during the core tracks. Each is real mathematics, none is needed for the
four problem types:

| Section | Why skip |
| --- | --- |
| 1.3 worked examples with cyclic difference matrices | Cute, not load-bearing. Read the concept, skip the matrices. |
| 2.3–2.4, 2.7 | Elimination matrices, block multiplication, permutations. Absorb by osmosis. |
| 3.1 problems 1–8 | Axiom checking. No model needs it. |
| 3.x challenge problems | Interesting, off-mission. |
| 5.2–5.3 | Cofactors, Cramer's rule, volumes. Formulas nobody computes. |
| 6.3 | Differential equations. Different course. |
| 8.x | Linear transformations and change of basis. Useful later for deep learning, not now. |
| 9.1–9.3 | Complex matrices, Hermitian, FFT. Only if you work with signals. |
| 10.2, 10.5–10.7 | Engineering matrices, Fourier series, graphics, cryptography. |
| 12.3 | Multivariate Gaussian. Needed for GMMs, not for these models. |

Chapter 10.1 (graphs), 10.3 (Markov), and 10.4 (linear programming) are worth a skim after
the capstone if you care about networks, ranking, or the geometry of SVMs.

## Beyond this book

Other sources are allowed, not just tolerated. Reach for them when they explain the idea
better than Strang, then record the addition in `RESOURCES.md`. Useful complements:

- **Calculus:** derivatives, chain rule, partials, gradients. MIT 18.01SC
  (<https://ocw.mit.edu/courses/18-01sc-single-variable-calculus-fall-2010/>) or
  3Blue1Brown's *Essence of Calculus*
  (<https://www.3blue1brown.com/topics/calculus>). Do 10–15 derivative exercises, not a
  course.
- **Probability:** Blitzstein's Stat 110
  (<https://projects.iq.harvard.edu/stat110>) for intuition; Strang 12 for the
  linear-algebra view of variance and covariance.
- **Optimization:** Boyd & Vandenberghe, *Convex Optimization*, chapters 1–5
  (<https://web.stanford.edu/~boyd/cvxbook/>). Read convex sets, convex functions, and
  gradient descent. Skip duality until SVMs.
- **Information theory:** entropy and information gain for trees. One read of ESL §9.2
  (<https://hastie.su.domains/ElemStatLearn/>) is enough.
- **Numerical linear algebra:** Trefethen & Bau, lectures 1–20, when `AᵀA` instability
  becomes a real problem in your code.

## Checkpoints

The TA should ask these before advancing a track. Answer without notes.

- **A1.** Why is `AᵀA` invertible when `A` has independent columns? (4.1.9)
- **A2.** Derive `AᵀAx = Aᵀb` from "the residual is perpendicular to the column space".
- **A3.** What does `(I − P)` project onto, and why is it the residual maker? (4.2.17)
- **A4.** Given `X` with two identical columns, what happens to the normal equations, and
  what should the code do?
- **B1.** Why does maximizing the likelihood of Bernoulli data give log-loss?
- **B2.** Write Bayes' theorem for two classes and say which term naive Bayes assumes away.
- **C1.** Why are eigenvalues of a symmetric matrix real, and why are its eigenvectors
  orthogonal? (6.4)
- **C2.** Why is `AᵀA` positive definite, and what does that buy you? (6.5.7)
- **C3.** What does the first singular vector of a centered data matrix point along? (7.3)
- **C4.** k-means minimizes which objective, and why does Lloyd's algorithm never increase
  it?

## Mixed practice (only after a track is done)

Textbook problem sets drill one section at a time. For exam-like mixing, use the 18.06
exams on OCW: <https://ocw.mit.edu/courses/18-06-linear-algebra-spring-2010/pages/exams/>.
Do one exam per completed track, open book. That is a better use of a study block than
finishing the leftover problems.
