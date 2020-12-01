# Lexical Structure

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