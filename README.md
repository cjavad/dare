# dare

Rust implementation of a tableau parser for logical expressions and generation of math output.

## Features

- [ ] Parse logical expressions and evaluate their result
- [ ] Parse logical expressions and evaluate them to a truth table or truth tree
- [ ] Generate LaTeX/Markdown output for the truth table or truth tree
- [ ] Reformat, simplify and process inputted expressions to a standard form
- [ ] Understand new definitions and update truth evaluations accordingly on the fly
- [ ] Solve generic logical expressions (Ex. sudoku)

## Definition of logical expression

A logical expression consists of atomic propositions, connectives, and parentheses. The atomic propositions are the letters of the alphabet. The connectives are the following: `¬` (negation), `∧` (conjunction), `∨` (disjunction), `→` (implication), `↔` (equivalence), and `⊕` (exclusive or). The parentheses are used to group expressions. The following are examples of logical expressions:

- `p`
- `¬p`
- `p ∧ q`
- `p ∨ q`
- `p → q`
- `p ↔ q`
- `p ⊕ q`
- `(p → q) ∧ (q → r)`
- `p → (q → r)`
- `p → q → r`

Atomic propositions are binary logical expressions, so they are either true or false. The connectives are use binary logic but the negation connective is unary meaning it only takes one operand (argument) unlike the rest which take a right hand and left hand operand. The parentheses are used to group expressions. There is no reading order and the operators are evaluated in terms of precedence as defined below.

### Logical expressions / operations

A logical expression is a string of characters that can be parsed into a tableau. The following characters are allowed:

From [https://en.wikipedia.org/wiki/List_of_logic_symbols](https://en.wikipedia.org/wiki/List_of_logic_symbols) and listed in order of precedence of symbols for individual characters and listed in order of precedence of operations:

- (1) Parentheses (Seperates subexpressions)
  - ``(, )``
- (2) Assignment (Assigns a value to a proposition or logical expression)
  - `:`, `:=`
- (3) Negation (NOT)
  - `¬`, `~`, `!`
- (4) Conjunction (AND)
  - `∧`, `&`, `&&`, `.`
- (5) Disjunction (OR)
  - `∨`, `|`, `||`
- (6) Exclusive Disjunction (XOR)
  - `⊕`, `⊻`, `^`
- (7) Implication (IF THEN)
  - `→`, `->`, `⇒`, `⊃`
- (8) Equivalence (IF AND ONLY IF)
  - `↔`, `<->`, `==`, `⇔`, `≡`
- (9) Tautology (True)
  - `T`, `1`, `⊤`, `■`
- (9) Contradiction (False)
  - `F`, `0`, `⊥`, `□`
- (10) Atomic Propositions (Variables)
  - Any valid variable name. (To be defined)

In the case multiple connectives of same precendence are used in the same expression, the precedence of the connectives is based on parentheses. For example, `p ∧ q ∨ r` needs to be specficied to `(p ∧ q) ∨ r` or `p ∧ (q ∨ r)`. Otherwise the grammer is incorrect.

### Non logical operations (to be defined)

- Definition (Assigment)
  - `≡`, `:=`, `:⇔`
- Proves
  - `⊨`
- Proven
  - `⊢`

We allow the use of multiple types of characters for the different operations to allow for different inputs styles and level of effort.

## Definition of truthtables for logical operations

### Implication

| A | B | `(A → B)` |
| - | - | --------- |
| T | T | T         |
| T | F | F         |
| F | T | T         |
| F | F | T         |

### Equivalence

| A | B | `(A ↔ B)` |
| - | - | --------- |
| T | T | T         |
| T | F | F         |
| F | T | F         |
| F | F | T         |

### Negation

| A | `(¬A)` |
| - | ------ |
| T | F      |
| F | T      |

### Conjunction

| A | B | `(A ∧ B)` |
| - | - | --------- |
| T | T | T         |
| T | F | F         |
| F | T | F         |
| F | F | F         |

### Disjunction

| A | B | `(A ∨ B)` |
| - | - | --------- |
| T | T | T         |
| T | F | T         |
| F | T | T         |
| F | F | F         |

### Exclusive Disjunction

| A | B | `(A ⊕ B)` |
| - | - | ---------- |
| T | T | F          |
| T | F | T          |
| F | T | T          |
| F | F | F          |

### Tautology

| A | `(T)` |
| - | ----- |
| T | T     |
| F | T     |

### Contradiction

| A | `(F)` |
| - | ----- |
| T | F     |
| F | F     |

## Behaviour of associative operations (#8)

We solve this issue by first defining a "strict mode". In "strict mode" all non-associative operations are required to be seperated individually by parentheses.

Ex.

`a <-> b <-> c`

Is required to be either:

`(a <-> b) <-> c` or `a <-> (b <-> c)`

In non-strict mode (which can be enabled via a argument) we default to the left-to-right reading so we parse the original expression as `(a <-> b) <-> c`. Otherwise the program will terminate with an grammar error.

For other non-assosiative chained operations we first order them by precedence so for instance `~a | b & c` will with this defintion of precedence from Wikipedia
![image](https://user-images.githubusercontent.com/22474016/189505924-9cba4abe-736e-4572-8dbb-1a7f7e4da2e9.png)
turn into `(~a) | (b & c)`.

But with our traditional definition of precedence where the conjunction and disconjuction (and hereby the Exclusive Disconjunction) that defines them as having the same precedence, the previous rules regarding strict and right-to-left are applied so the above expression becomes `((~a) | b) & c` unless otherwise specified. In the case of strict mode the program will terminate with an grammar error as the explicit parentheses needs to be defined.

## Parsing and evaluation (sub-expressions)

Given the expression in the following format:

```apl
((A → B) ∧ (B → C))
```

We can parse the expression into a tree of sub-expressions. The following is the tree of sub-expressions for the above expression:

```apl
((A → B) ∧ (B → C))

((A → B) ∧ (B → C))
  |         |
(A → B)   (B → C)
    |         |
    A         B
    |         |
    →         →
    |         |
    B         C
```

An example of a token representation of the above tree is:

```apl
[
  [
    [
      "A"
    ]
    "→"
    [
      "B"
    ]
  ]
  "∧"
  [
    [
      "B"
    ]
    "→"
    [
      "C"
    ]
  ]
]
```

Using the concept of Right Hand operand and Left Hand operand, we can evaluate the sub-expressions. For unary connectives only the right hand operand is valid. For binary connectives both operands are valid. For atomic propositions, the value of the proposition is used.

## License

As stated in LICENSE, this project is dual-licensed under the MIT license or the Apache 2.0 license as seen in LICENSE.

Copyright (C) 2022 Contributors as seen in CONTRIBUTORS.md
