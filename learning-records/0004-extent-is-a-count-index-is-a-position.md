# LR-0004: An extent is a count, an index is a position

**Date:** 2026-11-27
**Context:** Element 0 programming diagnostic, shape statement for a 3×2 `Vec<Vec<f64>>`.

Three answers in a row mixed the two:

1. `m.len() = i`, `m[0].len() = j` — the index variables returned as lengths.
2. `m.len() = i * j` — the outer length read as the total entry count.
3. `m.len()` = "row count", `m[0].len()` = "column count" — correct, in words.

The correction that landed was not a definition. It was the split of the structure: `m` holds
one inner vector per row, so `len()` on `m` counts rows; `m[0]` holds the values of one row, so
`len()` on it counts columns. The total is the product of the two lengths, not either one.
The worked 2×3 table in `study/0000a-shape-vs-index.md` §3 supplied the pattern; the drill was
the same table with rows and columns swapped.

**Why it changes what to teach next:** the split element is built from extents — `n`, `d`, and
three index-vector lengths. The answer came as a rule ("row count"), not as two numbers, which
is the stronger form and the one to reuse. It also repeats the pattern from §1.3: the row and
column roles are the fragile part, not the arithmetic. Any later shape claim gets stated in
words before it is written in code.
