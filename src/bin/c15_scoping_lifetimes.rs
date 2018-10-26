// A lifetime is a construct that the compiler (the borrow checker) uses to ensure all borrows are valid.
// Specifically, a variables lifetime begins when it is created and ends when it is destroyed.
// Lifetimes and scopes are not the same.

// When we borrow a variable via `&`. The borrow has a lifetime that is determined by where it is declared. 
// As a result, the borrow is valid as long as it ends before the lender is destroyed. 
// However, the scope of the borrow is determined by where the reference is used. 

fn main() {
    let i = 3; // lifetime for i starts
    {
        let borrow1 = &i; // `borrow1` lifetime starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends
    {
        borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` ends
} // i ends

// EXPLICIT ANNOTATION
// The borrow checker uses explicit lifetime annotations to detemine how long references should be valid.
// In cases where lifetimes are not elided*, rust requires explicit annotations to determine what the lifetime of a reference should be
// * - elision implicitly annotates lifetimes

// syntax for explicitly annotating a lifetime uses an apostrophe: foo<'a> 
// Similar to closures, using lifetimes requires generics. Additionally, this lifetime syntax indicates that the lifetime of foo may
// not exceed that of 'a. 