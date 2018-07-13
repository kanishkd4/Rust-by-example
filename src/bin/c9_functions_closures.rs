// Closures in rust, also called lambda expressions are functions that can capture the enclosing environment
// they are good for on the fly usage. Calling a closure is like calling a function
// Howver, both input and return types can be inferred and the input variable names must be provided

// use || instead of () around input variables.
// {} is optional for single expressions, mandatory otherwise.

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
}

