# Brainfuck_w_funs
Brainfuck interpreter with functions written in Rust.

## Using the Interpreter
**Compile** all the files with `cargo build --release`.

**Run** with `cargo run [filename(s)]`, where the optional files are scripts containing Brainfuck code.

After the files are compiled and executed, you will be presented with an inline interpreter where you can continue inserting code, load more files, save the code you write in new files. Some script examples are found in `hello_world`, `loadtest` and `funtest`. They have to be run in this order.

## Base Language and New Syntax
The standard operators can be foud at [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)

**Debugging** information:
* `#`  : prints the non-zero values in the array, the position of the `pointer` and the number of functions defined

Declaring and calling **functions**:

Operator | Use 
:--------:|:---:
`~`|Begin/end a function
`\|`|Call the function with the number at the `pointer`
`/` | Same as `\|`, but moves to a new array, copies function number
`\\`| Used only after`/`
`!`| Inserts the total number of functions at runtime in the position
`!!`| Inserts the number of functions compiled when the symbol is compiled

Functions cannot be declared in loops. Functions can call other functions.

For each '\\' after a '/' a number from the original array is copied to the new one.

`!` allows easy access to the last functions.
Using `!!` in a function inserts the number that the function will get after compiling.

At the end of a function called on a new array, the number at the last position of the `pointer` in that array is moved to the position from where the function was called in the old array. The new array is discarded.

Examples can be found in the files provided. To run those files, the compile order is: `hello_world`.

## Interpreter Features
**Compiling:** the scripts are parsed, functions are created and the operations are stored in a queue waiting to execute. Files which fail to compile don't make any changes to the existing array or number of functions.

**Running:** operations are executed in the order in which they were inserted in the queue. There are options to time the run, but not the compile.

**Reading characters:** characters are sent to the program only after typing the `\n` character. The input received contains the `\n` character. 

**Features:** all arrays are infinite-dimensional and support negative points. The number of functions is uncapped. 

**Performance improvement:** 
* Multiple operations of the same kind are compiled together into a single operation. For example `+++++>>>---` is stored as `add 5`, `move right 3`, `subs 3`. Loops and function calls are not stored together: `++++[-->>]` compiles to `add 4`, `loop(subs 2, move right 2)`.
* TODO: add more.

## Command Line Interpreter
The command line interpreter will continue to ask for input untill all functions/loops are closed.
Input can contain all types of characters which will be ignored later.

Special commands have to be alone on a line:

 Command | Function
 :---:|:---:
 `:t` | Toggles the timer for execution
 `:l [filename(s)]` | Loads, compiles and executes scripts in order
 `:s [file]` | Records the next commands (except debugging) for saving to file
 `:s` | Saves the recorded commands to a file
 `:q` | Exits the program
 
 ## Standard Library
 I started building a standard library with a few useful functions and included it in Code Examples. Updates to the library will be included as they are made.

