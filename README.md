# Brainfuck with stack and functions
Optimised Brainfuck interpreter with stack and functions capabilities written in Rust.

## Using the Interpreter
**Compile** all the files with `cargo build --release`.

**Run** with `cargo run [filename(s)]`, where the optional files are scripts containing Brainfuck code.

After the files are compiled and executed, you will be presented with an inline interpreter where you can continue inserting code, load more files, save the code you write in new files. Some script examples are found in `hello_world`, `loadtest`, `funtest` and `stack_test`.

## Base Language and New Syntax
The standard operators can be found at [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck).

**Debugging** information:
* `#`  : prints the non-zero values in the array, the position of the `pointer` and the number of functions defined

**Stack** information:

Operator | Usage
:---:|:---:
`@`|Adds the number at the top of the stack to the current location
`^`|Removes the top element and adds it to the current location
`&`|Pushes the element at the current location into the stack
`?`|Sets the int at the current position to the length of the stack

Everything uses the same stack, including functions called on a separate array(see next part).

Peeking and removing from an empty stack will result in the program crashing.

Examples can be found in `stack_test`. The file has to be loaded on an empty array.

Declaring and calling **functions**:

Operator | Usage 
:--------:|:---:
`~`|Begin/end a function
`\|`|Call the function with the number at the `pointer`
`/` | Same as `\|`, but moves to a new array, copies function number
`\\`| Used only after`/`
`!`| Inserts the total number of functions at runtime in the position
`!!`| Inserts the number of functions compiled when the symbol is compiled

Functions cannot be declared in loops. Functions can call other functions.

For each `\\` after a `/` a number from the original array is copied to the new one.

`!` allows easy access to the current last functions anywhere in a file.
Using `!!` in a function inserts the number that the function will get after compiling.

At the end of a function called on a new array that array is discarded.

Examples can be found in the files provided. To run those files, the compile order is: `hello_world`, `loadtest`, `funtest`.

## Interpreter Features
**Compiling:** the scripts are parsed, functions are created and the operations are stored in a queue waiting to execute. Files which fail to compile don't make any changes to the existing array or number of functions.

**Running:** operations are executed in the order in which they were inserted in the queue. There are options to time the run, but not the compile.

**Reading characters:** characters are sent to the program only after typing the `\n` character. The input received contains the `\n` character. 

**Features:** all arrays are infinite-dimensional* and support negative points. The stack is infinite-dimensional*. The number of functions is uncapped*. 

*Depends on the available memory.

**Performance improvements:** 
* Multiple operations of the same kind are compiled together into a single operation. For example `+++++>>>---` is stored as `Add(5)`, `Move(3)`, `Add(-3)`. Loops and function calls are not stored together: `++++[-->>]` compiles to `Add(4)`, `While[Add(-2), Move(2)]`.
* Similar operations one after another are combined:

Script | Compiled | Optimised
:---:|:---:|:---:
`+++--`|`[Add(3), Add(-2)]`|`[Add(1)]`
`++--`|`[Add(2), Add(-2)]`|`[]`
`>><<<`|`[Move(2), Move(-3)]`|`[Move(-1)]`
`++!!--`|`[Add(2), Set(x*), Add(-2)]`|`[Set(x-2)]`
`!!++!!`|`[Set(x*), Add(2), Set(x*)]`|`[Set(x)]`

*The value of `x` is determined at compile-time, and thus a `Set` operation is used.

Inserting the length of the stack is treated as a special `Set` command and treated accordingly.
* Empty loops are ignored (`[]` or `[<>]`).
* `[+]` and `[-]` are compiled to `Set(0)`.
* Loops whose operations reduce to a single Set operations are reduced to `Set(0)` (otherwise they are infinite loops -
not expected and easier to track while debugging)
* Loops that move to right or left are compiled to the same operation, but hardcoded in the compiler:

  `[>]` to `SkipMove(1)`, `[><<<]` to `SkipMove(-2)`
  
  This does not seem like a huge improvement, but in practice it reduced the run time of the Fractal Viewer by 15%.
* Loops that move the value of a cell in others are compiled in a single operation:

Script | Compiled | Optimised
:---:|:---:|:---:
`[->+<]`|`While[Add(-1), Move(1), Add(1), Move(-1)]`|`AddTo*[(1,1)]`
`[->++<]`|You get the idea|`AddTo[(1,2)]`
`[->+>--<<]`|_____________|`AddTo[(1,1),(2,-2)]`
`[->+<-<+>+]`|_____________|`AddTo[(1,1), (-1, 1)]`

*Basically, `AddTo` has a list of offsets and how many times it has to add the value at the source to it. The complexity is linear in the length of the list.

Since the stack implementations, those operations can be replaced with easier/more robust code with almost the same time complexity.
* Peek commands are ignored if followed by a Set command.

* Multiple Pop commands are executed together.

* Copying: while there is no pattern recognition for copying, due to the functionality provided by the stack there is no need of improving it.

Current running time of the Fractal Viewer (`test_file`): 10.5s. (just the standard one, does not use functions/stack)
## Command Line Interpreter
The command line interpreter will continue to ask for input until all functions/loops are closed.
Input can contain all types of characters which will be ignored later.

Special commands have to be alone on a line:

 Command | Function
 :---:|:---:
 `:t` | Toggles the timer for execution
 `:l [filename(s)]` | Loads, compiles and executes scripts in order
 `:s [file]` | Records the next commands for saving to file
 `:s` | Saves the recorded commands to the previous file
 `:q` | Exits the program
 
 ## Standard Library
 I started building a standard library with a few useful functions and will soon upload it.