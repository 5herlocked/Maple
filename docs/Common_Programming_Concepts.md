# Common Programming Concepts

This document cover concept that appear in most mainstream programming languages and
how they work in Maple. There is a lot of overlap between the cores mainstream
 of programming languages. We'll be discussing these concepts in the context of Maple.
 Specifically, we'll learn about variables, basic types, functions, comments and
 control flow. These foundations will be in every Maple program and learning these,
 will give you a strong core to start from.
 
 ### Keywords
 Maple has a set of keywords that are reserved for use by the language only.
 You cannot use these words as names of variables or functions. Most of them have
 special meanings, and you'll be using them to do various tasks in your Maple programs.
 You can find a list of them in the [Appendix](SpecDoc.md).
 
 # Variables and Mutability
 Variables in Maple are mutable by default. However, once they're declared, their type
 cannot change. For example,
 ```
tree Variables {
    branch main() {
        var x = 5;
        print("The value of x is: {}", x); // fine
        x = 6;
        print("The value of x is: {}", x); // fine
        x = "Seven"; // attempting to change an immutable
        print("The value of x is: {}", x); // not fine
    }
}
```

### Mutable Type Variables
Variables can have mutable type when declared although, whenever the type changes,
it needs to be explicitly declared. For example,
```
let x: int = 5;
x = 6; // standard reassignment statement
x: string = "Seven"; // mutating type
```
Although possible, it is mostly advised to minimise mutable type variables.