# Study elements

A study element is the unit of work. It is one model, one concept, one gate. The template is
in `TEMPLATE.md`. The core path is in `../reference/curriculum.html`.

## How to use an element

1. Read the diagnostic. Answer it closed book.
2. If the diagnostic fails, add a prerequisite unit. Do not start the element.
3. Read the primary source. Stop at the stated point.
4. Work the concept lesson. Restate the idea in your own words.
5. Write the reference card. Compress the definition, formula, and failure mode.
6. Write the acceptance test. Compute the cases by hand first.
7. Write the code from a blank file.
8. Answer the explain-back without notes.

## Status

| Part | Status |
| --- | --- |
| Diagnostic | `todo` |
| Concept lesson + reference card | `reading` |
| Build | `implemented` |
| Acceptance test | `verified` |
| Explain-back | `explained` |

A failed diagnostic or gate adds a prerequisite element. Record it in `../reference/progress.html` under
**Gap log**. The main element waits. The path changes.

## File names

The number is the build ID. The core path uses `0000`, `0001`, `0002`, `0005`, `0008`,
`0011`, and `0013`. The gaps show the optional builds.

## What lives where

| Artifact | Path | Purpose |
| --- | --- | --- |
| Contract | `study/{ID}-{slug}.md` | What to study and what must be true. |
| Lesson | `../lessons/{ID}-{slug}.html` | The teaching for one idea. |
| Reference card | `../reference/{ID}-{slug}.html` | The compressed form. |
| Code | `src/` or the build file | The learner's implementation. |

The contract is stable. The lesson and the card start when the element starts.
