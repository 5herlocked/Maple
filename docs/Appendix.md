# Lexical Structure

## Keywords

Keywords in Maple, for now only have strict application, i.e. they can only be used in their correct contexts. They cannot be used as names of:
* Items
* Variables and function parameters
* Loop Labels

```
 Lexer:
    KW_TREE:    tree    (class declaration)
    KW_BRANCH:  branch  (method declaration)
    KW_GROUND:  ground  (public method/class)
    KW_FINAL:   final   (constant declaration)
    KW_VAR:     var     (variable declaration)
    KW_LOOP:    loop    (loop initialiser)
    KW_IF:      if      (if keyword)
    KW_ELSE:    else    (else keyword)
    KW_REF:     ref     (pass by reference)
```

## Identifiers

An identifier is a nonempty ASCII string of the form:

Either:
* The first character is a letter.
* The remaining characters are alphanumeric or `_`.

Or
* The first character is `_`.
* The identifier is more than one character. `_` alone is not an identifier.
* The remaining characters are alphanumeric or `_`.