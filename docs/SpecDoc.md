# Specification Document

## Keywords

Keywords in Maple, for now only have strict application, i.e. they can only be used in their correct contexts. They cannot be used as names of:
* Items
* Variables and function parameters
* Loop Labels

```
 Lexer:
    KwTree:    tree    (class declaration)
    KwBranch:  branch  (method declaration)
    KwGround:  ground  (public method/class)
    KwFinal:   final   (constant declaration)
    KwVar:     var     (variable declaration)
    KwLoop:    loop    (loop initialiser)
    KwIf:      if      (if keyword)
    KwElse:    else    (else keyword)
    KwRef:     ref     (pass by reference)
    KwFalse:   false   (the static value of boolean false)
    KwTrue:    true    (the static value of boolean true)
```

## Identifiers

An identifier is any nonempty ASCII strings of the form:

Either:
* The first character is a letter.
* The remaining characters are alphanumeric or `_`.

Or
* The first character is `_`.
* The identifier is more than one character. `_` alone is not an identifier.
* The remaining characters are alphanumeric or `_`.

###### Defence

The choice of strictly ASCII strings is because it lets the developer be creative and informative with their
variable names without having to deal with UTF-8/UTF-16 codepoints and multi-byte characters in our parser and compiler,
which is taxing on the CPU as well as the virtual environment.

## Literals

Literals in Maple are anOperatorExpressionconsisting of a single token. This immediately denotes the value it evaluates to,
instead of referring to it by name or some other rule. It is a form of constant expression.

```
Character:
    Accepted characters: All ASCII
    Escapes: Quote, ALL ASCII whitespaces and null values
String:
    Accepted characters: ALL ASCII
    Escapes: Quote, ALL ASCII whitespaces and null values

Numbers:
    Decimals:
        Accepted: [0-9, _ (for visual seperation)]+
    Hex integers:
        Accepted: 0x[0-F, _ (for visual seperation)]+
    Octal integers:
        Accepted: 0o[0-7, _ (for visual seperation)]+
    Binary Integers:
        Accepted: 0b[0-1, _ (for visual seperation)]+
    Real numbers:
        Accepted: [0-F]+ (e|E|p|P) [0-F]+
        Conditionals: if opening integer is hex, P must be used to seperate exponent
                      if opening integer is decimal, E must be used to seperate exponent
Boolean:
    true | false
```

###### Defence
* Character: Technically Extended-ASCII (8-bit ASCII) is accepted, 
  mostly to simplify the design of the language while simultaneously allowing varied expression
* String: Technically Extended-ASCII (8-bit ASCII) is accepted, which allows easy usage of this language with the
  ISO Latin 1 standard (ISO 8859-1)
* Decimals: Contain numbers that are represented in base10. The addition of the Visual Separator
  was so that long numbers are readable
* Hex: Contains numbers represented in base16.
* Octal: Contains numbers represented in base8.
* Binary: Contains numbers represented in base2.

  All these types can be understood because those are the most used formats in computing.

## Operators
Operators are defined for built in types by the Maple language. For now, they cannot be overloaded.

```
Limited to integer and real literals or variables
Mathematical Operators:
    Binary Operators:
        Addition: (+)
        Subtraction: (-)
        Multiplication: (*)
        Division: (/)
        Modulo: (%)
    Unary Operators:
        Increment: (++)
        Decrement: (--)
        Mathematical Negation: (-)
    
Limited to Conditional expressions and boolean variables are values
Logical Operators:
    Binary Operators:
        Logical AND: (&&)
        Logical OR: (||)
    Unary Operators:
        Logical NOT: (`)
Conditional Operators:
    Binary Operators:
        Equivalence: (==)
        NotEqual: (`=)
        GreaterThan: (>)
        LessThan: (<)
        GreaterThanOrEqual: (>=)
        LessThanOrEqual: (<=)
```

# EBNF Rules

## Literals

**Literal Expressions**
```
OperatorExpression:
    CHAR_LITERAL |
    STRING_LITERAL |
    INTEGER_LITERAL | 
    FLOAT_LITERAL |
    BOOLEAN_LITERAL
```

**Character Literals**
```
CHAR_LITERAL:
    '(ASCII - [', \, \n, \r, \t] | QUOTE_ESCAPE | ASCII_ESCAPE)'
ASCII:
    Set of all Extended ASCII characters
QUOTE_ESCAPE:
    \' | \"
ASCII_ESCAPE:
    \x OCT_DIGIT HEX_DIGIT
    | \n | \r | \t | \\ | \0
```


**String Literals**
```
STRING_LITERAL:
    "( 
        ASCII - [", \, Isolated CR] |
        QUOTE_ESCAPE |
        ASCII_ESCAPE |
        STRING_CONTINUE
      )*"
STRING_CONTINUE:
    \ followed by \n
```

**Integer Literals**
```
INTEGER_LITERAL:
    (DEC_LITERAL | BIN_LITERAL | OCT_LITERAL | HEX_LITERAL)

DEC_LITERAL:
    DEC_DIGIT(DEC_DIGIT | _)*

BIN_LITERAL:
    0b (BIN_DIGIT | _)* BIN_DIGIT (BIN_DIGIT | _)*

OCT_LITERAL:
    0o (OCT_DIGIT | _)* OCT_DIGIT (OCT_DIGIT | _)*

HEX_LITERAL:
    0x (HEX_DIGIT | _)* HEX_DIGIT (HEX_DIGIT | _)*

BIN_DIGIT: [0-1]
OCT_DIGIT: [0-7]
DEC_DIGIT: [0-9]
HEX_DIGIT: [0-9 a-f A-F]
```

**Boolean Literals**
```
BOOLEAN_LITERALS:
    true | false
```

**Identifier Literals**
```
IDENTIFIER_LITERALS:
    (a - z)+ 
        [
            ASCII - [", \, Isolated CR]
        ]* 
    (a - z)+
```

## Expressions

**Operator Expressions**
```
OperatorExpression:
    ArithmeticOrLogicalExpression |
    ComparisonExpression |
    LazyBooleanExpression |
    AssignmentExpression 
```

**Arithmetic Or Logical Expression**
```
ArithmeticOrLogicalExpression:
    OperatorExpression "+" OperatorExpression |
    OperatorExpression "-" OperatorExpression |
    OperatorExpression "*" OperatorExpression |
    OperatorExpression "/" OperatorExpression |
    OperatorExpression "%" OperatorExpression
```
###### Semantics
A arithmetic or logical expression is evaluated only when the left and right expressions
have either arithmetic or logical value. That is, they consist of an expression that
can either be evaluated to a boolean or an integer.

**Comparison Expression**
```
ComparisonExpresion:
   OperatorExpression "==" OperatorExpression |
   OperatorExpression "`=" OperatorExpression |
   OperatorExpression ">" OperatorExpression |
   OperatorExpression "<" OperatorExpression |
   OperatorExpression ">=" OperatorExpression |
   OperatorExpression "<=" OperatorExpression 
```
###### Semantics
A Comparison expression is evaluated only when the left and right expressions can be compared
to each other. That is, they either have the same type OR have an implicit type conversion.

**Lazy Boolean Expression**
```
LazyBooleanExpression:
    OperatorExpression "||" OperatorExpression |
    OperatorExpression "&&" OperatorExpression
```


**Assignment Expression**
```
AssignemntExpression:
    (IDENTIFIER_LITERAL | OperatorExpression) "=" (IDENTIFIER_LITERAL | OperatorExpression)
```
###### Semantics
An assignment statement consists of a place expression followed by an equals sign and
a value expression.