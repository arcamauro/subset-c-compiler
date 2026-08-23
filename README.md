# subset-c-compiler
Compiler for a subset of C programming language. Made in Rust

*Disclaimer*: this codebase contains/will contain code that I will use for my bachelor thesis project, since I want to use this compiler for that scope. So I won't accept any PRs or external contributions until the end of my bachelor degree.

Since this project currently follows the building of a compiler of a *subset* of the C programming language, I choose to stay simple and choose a limited number of functionalities, structures and types, specifically:
- int, char and string literals
- #include and #define preprocessor directives
- return, if-else, for, while, do while, switch-case-default
- operators, both logical and mathematical
- comments, both inline and block comments

I'll probably add more functionalities in the future, such as static arrays or other data type like float, double, long, unsigned, signed and sized types

## Lexer

### Implemented Tokens

**Keywords**: `Int`, `Char`, `Return`, `If`, `Else`, `While`, `Do`, `For`, `Switch`, `Case`, `DefaultCase`, `Break`, `Continue`

**Preprocessor**: `Include`, `Define`

**Punctuation**: `OpenParentheses`, `ClosedParentheses`, `OpenBracket`, `ClosedBracket`, `Semicolon`, `Colon`

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
- InDirective: state reached if we start a string with the preprocessor directive character, so if the lexer sees an hash symbol (#)
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
