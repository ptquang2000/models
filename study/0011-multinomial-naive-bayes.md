# 0011 — Multinomial naive Bayes

**Status:** `todo`
**Primary source:** ISL §4.4, the naive Bayes part, stop at §4.5
**Prerequisite:** element 0. MATH-MAP track B §12.2 and track D §12.2.

## 1. Why this element exists

This is the first text model. It classifies documents by counting words. It uses Bayes'
theorem and a strong independence assumption. The assumption is false in most text, but the
classifier still works well enough to set a baseline.

## 2. Diagnostic

The diagnostic tests the probability prerequisite, not the new concept.

- **Question:** State Bayes' theorem for two classes. Use a 2×2 count table to compute
  `P(A|B)`. State what conditional independence means in one sentence.
- **Pass:** the theorem and the conditional probability are correct. Independence is stated
  as a product of conditionals.
- **Fail:** add a prerequisite unit from `../reference/math-map.html` track B, §12.2.

## 3. Concept lesson

- **Idea:** classify a document by the class that gives the highest posterior probability.
  Assume the words are independent given the class. The product of word probabilities
  becomes a sum of log-probabilities.
- **Numeric example:** two classes, a two-word vocabulary, six documents. Compute the priors
  and the conditional counts by hand. Classify one new document with and without smoothing.
- **Notation:** `P(c | d) ∝ P(c) Πᵢ P(wᵢ | c)`. In log space:
  `log P(c) + Σᵢ log P(wᵢ | c)`.
- **Source section:** ISL §4.4, the naive Bayes part.

## 4. Theory

The learner derives this.

- **Objective:** maximize the posterior probability of the class given the document.
- **Assumption:** the words are conditionally independent given the class.
- **Derivation:** start from Bayes' theorem. Apply the independence assumption. Take the log.
  Add Laplace smoothing to avoid a zero count.
- **Failure mode:** a word that never appears in a class gives probability zero. Smoothing
  fixes this. The independence assumption is wrong, so the probabilities are not calibrated.

## 5. Reference card

- **Definition:** multinomial naive Bayes models a document as a bag of words and classifies
  it by the posterior class probability under conditional independence.
- **Formula:** `log P(c | d) = log P(c) + Σᵢ log P(wᵢ | c) + const`.
- **Failure mode:** zero counts, unseen words, false independence, uncalibrated probabilities.

## 6. Build contract

- **Input:** a list of documents, a list of labels, a smoothing constant.
- **Output:** a vocabulary, class priors, conditional log-probabilities,
  `predict_proba(doc) -> p`, `predict(doc) -> label`.
- **Shapes:** one prior per class. One conditional probability per word and class.
- **Error cases:** an empty document must return the prior. A word outside the vocabulary
  must be ignored or mapped to a stated token.
- **No ML library:** the tokenizer, the counts, the smoothing, and the log-space computation
  are hand-written.

## 7. Acceptance test

- **Invariants:** the class probabilities sum to 1. Adding a constant to every count does not
  change the predicted class.
- **Oracle or closed form:** a hand-computed 2-word example matches.
- **Degenerate cases:** one class. An empty document. A word that appears in one class only.
- **Tolerance:** 1e-12 for the hand-computed example.
- **Baseline:** F1 on the validation set beats the majority-class baseline.

## 8. Explain-back

- **Question:** State Bayes' theorem and name the naive Bayes assumption. Why does the
  classifier still work when the assumption is false?
- **Pass:** the answer states the assumption and separates classification accuracy from
  probability calibration.

## 9. Sources

- **Primary:** ISL §4.4.
- **Math:** Strang §12.2. `../reference/math-map.html` track D.
- **Oracle:** hand-computed 2-word example. Optional reference implementation in tests only.

## 10. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| — | — | — | — |
