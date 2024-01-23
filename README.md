# Calculus
Calculus is a very simplistic **calculator interpreter** capable to perform basic aritmethic operations respecting the respective precedence and associativity rules for these operations.

It has been developed as proof of my (**very little**) understanding about **Rust**, **lexical analysis** and **syntactic analysis**.

# Requirements
- **+1.74.0** rustc version
- **+1.74.0** cargo version

# Modules
The whole application is built over three main modules: [Repl](#repl), [Tokenizer](#tokenizer) and [AST](#ast). 

## Repl
Contains few functions in order to run the **repl** loop, allowing the input reading from terminal and passing the input to the [Tokenizer](##tokenizer) and [AST](##ast) modules.

## Tokenizer
It contains the core functions for transforming the input characters stream into a stream of **Tokens**. **Tokens** are a intermediate representation of the user's input that eases the parsing process later.

As long as it is a super (and really super) simplistic interpreter, just exists two kind of tokens:
- `Number`: literally the numbers representation. All numbers are coerced to `f64`.
- `Operator`: represents the available math operators. Currently, supported operators are:
  - `+`: for addition
  - `-`: for substraction and numbers negation
  - `*`: for multiplication
  - `/`: for division

## AST
It is the responsible for take the stream of generated **Tokens** and relate them in order to build the corresponding **Abstract Syntax Tree** for the entered math expression. It contains the set of functions for build the relationships between tokens based on the following  production rules:
- `Program -> (Term)*`
- `Term -> Factor (("+" | "-") Factor)*`
- `Factor -> Unary (("*" | "/") Unary)*`
- `Unary -> "-" Literal | Literal ` 
- `Literal -> NUMBER`

Also, it is the responsible for take the build tree and evaluate the expressions in order to compute the final result.
