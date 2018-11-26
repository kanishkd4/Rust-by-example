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

