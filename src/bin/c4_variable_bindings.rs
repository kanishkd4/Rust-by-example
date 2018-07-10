
// variable bindings - seems simple enough. The 
// https://stackoverflow.com/questions/41659471/difference-between-variable-bindings-and-variables
// note: prefix unused variables with an underscore to supress warnings of unused variables

// variables are immutable by default - use mut if you want mutable variables


fn main() {
    let _an_integer = 1u32; // prefixing with `_` to supress unused variable warning

    // SCOPE AND SHADOWING
    let long_live_binding = 1;
    
    {
        let _short_lived_binding = 2; // short_lived_binding is in scope
        let long_live_binding = 3; // this is a different long_live_binding
        println!("long_live_binding inner scope: {}", long_live_binding);
    } // short_lived_binding is out of the scope

    println!("long_live_binding outer scope: {}", long_live_binding); // this is trippy!

    // already know about shadowing: can use let to shadow an existing immutable variable

    // DECLARE FIRST
    // it is possible to declare variable bindings first and initialize them later but it is seldom used.
    let a_binding;
    {
        let x = 2;
        a_binding = x*x;
        println!("Inner scope a_binding: {}", a_binding);
    }
    println!("Outer scope a_binding: {}", a_binding);
    println!("Inner scope a_binding and outer scope a_binding are the same \
    as the variable is initialized in the outer scope")
}