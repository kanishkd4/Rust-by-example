// Derive
// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute.
// These traits can still be manually implemented if more complex behaviour is required. 
// List of derivable traits
// 1. comparison traits: Eq, ParitalEq, Ord, PartialOrd
// 2. Clone, to create T from &T via a copy
// 3. Copy, to give a type "copy semantics" instead of "move semantics"
// 4. Hash, to compute a hash from &T
// 5. Default, to create an empty instance of a data type.
// 6. Debug, to format a value using the {:?} formatter

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64); // Centimeters, a tuple struct that can be compared

#[derive(Debug)]
struct Inches(i32); // A tuple struct that can be printed

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self; // this is destructuring a struct to create a variable `inches` that will be an i32

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32); // a tuple struct with no attributes

fn main() {
    let _one_second = Seconds(1); // can't be printed or compared
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = 
        if foot.to_centimeters() < meter { // comparing 2 variables that are type Centimeters as they have PartialEq implemented
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter", cmp);
}

// OPERATOR OVERLOADING