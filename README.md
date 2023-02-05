# Simple Scripting Language (SSL)

One of the project that I hope to undertake soon is writing a simple programing language.

## Scope

- Variables/Literals `let a = 1`
- Integers, Booloean, Strings
- Block Based
- if/else: `if else`
- comparison operators: `== != > >= < <=`
- boolean operators: `|| && !`
- mathematical operators: `+ - / * ** %`
- print: `print`

## Example

```rust
let a = 1;
if a ** 2 == 4 {
  print a;
} else {
  print false;
}
```

## Abstract Syntax Tree

- statement
  - Assignment
    - variable: `num`
    - value: `1`
- if statement
  - condition
    - binary operator
      - op: `comparsion`
      - a
        - binary operator
          - op: `exponent`
          - a
            - Variable: `num`
          - b
            - Literal: `2`
      - b
        - Literal: `4`
  - branch
    - statement
      - print
        - variable: `num`
  - else
    - statement
      - print
        - Literal: `false`
