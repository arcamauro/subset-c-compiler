# subset-c-compiler
Compiler for a subset of C programming language. Made in Rust.

Since this project currently follows the building of a compiler of a *subset* of the C programming language, I choose to stay simple and choose a limited number of functionalities, structures and types, specifically:
- int, char and string literals
- #include and #define preprocessor directives, delegated to gcc's own preprocessor rather than handled by the lexer/parser
- return, if-else, for, while, do while, switch-case-default
- operators, both logical and mathematical
- comments, both inline and block comments

I'll probably add more functionalities in the future, such as static arrays or other data type like float, double, long, unsigned, signed and sized types

---
## Preprocessor

Real C preprocessing (macro expansion, conditionals, and correctly resolving `#include`/`#define`, which are only terminated by a newline the lexer doesn't track) is a different concern from parsing the language's grammar, so instead of reimplementing it, this stage is delegated to gcc's own preprocessor.

### Used Approach
Before the source ever reaches the Lexer, it's piped through:
```
gcc -E -P -x c -
```
- `-E`: stop after preprocessing, don't compile;
- `-P`: inhibit line markers, since the Lexer doesn't need source line/file bookkeeping;
- `-x c -`: force the input, read from stdin, to be treated as C source.

By the time the Lexer sees the result, every `#include` and `#define` has already been resolved or expanded, comments are stripped, and no `#` character remains in the stream.

### Caveat
Since this calls the *real* gcc preprocessor, `#include <stdio.h>` doesn't just disappear: it pastes in the full contents of the actual system header, which is far beyond this project's subset grammar. For now, test programs avoid including real system headers and simply call external functions like `printf` without declaring them, since there's no type-checking yet.

---
## Lexer

### Implemented Tokens

**Keywords**: `Int`, `Char`, `Return`, `If`, `Else`, `While`, `Do`, `For`, `Switch`, `Case`, `DefaultCase`, `Break`, `Continue`

**Punctuation**: `OpenParentheses`, `ClosedParentheses`, `OpenBracket`, `ClosedBracket`, `Semicolon`, `Colon`, `Comma`

**Operators** (`Operator(String)`): `+`, `-`, `*`, `/`, `%`, `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `++`, `--`, `+=`, `-=`

**Literals**: `IntegerLiteral(i32)`, `StringLiteral(String)`, `CharLiteral(char)`

**Other**: `Identifier(String)`

### Used Approach
The lexer was engineered following a **Finite State Automaton (FSA)** approach, which consists of the following states:

- Start: starting state of the automaton;
- InIdentifier: state reached if we find an identifier in the Start state;
- InNumber: state reached if we find a number in the Start state;
- InString: state reached if we see that a string is starting in the Start state, so if the lexer sees a double quotation (");
- InChar: state reached if we see that a char is starting in the Start state, so if the lexer sees a single quotation (');
- InLineComment: state reached if we start the string with a slash and another slash after (//) in the Start state;
- InBlockComment: state reached if we start the string with a slash and an asterisk after (/*) in the Start state.

### Example
```c
int main() {
    int a = 10;
    int b = 11;

    for(int i = 0; i < 10; i++) {
        printf("Hello world\n");
        a += 2;
        b -= 1;
    }
    return 0;
}
```

Result:
```
Int
Identifier("main")
OpenParentheses
ClosedParentheses
OpenBracket
Int
Identifier("a")
Operator("=")
IntegerLiteral(10)
Semicolon
Int
Identifier("b")
Operator("=")
IntegerLiteral(11)
Semicolon
For
OpenParentheses
Int
Identifier("i")
Operator("=")
IntegerLiteral(0)
Semicolon
Identifier("i")
Operator("<")
IntegerLiteral(10)
Semicolon
Identifier("i")
Operator("++")
ClosedParentheses
OpenBracket
Identifier("printf")
OpenParentheses
StringLiteral("Hello world\n")
ClosedParentheses
Semicolon
Identifier("a")
Operator("+=")
IntegerLiteral(2)
Semicolon
Identifier("b")
Operator("-=")
IntegerLiteral(1)
Semicolon
ClosedBracket
Return
IntegerLiteral(0)
Semicolon
ClosedBracket
```
---
## Parser
The parser was implemented following an **LL(1)** parser structure

### Implemented Grammar

**Expressions**: assignment (`=`, `+=`, `-=`), logical and (`&&`), equality (`==`, `!=`), relational (`<`, `<=`, `>`, `>=`), additive (`+`, `-`), multiplicative (`*`, `/`, `%`), unary (`-`, `!`, prefix `++`/`--`), postfix (`++`, `--`), and primary expressions (literals, identifiers, function calls, parenthesized expressions)

**Statements**: variable declarations, `return`, `if`-`else`, `while`, `do`-`while`, `for`, `switch`-`case`-`default`, expression statements, and braced blocks

**Items**: functions, with typed parameters and a statement body

### Used Approach
The parser follows a **recursive descent** structure driven by a single token of lookahead, matching the grammar's LL(1) nature: every parsing function peeks at the next token *before* consuming it, and only advances once it knows which rule applies, never consuming speculatively.

Expressions are parsed through a **precedence climbing chain**, where each function calls the next-higher-precedence one and only combines its own operators once it already has a left-hand operand:

```
parse_expression (assignment)
  -> parse_logical_and (&&)
    -> parse_equality (==, !=)
      -> parse_relational (<, <=, >, >=)
        -> parse_additive (+, -)
          -> parse_multiplicative (*, /, %)
            -> parse_unary (-, !, prefix ++/--)
              -> parse_postfix (postfix ++/--)
                -> parse_primary (literals, identifiers, calls, grouping)
```

Statements are parsed through a single dispatcher, `parse_statement`, which peeks at the next token and routes to the matching rule (`parse_if_stmt`, `parse_while_stmt`, `parse_for_stmt`, etc.), each of which consumes its own keyword, delimiters, and, where present, trailing semicolon.

### Example
```c
int main() {
    int a = 10;
    int b = 11;

    for(int i = 0; i < 10; i++) {
        printf("Hello world\n");
        a += 2;
        b -= 1;
    }
    return 0;
}
```

Result:
```
Program {
    items: [
        Function(
            Function {
                ret_type: Int,
                name: "main",
                params: [],
                body: [
                    VarDeclaration {
                        varType: Int,
                        name: "a",
                        value: Some(
                            IntegerLit(
                                10,
                            ),
                        ),
                    },
                    VarDeclaration {
                        varType: Int,
                        name: "b",
                        value: Some(
                            IntegerLit(
                                11,
                            ),
                        ),
                    },
                    For {
                        init: Some(
                            VarDeclaration {
                                varType: Int,
                                name: "i",
                                value: Some(
                                    IntegerLit(
                                        0,
                                    ),
                                ),
                            },
                        ),
                        condition: BinaryOp {
                            left: Identifier(
                                "i",
                            ),
                            op: "<",
                            right: IntegerLit(
                                10,
                            ),
                        },
                        update: Some(
                            UnaryOp {
                                op: "++",
                                expr: Identifier(
                                    "i",
                                ),
                            },
                        ),
                        body: Block(
                            [
                                Expr(
                                    FunCall {
                                        callee: "printf",
                                        args: [
                                            StringLit(
                                                "Hello world\n",
                                            ),
                                        ],
                                    },
                                ),
                                Expr(
                                    Assignment {
                                        target: Identifier(
                                            "a",
                                        ),
                                        op: "+=",
                                        value: IntegerLit(
                                            2,
                                        ),
                                    },
                                ),
                                Expr(
                                    Assignment {
                                        target: Identifier(
                                            "b",
                                        ),
                                        op: "-=",
                                        value: IntegerLit(
                                            1,
                                        ),
                                    },
                                ),
                            ],
                        ),
                    },
                    ReturnStmt(
                        Some(
                            IntegerLit(
                                0,
                            ),
                        ),
                    ),
                ],
            },
        ),
    ],
}
```
