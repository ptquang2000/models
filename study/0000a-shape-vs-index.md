# 0000a — Shape is a count, an index is a position

**Status:** `verified`
**Primary source:** The Rust Programming Language, ch. 8.1, *Storing Lists of Values with
Vectors* — `len`, indexing, and the out-of-bounds panic.
**Added by:** gap-fill rule, 2026-11-27. Prerequisite for element 0.

## 1. Why this unit exists

The element 0 diagnostic asked for the shape of a 3×2 `Vec<Vec<f64>>`. The answer given was
`m.len() = i` and `m[0].len() = j`. That answer mixes two different things:

- an **extent** — how many entries exist, a `usize` count,
- an **index** — where one entry sits, a value in `0 .. extent`.

A shape is a pair of extents. A function argument like `i` is an index. They cannot be equal
in general, and code that confuses them reads the wrong row or panics.

## 2. Diagnostic

Answer without notes, in numbers, not symbols:

1. `let m = vec![vec![0.; 2]; 3];` — how many entries in total?
2. `m.len()` — what number?
3. `m[0].len()` — what number?
4. `m[2][1]` — which row, which column?
5. Which index runs `0 .. 2`, and which runs `0 .. 1`?

**Pass:** five numbers, and the words "row" and "column" used for the right slot.

## 3. Concept

A 2×3 matrix has 2 rows and 3 columns, so 6 entries. The rows are `m[0]` and `m[1]`. The
columns of row 0 are `m[0][0]`, `m[0][1]`, `m[0][2]`.

| question | answer |
| --- | --- |
| total entries | 2 × 3 = 6 |
| `m.len()` | 2 (rows) |
| `m[0].len()` | 3 (columns) |
| valid row index | 0, 1 |
| valid column index | 0, 1, 2 |

The last valid index is always `extent − 1`. That is why a length and an index differ by one,
and why the length is a count, not a position.

## 4. Source

- **Primary:** TRPL ch. 8.1, `Vec<T>` — `len`, indexing, out-of-bounds panic.
- **Why it matters here:** the harness states every shape as `n × d`, and every loop bound is
  an extent. The split code in element 0 is built from extents.

## 5. Gate

Answer the five diagnostic questions in numbers. Then return to element 0 and answer the
original shape question for a 3×2 matrix.

## 6. Gap log

| Date | Gap | Unit added | Status |
| --- | --- | --- | --- |
| 2026-11-27 | Extent confused with index; `m.len()` answered as `i` | This unit | `reading` |
