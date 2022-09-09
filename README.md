# dare

Rust implementation of a tableau parser for logical expressions and generation of math output.

## Definition of logical expression

A logical expression is a string of characters that can be parsed into a tableau. The following characters are allowed:

From [https://en.wikipedia.org/wiki/List_of_logic_symbols](https://en.wikipedia.org/wiki/List_of_logic_symbols)

- Parentheses
  - `(`, `)`
- Negation
  - `¬`, `~`, `!`
- Conjunction
  - `∧`, `.`, `&`, `&&`
- Disjunction
  - `∨`, `|`, `||`
- Implication
  - `⇒`, `→`, `⊃`, `->`
- Equivalence
  - `⇔` `≡`, `↔`, `<->`, `==`
- Excluse Disjunction (XOR)
  - `↮`, `⊕`, `⊻`, `≢`, `+`
- Tautology (True)
  - `⊤`, `T`, `1`, `■`
- Contradiction (False)
  - `⊥`, `F`, `0`, `□`
- Definition
  - `:⇔`, `≡`, `:=`
- Proves
  - `⊨`
- Proven
  - `⊢`

We allow the use of multiple types of characters for the different operations to allow for different inputs styles and level of effort.

## License

As stated in LICENSE, this project is dual-licensed under the MIT license or the Apache 2.0 license as seen in LICENSE.

Copyright (C) 2022 Contributors as seen in CONTRIBUTORS.md
