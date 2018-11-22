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

    // OPERATOR OVERLOADING
    println!("Foo + Bar =  {:?}", Foo + Bar);
    println!("Bar + Foo =  {:?}", Bar + Foo);

    // DROP
    let _a = Dropable { name: "a" };
    {
        let _b = Dropable { name: "b" };
        {
            let _c = Dropable { name: "c" };
            let _d = Dropable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");

    // drop(_a); // The variable can also be manually dropped; otherwise will get dropped at the end of the main function
    println!("End of main function");
}

// OPERATOR OVERLOADING
// Many operators can be overloaded via traits. Some operators can be used to accomplish different tasks based on their input arguments.
// This is possible because operators are syntactic sugar for method calls. The + operator in a + b calls the add method (a.add(b))
// The add method is a part of the Add trait. Hence, the + operator can be used by any implementor of the Add trait.
// A list of traits such as Add, that overload operators can be found in core::ops
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo { // std::ops::Add trait is used to specify the functionality of `+`
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

// DROP
// The Drop trait has only one method: drop, which is called automatically when an object goes out of scope.
// The main use of the Drop trait is to free the resource that the implementor instance owns.
// Some examples of the types that implement the Drop trait are Box, Vec, String, File, and Process. It can also be manually 
// implemented for custom types. The example adds a print to console to the drop function to announce when it's called.

struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    fn drop(&mut self) {
        println!(">Dropping {}", self.name);
    }
}