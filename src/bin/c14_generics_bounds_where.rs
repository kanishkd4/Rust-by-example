// When working with generics, the type parameters often must use traits as bounds to stipulate what functionality
// a type implements, e.g. one example here uses the traitl Display to print so it requires T to be 
// bound by Display; that is, T must implement Display

use std::fmt::Display;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// Bounding restricts the generic to types that conform to the bounds. That is :
struct s<T: Display>(T); // S can only be a type that implements the Display trait - so no vectors

// Another effect of bounding is that generic instances are allowed to access the methods of traits 
// specified in the bounds
use std::fmt::Debug; // a trait that implements print marker {:?}
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// The generic T must implement Debug. Regardless of the type, this will work properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// T must implement HasArea. Any function which meets the bound can access HasArea's function area
fn area<T: HasArea>(t: &T) -> f64 { t.area() }


fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));


    // TESTCASE: EMPTY BOUNDS
    let cardinal = Cardinal;
    let bluejay = BlueJay;
    let _turkey = Turkey;

    // red() won't work on bluejay and blue won't work on cardinal
    println!("A cardinal is: {}", red(&cardinal));
    println!("A blue jay is: {}", blue(&bluejay));

    // MULTIPLE BOUNDS
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);

    // WHERE CLAUSES
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}


// TESTCASE: EMPTY BOUNDS
// A consequence of how bounds work is that even if a trait doesn't include any functionality, it can be used
// as a bound. Eq and Ord are example of such traits from the std library

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these traits.
// The fact that these traits are empty is irrelevant

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

// MULTIPLE BOUNDS
// can be applied with a `+`. Different types are separated by a `,`.
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);    
}

// WHERE CLAUSES
// A bound can also be expressed using a where clause immediately before the opening {, rather than at the type's first mention
// where clauses can apply bounds to arbitrary types, rather than to type parameters.

// A where clause is useful when specifying generic types and bounds separately is clearer
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a where clause
// impl <A, D> MyTrait<A, D> for YourType where 
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}


// Sometimes, using a where clause is more expressive than using normal syntax. The impl in this example cannot be 
// directly expressed 
trait PrintInOption {
    fn print_in_option(self);
}

// Because we would have to express this as `T: Debug` or use another method of indirect approach,
// this requires a where clause
impl <T> PrintInOption for T where
    Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

