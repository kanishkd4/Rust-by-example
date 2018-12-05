// Closures in rust, also called lambda expressions are functions that can capture the enclosing environment
// e.g. `|val| val + x` is a closure that captures the x variable
// they are good for on the fly usage. Calling a closure is like calling a function
// Howver, both input and return types can be inferred and the input variable names must be provided

// Other characteristics
// use || instead of () around input variables.
// {} is optional for single expressions, mandatory otherwise.
// the ability to capture outer environment variables

fn main() {
    // incremement via functions and closures
    fn function(i: i32) -> i32 { i + 1 } 

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure annotated: {}", closure_annotated(i));
    println!("closure inferred: {}", closure_inferred(i));

    // a closure can take no arguments and return a value
    let one = || 1;
    println!("Closure returning One: {}", one());

    // CAPTURING
    // closures can capture and move variables without annotation and flexibly adapt to the use case
    // closures can capture variables by reference, mutable reference or by value (in that order of preference)
    use std::mem;
    let color = "green";
    let print = || println!("`color`: {}", color); // the closure borrows color and stores the borrow and the closure
    // in the print variable - it will remain borrowed till print goes out of scope. println! only requires `by reference`
    print();
    print();

    let mut count = 0; // a closure to increment count can take either `&mut count` or `count` but `&mut count` is less restrictive
    // so it takes that
    let mut inc = || { // a mut is required on inc because a `&mut` is stored inside. Calling the closure mutates it
        count += 1;
        println!("`count`: {}", count);
    }; // count is mutably borrowed here in the closure definition

    inc();
    inc();

    // let reborrow = &mut count; // this will give an error as count is already borrowed as a mutable inside the closure
    let movable = Box::new(3);  // this is a non copy type; has to be a value as mem::drop will require T so it must take by value
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // a second invocation will give an error

    // using move before || forces closure to take ownership of captured variables
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("{}", haystack.len()); // This will give an error as haystack has moved into the closure
    // removing move from the closure definition will make the closure borrow it immuatbly

    // AS INPUT PARAMETERS
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned(); // to_owned creates owned data from borrowed data
    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now, I can sleep");
        mem::drop(farewell);
    };
    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    // INPUT FUNCTIONS
    let closure = || println!("I am a closure");
    call_me(closure);
    call_me(function1);
}

// AS INPUT PARAMETERS - seems to be more advanced. Will probably have to revisit a few times
// Function input parameters always have to be annotated.
// If a closure is used as an input parameter, the closure's complete type must be annotated using only a few traits
// Fn: &T; FnMut: &mut T; FnOnce: T (this is in order of decreasing restriction)
// if we use FnOnce, it can be either of the 3 based on how the captured varaibles are used in the closure
// This is because if a move is possible, any type of borrow should be possible. If a mutable borrow is possible,
// then an immutable borrow should also be possible
fn apply<F>(f: F) where
    F: FnOnce() {
        f(); // the closure takes no input and returns nothing
    }

// a function that takes a closure and returns an i32
fn apply_to_3<F>(f: F) -> i32 where 
    F: Fn(i32) -> i32 {
        f(3)
    }

// TYPE ANONYMITY
// https://huonw.github.io/blog/2015/05/finding-closure-in-rust/

// INPUT FUNCITONS - functions can also take other functions as parameters.
// if we define a function that takes a closure as a parameter, any function that satisfies the trait bound can be passed as a parameter
fn call_me<F: Fn()>(f: F) { // function that takes a generic `F` argument bounded by Fn() and calls it.
    f();}

fn function1() {
    println!("I am a function");
}

// OUTPUT PARAMETERS - a function can return a closure but rust only supports returing concrete types. 
// Returning a closure is only possible by making it concrete. This can be done via boxing