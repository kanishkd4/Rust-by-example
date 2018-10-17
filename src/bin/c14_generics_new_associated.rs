
// NEW_TYPES
// The newtype idiom gives compile time guarantees that the right type of value is supplied to a program
// For example, an age verification function that checks age in years must be given a value of type `Years`

struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    // truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    // NEW_TYPES
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_days.to_years()));
    // println!("Old enough: {}", old_enough(&age_days)); // gives an error

    // ASSOCIATED ITEMS
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}", 
    &number_1, &number_2, container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

// ASSOCIATED ITEMS
// Associated items refers to a set of rules pertaining to item S of various types. It is an extention of trait generics
// and allows trait S to internally define new items. One such item is an associated type, providing simpler
// usage patterns when the trait is generic over its container type

// A trait that is generic over its container type has specificaion requirements - users of the trait must specify all 
// of its generic types. In the example, the `Contains` trait allows the use of generic types A and B. 
// The trait that is then implemented for the Container type, specifying i32 for A and B so it can be used with 
// fn difference()
// Because, `Contains` is generic, we are forced to explicitly state all of the generic types for fn difference().
// In practise, we want a way to express that A and B are determined by the input C.

struct Container(i32, i32);
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // explicitly requires A and B
    fn first(&self) -> i32; // doesn't explicitly require A and B
    fn last(&self) -> i32; // doesn't explicitly require A and B
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// C contains A and B. In light of that, having to express A and B again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
        container.last() - container.first()
}
