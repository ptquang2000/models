# Mission: machine learning from scratch

## Why

I want to be able to look at a dataset, name the problem type, and build the model that
fits it — with my own code and my own understanding of why it works. Libraries should be
a convenience I choose, not a black box I depend on. The math is the bottleneck I am
fixing: I tend to over-study and do every textbook exercise, which burns time without
producing understanding.

## Success looks like

- Implement linear regression, logistic regression, k-means, and naive Bayes from a blank
  file, from memory, without an ML library.
- Derive the normal equations from the projection picture, and the logistic gradient from
  the likelihood, on paper, without notes.
- Explain the upgrade paths — random forest, XGBoost, hierarchical clustering, DBSCAN,
  TF-IDF + logistic regression — and when each one beats its starter model.
- Take one dataset end to end: split, baseline, train, evaluate, diagnose, compare, and
  defend the choice with numbers rather than vibes.
- Read the linear algebra I actually need, at the depth the model needs it, not the whole
  book.

## Constraints

- Self-directed and deadline-free. Work continues until each gate in `CURRICULUM.md` is
  met, not until a date or a session count.
- Stack: Rust, built and tested with cargo. Array library and support crates are the
  learner's choice. Model code uses no ML crate; a reference crate is allowed in tests only.
- The linear algebra text on hand is Strang, *Introduction to Linear Algebra*, 5th ed.
  (`docs/`). It is the spine of the math track, not the only permitted source.
- Math study is scope-bound, not time-bound: the assigned subset of problems is the unit of
  work, and finishing that subset is the only stopping rule. Completing a chapter is not a
  goal; using a section is.
- No GPUs assumed. Everything must run on a laptop CPU in seconds.

## Out of scope (for now)

- Deep learning and neural networks.
- Distributed training, big-data tooling, MLOps, serving.
- Kaggle leaderboard optimization.
- Implementing papers that need GPU-scale compute.
- Calculus, probability, and optimization from cover to cover — only the pieces the
  models require.

## Open questions

- Which array library or numeric approach in Rust?
- Which dataset domains are interesting enough to sustain the capstone?
