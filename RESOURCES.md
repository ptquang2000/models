# Resources

Curated, annotated, and short on purpose. Knowledge for explanations comes from these
sources, other high-trust material, or the primary literature — never from memory. Five
sharp sources beat thirty mediocre ones.

**How to use:** one primary source per topic. Watch or read it once, then do the assigned
problems or build the thing. Do not collect resources as a substitute for either. Any
source not listed here is allowed if it is high-trust; append it below with one line on
what it is for.

## Local

- `docs/Ed 5, Gilbert Strang - Introduction to Linear Algebra (2016, Wellesley-Cambridge
  Press).pdf`
  The linear algebra reference. Use for: chapters 1–4, 6–7, 12 as mapped in `reference/math-map.html`.
- `docs/Solutions Manual for Introduction to Linear Algebra Fifth Edition by Gilbert
  Strang.pdf`
  Answers to the problem sets. Use only to check a real attempt; reading first does not
  count as practice.
- `docs/ISLR_First_Printing.pdf`
  James et al., *An Introduction to Statistical Learning*, 2nd ed., first printing (R).
  The reading track in `reference/curriculum.html`. §2.2 verified: printed pp. 29–41 = PDF
  pp. 42–54; §2.3 R lab starts PDF p. 55.
- `docs/james2023islp.pdf`
  James et al., *An Introduction to Statistical Learning with Applications in Python*
  (ISLP, 2023). Same text as the R printing, different pagination. The learner reads from
  this one. §2.2 verified: printed pp. 27–39 = PDF pp. 37–49 (offset +10); 2.2.1 p. 28,
  2.2.2 p. 31, 2.2.3 p. 34; §2.3 Python lab starts printed p. 40. Subtract 2 from any
  printed page number in `reference/curriculum.html` / `reference/math-map.html` when reading this edition.

## Knowledge

### Programming

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
  The language reference. Use for: the programming prerequisite repair unit, chapters 3–4,
  8, and 11 only. The learner is strong. Read only the gap.

### Linear algebra

- [MIT 18.06, Strang — video lectures and exams](https://ocw.mit.edu/courses/18-06-linear-algebra-spring-2010/)
  The course that matches the book. Use for: a second pass on any section that did not
  land, and the exams for mixed practice after a track.
- [3Blue1Brown — Essence of Linear Algebra](https://www.3blue1brown.com/topics/linear-algebra)
  Visual intuition for span, rank, determinants, eigenvectors, and the SVD. Use for:
  building the picture before the algebra, not as a substitute for problems.
- [Deisenroth, Faisal, Ong — Mathematics for Machine Learning (free PDF)](https://mml-book.github.io/)
  The bridge text. Use for: the ML framing of linear algebra, calculus, and probability,
  especially chapters 2–5 and 7. Better than Strang when you need the ML connection stated
  explicitly.

### Machine learning

- [James et al. — An Introduction to Statistical Learning (free PDF)](https://www.statlearning.com/)
  The best first ML book. Use for: model intuition, bias–variance, resampling, trees,
  boosting, SVM, unsupervised learning. Read the chapter before building the model.
- [Hastie, Tibshirani, Friedman — The Elements of Statistical Learning](https://hastie.su.domains/ElemStatLearn/)
  The reference for trees, boosting, and clustering. Denser than ISL. Use for: the chapter
  that explains why the algorithm is what it is, after you have built it once.
- [Stanford CS229 lecture notes](https://cs229.stanford.edu/main_notes.pdf)
  Concise derivations: least squares, logistic regression, exponential families, GDA.
  Use for: the math behind a model when ISL is too hand-wavy.
- [scikit-learn user guide](https://scikit-learn.org/stable/)
  Not a tutorial. Use for: API expectations, metric definitions, and as the oracle in
  tests. Never copy an implementation from it into model code.
- [Jurafsky & Martin — Speech and Language Processing, 3rd ed. draft](https://web.stanford.edu/~jurafsky/slp3/)
  Use for: vector semantics — tf-idf, cosine similarity, sparsity. Chapter 6 only, before
  the text build.

### Probability, calculus, optimization

- [Blitzstein — Stat 110 course site](https://projects.iq.harvard.edu/stat110)
  Probability intuition, conditional probability, Bayes, expectation, variance. Use for:
  the probability foundations before logistic regression and naive Bayes. The site may
  block scripted requests; open it in a browser.
- [3Blue1Brown — Essence of Calculus](https://www.3blue1brown.com/topics/calculus)
  Derivatives and chain rule, visually. Use for: a fast refresher, then do derivative
  drills elsewhere.
- [MIT 18.01SC — Single Variable Calculus](https://ocw.mit.edu/courses/18-01sc-single-variable-calculus-fall-2010/)
  Use for: chain rule, implicit differentiation, and optimization problems when you need
  reps rather than intuition.
- [Boyd & Vandenberghe — Convex Optimization (free PDF)](https://web.stanford.edu/~boyd/cvxbook/)
  Use for: convex sets and functions, gradients, and the justification for gradient
  descent. Chapters 1–5 only, and only the parts the current model needs.
- [MacKay — Information Theory, Inference, and Learning Algorithms (free PDF)](https://www.inference.org.uk/itila/book.html)
  Use for: entropy, KL divergence, and the Bayesian view, when you want depth beyond
  ISL's treatment of impurity measures.

### Papers and deep dives

- [Chen & Guestrin — XGBoost: A Scalable Tree Boosting System (2016)](https://arxiv.org/abs/1603.02754)
  The primary source for the regularized objective and the gain formula. Use for: phase 4
  and 7, after a plain gradient-boosted tree works.
- [Arthur & Vassilvitskii — k-means++ (2007)](https://theory.stanford.edu/~sergei/papers/kMeansPP-soda.pdf)
  The initialization that makes k-means reliable. Use for: phase 8.
- [Ester et al. — A Density-Based Algorithm for Discovering Clusters (DBSCAN, 1996)](https://www.aaai.org/Papers/KDD/1996/KDD96-037.pdf)
  The original definition of core, border, and noise points. Use for: phase 10.
- [Distill — Why Momentum Really Works](https://distill.pub/2017/momentum/)
  Interactive explanation of conditioning and momentum. Use for: when gradient descent
  crawls and you want to understand why.

## Gaps

- **Programming prerequisite repair.** The path tests Rust skill but does not teach it. Use
  the Rust book, chapters 3–4, 8, and 11, only if the diagnostic fails.
- **Capstone dataset.** Not chosen. The dataset gate is before element 1.

## Wisdom (communities)

Communities are where you test whether you can defend a choice. Use them sparingly and
with a specific question, not "is this good?".

- [Cross Validated](https://stats.stackexchange.com/)
  High-signal statistics and ML Q&A. Use for: "is my derivation right?", metric selection,
  and diagnosing a suspicious result. Search before asking.
- [r/learnmachinelearning](https://www.reddit.com/r/learnmachinelearning/)
  Beginner-friendly, mixed quality. Use for: implementation sanity checks and study
  planning, not for authoritative answers.
- [Kaggle discussions](https://www.kaggle.com/discussions)
  Use for: seeing how practitioners frame a tabular problem, feature engineering habits,
  and public notebooks to compare against — after your own baseline exists.

The user has not opted into communities yet. Do not push them; offer once, then drop it.

## Adding sources

- Anything high-trust is fair game: textbooks, lecture notes, papers, documentation,
  well-moderated forums, a practitioner's write-up. The list above is a starting point, not
  a whitelist.
- Append what you actually used, in the right group, with one line: what it covers and when
  to reach for it. A bare link is useless in three months.
- If a source turns out to be wrong or shallow, remove it rather than leaving it buried.
- No calculus, probability, optimization, or dataset text is required upfront. Pull in a
  source when the current model needs it, and only as deep as the model needs.
