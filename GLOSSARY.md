# Glossary

The canonical language of this repo. Definitions are compressed, opinionated, and written
in the user's words once they can use the term correctly.

**Rule:** a term enters only after an explain-back, not when it is first mentioned. If the
user has just been introduced to something, it is not ready.

## Format

```md
**Term**:
One or two sentences. Define what it is, not how to do it.
_Avoid_: loose synonyms used elsewhere
```

Use glossary terms inside other definitions. One preferred word per concept; list the
aliases to avoid. Revise definitions when understanding deepens — a week-one definition is
often wrong by week six.

## Terms

**Linear combination**:
A sum of scalar multiples of vectors, `c u + d v`. For two vectors, the set of all of them is a line when one is a multiple of the other and a plane otherwise; adding the restriction `c, d ≥ 0` shrinks it to a wedge or a ray.
_Avoid_: "linear compilation", "mix", "sum of vectors" (that is only the `c = d = 1` case)

**Column picture**:
Reading `Ax = b` as a combination `b = x₁a₁ + ⋯ + xₙaₙ` of the columns of `A`. The
alternative reading, `(row i)·x = bᵢ`, is the row picture; both produce the same `b`.
_Avoid_: "column view", "column form"

**Dependent columns**:
Columns are dependent when some nonzero combination of them is zero, so at least one of
them lies in the span of the others and the matrix is not invertible. Independent means the
only zero combination is the all-zero one.
_Avoid_: "dependent matrix", "linearly related"
