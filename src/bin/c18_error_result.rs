// RESULT
// Result is a richer version of Option that describes possible error instead of possible absence
// Result<T, E> can have 2 outcomes: Ok<T> (an element T was found) and Err<E> (An error was found with element E)
// The expected outcome is Ok and unexpected outcome is Err. Like Option, Result has associated methods, like unwrap
// For case handling, there are many combinators between Result and Option that overlap
// Some methods return a Result type, like parse()

fn multiply(fist_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = fist_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("10 * 2: {}", twenty);
    // let tt = multiply("tt", "2"); // parse leaves an error for unwrap to panic on. 
    // println!("parse error: {}", tt); // We can also explicitly handle the error

    let twenty = multiply1("t", "2");
    print(twenty); // prints a more helpful error message

    let thirty = multiply2("10", "3");
    print(thirty);
    let thirty = multiply2("fist_number_str: &str", "2");
    print(thirty); // same error as twenty above but with and_then and map

    // ALIASES FOR RESULT
    print1(multiply3("12", "3"));
    print1(multiply3("t", "2"));

    // EARLY RETURNS
    print(multiply4("10", "4"));
    print(multiply4("t", "2"));

    // INTRODUCING ?
    print(multiply5("10", "5"));
    print(multiply5("t", "2"));
}

// MAP FOR RESULT
// The Err type in parse is specified as ParseIntError
use std::num::ParseIntError;

fn multiply1(fist_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match fist_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// we can use a map instead of the long match in multiply1
// it reads: modify n if the value is valid, otherwise pass on the error
fn multiply2(fist_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    fist_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// ALIASES FOR RESULT
// If we want to reuse a specific Result type many times, rust allows us to create aliases
// At a module level, creating aliases can be helpful as errors in a specific module have the same 
// Err type, so a single alias can succinctly define all associated Results. The std library has one in io::Result

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply3(fist_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    fist_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print1(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// EARLY RETURNS
// In the previous example, we explicitly handled the errors using combinators. Another way to deal with this case
// analysis is to use a combincation of match and early returns, i.e. we can stop executing the function and
// return the error if one occurs. We can rewrite multiply3 using early returns

fn multiply4(fist_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match fist_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };
    Ok(first_number * second_number)
}

// INTRODUCING ?
// Sometimes, we want the simplicity of unwrap without the possiblity of a panic. unwrap has forced us to 
// go deeper and deeper when we really just want to get the variable out. That's done by `?`
// `?` is almost exactly equivalent to unwrap which returns instead of panics on Err
fn multiply5(fist_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = fist_number_str.parse::<i32>()?;   // Before `?`, this was done using try! (not recommended anymore)
    let second_number = second_number_str.parse::<i32>()?; // as try!(x.parse::<i32>())

    Ok(first_number * second_number)
}