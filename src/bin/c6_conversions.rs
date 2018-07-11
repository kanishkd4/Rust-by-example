
// Rust addresses conversion between types by using traits
// From and Into are for generic conversions and there are more specific ones.

fn main() {
    let my_str = "hello";
    // the from trait allows for a type to create itself from another type.
    // my_str is a `str` and we convert it to a String
    let _my_string = String::from(my_str); // prefixed with `_` as it isn't used in the program

    let num = Number::from(30);
    println!("Number on using from: {:?}", num);

    // Into is the reciprocal of From. Using into will require the specification of the type to convert to
    // The compiler is unable to identify it most of the time

    let int = 5;
    let num: Number = int.into();
    println!("Number on using into: {:?}", num);

    // ToString
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // It's more common convert stings to a numeric type.
    // we can use the parse function and provide the type for the function to parse the string value into
    // this can be done either without type inference or with the turbofish syntax.
    // A string will be converted to a type as long as the FromStr trait is implemented for the type.
    // to have this functionality for a user defined type, simply define this trait for the type

    let parsed: i32 = "5".parse().unwrap(); 
    let turbo_parsed = "20".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("parsed + turbo_parsed: {}", sum);
}

// we can define a conversion for our own type

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// To and From Strings
// To convert any type to a string, it is as simple as implementing the ToString trait for the type

use std::string::ToString;

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius: {:?}", self.radius)
    }
}

