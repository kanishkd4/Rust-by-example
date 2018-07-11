#[allow(unreachable_code)]

// Branching with if-else is similar to other languages, unlike other languages, the boolean condition doesn't need to be surrouded by ()
// if-else conditionals are expressions, all branches must return the same type


fn main() {
    // IF-ELSE
    let n = 5;
    if n < 0 {
        print!("{} is negetive", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!("{} is a small number, increase 10-fold", n);
            10 * n // this expression returns an i32
        } else {
            println!("{} is a big number, half it", n);
            n / 2
        }; // the `;` here is needed as all let bindings need it

    println!("{} -> {}", n, big_n);

    // LOOP
    // `loop` can be used to run an infinite loop. 
    // Use `break` to exit the loop; `continue` to skip the rest of the iteration and start a new one

    let mut count = 0u32;
    println!("Counting till infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");

            continue; // skip the rest of this iteration
        }
        println!("{}", count);

        if count == 5 {
            println!("Enough counting");
            break; // exit the loop
        }
    }

    // NESTED LOOPS
    // It's possible to break or continue outer loops when dealing with nested loops. 
    // To do this, the loops must be annotated with some labels.
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // break; // this would break only the inner loop
            break 'outer; // breaks the outer loop
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // RETURNING FROM LOOPS
    // Put the value after the break and it will be returned by the loop expression
    // we'll use the count integer created in the first loop above.
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    assert_eq!(result, 20);

    // WHILE LOOP
    // can be used to run a loop till a condition is met
    let mut n = 1;
    let mut m = 1;
    while n < 101 && m < 90 {
        if n % 50 == 0 {
            println!("{}", n);
        }
        n += 1;
        m += 1;
    }

    // FOR
    // the same loop in while using for (without m)
    for n in 1..101 { // we can also use 1..=100 to include both sides of a range
        if n % 50 == 0 {
            println!("{}", n);
        }
    }

    // we can use `for in` to interact with an iterator.
    // if not specified, the for loop will apply the `into_iter` function on the collection
    // provided to convert the collection into an iterator. There is `iter`, `into_iter` and `iter_mut`

    // iter: borrows each element of the collection through each iteration leaving the collection untouched and available to use after the loop
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello, {}", name),
        }
    }

    // into_iter: consumes the collection so on each iteration, the exact data is provided.
    // once the collection has been consumed, it is no longer available as it has been moved within the loop
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello, {}", name),
        }
    }
    // names no longer exists in scope

    // iter_mut: this mutably borrows each element of the collection, allowing for the collection to be modified in place
    // the collection has to be mutable to be used.
    let mut names1 = vec!["Bob", "Frank", "Ferris"];
    for name in names1.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello, {}", name),
        }
    }
}