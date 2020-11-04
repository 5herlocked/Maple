# Hello Maple, World

Hello World is a rite of passage for most software developers. At some point, 
almost everyone has written a program that simply prints "Hello, World"

Which brings me to the title of this file. `Hello Maple, World` is an
introduction of Maple to the world.

```
tree Hello_World {
    branch main() {
        print("Hello, World");
    }
}
```

If "Hello, World" printed congratulations, you've officially written a Maple Program.

## Anatomy of a Maple Program

Let's review in detail what we did with the "Hello World" program.
The first piece of the puzzle is:
```
branch main() {

}
```
These lines define a function in Maple. The `main` function is special
it is always the first code that runs in every Maple program. The first line
declares a function named `main` with no parameters and returns nothing.
If there were parameters, they would go inside the parenthesis'.

A thing to note, the function body is wrapped in braces `{}`. Maple requires
these around all function bodies.

