// ASSOCIATED TYPES
// The use of associated types improves the overall readability of code by moving inner types
// locally into a trait as output types. Syntax as follows

// A and B are defined in the trait via the type keyword.
trait Contains {
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// the functions that use the triat Contains are no longer required to express A or B at all

struct Container(i32, i32);

impl Contains for Container {
    // specify what type A and B are. If the input type is Container(i32, i32), then both output types are i32
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 { // without using associated types in the previous section
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}", 
    &number_1, &number_2, container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));

    // PHANTOM TYPE PARAMETERS
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // PHANTOM TYPE TEST CASE

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    //`+` calls the `add()` method we implemented for Length<Unit>
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot: {:?} in", two_feet.0);
    println!("one meter + one meter: {:?} mm", two_meters.0);
}

// PHANTOM TYPE PARAMETERS
// A phantom type parameter is one that doesn't show up at runtime but is checked statically only at compile time

use std::marker::PhantomData;

// A phantom tuple struct which is a generic over A with hidden parameter B
#[derive(PartialEq)] // allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);


// A phantom type struct which is a generic over A with hidden parameter B
#[derive(PartialEq)] // allow equality test for this type
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// storage is allocated for generic type A but not for type B; therefore, B cannot be used for computations

// PHANTOM TYPE TEST CASE

// pub trait Add<RHS = Self> {
//     type Output;
    
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// impl<U> Add for T<U> {
//     type Output = T<U>;
// }

use std::ops::Add;

// Create void enumerations to define unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

// Lenght is a type with phantom parameter `Unit` and is not generic over the length type(that is `f64`)
// f64 already implements the Cloen and Copy traits

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for f64
        Length(self.0 + rhs.0, PhantomData)
    }
}