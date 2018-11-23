// HOF's and lazy iterators give rust it's functional flavour

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers between 1 and 100");
    let upper = 1000;

    // imperative appraoch
    let mut acc = 0; // accumulator variable
    for n in 0.. { // 0 to infinity
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("Imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                            // all natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // below upper limit
            .filter(|&n_squared| is_odd(n_squared))     // odd numbers only
            .fold(0, |acc, n_squared| acc + n_squared); // add them
    println!("Functional style: {}", sum_of_squared_odd_numbers);


    // DIVERGING functions
    let _a: () = some_fn();
    println!("some_fn() returns and you can see this line");

    // let x: ! = panic!("This call never returns");
    // println!("This line will never print");
}

// DIVERGING FUNCTIONS
// Diverging functions never return. They are masked using !, which is an empty type.
// The set of all possible values `!` can have is empty. `!` is different from `()`, as `()` has exactly one possible value
// Leave this for later if ever needed as ! type is experimental at rust 1.26
fn foo() -> ! {
    panic!("This call never returns");
}

fn some_fn() { // this function returns as usual, although there is no information in the return value
    ()
}

