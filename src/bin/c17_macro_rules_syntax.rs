// MACROS
// Macros look like functions, except their name ends with `!`.
// Instead of generating a function call, macros are expanded into source code that gets compiled with the rest
// of the program. 

// Macros are created using the macro_rules! macro
// a simple macro to say hello
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => (
        // The macro will expand into the contents of this block.
        println!("Hello");
    )
}



// SYNTAX: there are 3 basic syntax ideas: patterns and designators, overloading and repitition
// DESIGNATORS
// The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator

// This macro takes an argument of designator ident and creates a function named $func_name
// The ident designator is used for variable/function names
macro_rules! create_function {
    ($func_name:ident) => (
    fn $func_name() {
        println!("You called {:?}()", stringify!($func_name));
    }
    )
}

create_function!(foo); // create functions named foo and bar with the above macro
create_function!(bar);

macro_rules! print_result { // this macro takes an expression and prints it as a string with its result
    ($expression:expr) => ( // the expr designator is used for expressions
    println!("{:?} = {:?}", stringify!($expression), $expression);
    ) // stringify will convert the expression as it is into a string
}

// a list of all designators is here: https://doc.rust-lang.org/rust-by-example/macros/designators.html
// block, expr, ident, item, pat, path, stmt, tt, ty, vis


// OVERLOAD
// Macros can be overloaded to accept different combinations of arguments. 
// In that regard, macro_rules! can work similarly to a match block
// test! will compare $left and $right in different ways depending on how you invoke it.
macro_rules! test {
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}", 
                stringify!($left),
                stringify!($right),
                $left && $right)
    ); // each arm must end with a `;`
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}", 
                stringify!($left),
                stringify!($right),
                $left || $right)
    );
}

// REPEAT
// Macros can use + in the argument list to indicate that an argument may repeat at least once, or * to indicate
// that the argument may repeat zero or more times.
// `$(...),+ will match one or more expression, separated by commas.

macro_rules! find_min { // will calculate the min of any number of arguments
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    );
}

// Here, the main function above macro declaration is giving errors
fn main() {
    say_hello!();

    // SYNTAX
    // DESIGNATORS
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({ // a block is also an expression
        let x = 1u32;

        x * x + 2 * x - 1
    });

    // OVERLOAD
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    // REPEAT
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

