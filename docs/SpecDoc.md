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
variable names without having to deal with UTF-8/UTF-16 codepoints and multi-byte characters in our parser and compiler
which is for the most part very taxing on the CPU as well as the virtual environment.

## Literals

Literals in Maple are an expression consisting of a single token. This immediately denotes the value it evaluates to,
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
        Accepted: [0-9, _ (for visual seperation)]
    Hex integers:
        Accepted: 0x[0-F, _ (for visual seperation)]+
    Octal integers:
        Accepted: 0o[0-7, _ (for visual seperation)]+
    Binary Integers:
        Accepted: 0b[0-1, _ (for visual seperation)]+
    Real numbers:
        Accepted: [0-F](e|E|p|P)[0-F]+
        Conditionals: if opening integer is hex, P must be used to seperate exponent
                      if opening integer is decimal, E must be used to seperate exponent
```

###### Defence
* Character: Mostly to simplify the design of the language while simultaneously allowing varied expression
* String: Technically Extended-ASCII (8-bit ASCII) is accepted which allows for usage of this language with ease up
to the ISO Latin 1 standard (ISO 8859-1)
* Decimals: Contain numbers that are represented in base10 and is only limited by that. The addition of the Visual Seperator
was so that long numbers are human readable
* Hex: Contains numbers represented in base16.
* Octal: Contains numbers represented in base8.
* Binary: Contains numbers represented in base2.
    
    All these types can be understood because those are the most used formats in computing.

## Operators


# EBNF Rules

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