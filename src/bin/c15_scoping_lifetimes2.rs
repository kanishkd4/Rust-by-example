// METHODS
// Methods are annotated similar to functions

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(19);
    owner.add_one();
    owner.print();

    // STRUCTS
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);
    println!("x is borrwed in {:?}", single);
    println!("x and y are borrwed in {:?}", double);
    println!("x is borrwed in {:?}", reference);
    println!("y is not borrwed in {:?}", number);

    // BOUNDS
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);

    // COERCION
    let first = 2;      // longer lifetime
    {
        let second = 3; // shorter lifetime
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

// STRUCTS
// Annotations of lifetimes in structs are also similar to functions
// A type `Borrowed` that houses a reference to an `i32`. The reference to `i32`  must outlive `Borrowed`
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// BOUNDS
// Just like generic types can be bounded, lifetimes(also generic) can use bounds.
// The : character has a different meaning here but + is the same
// 1. `T: 'a`: All references in T must outlive lifetime 'a
// 2. `T: Trait + 'a`: Type T
use std::fmt::Debug; // trait to bound with
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T); // Ref contains a reference to a generic type `T` that has an unknown lifetime 'a. `T` is bounded
// such that any references in T must outlive 'a. Additionally, the lifetime of Ref may not exceed 'a

fn print<T>(t: T) where  // a generic function which prints using the `Debug` trait
    T: Debug {
        println!("`print`: t is {:?}", t);
    }

fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a { // Here a reference to `T` is taken where `T` implements `Debug` and all references in T outlive 'a
        println!("`print_ref`: t is {:?}", t); // In addition, 'a must outlive the function
    }

// COERCION
// A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in
// This comes in the form of inferred coercion by the rust compiler, and also in the form of declaring a lifetime difference

// Here, rust inferrs a lifetime as short as possible. The 2 references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// <'a: 'b, 'b> reads as lifetime 'a is at least as long as 'b
// Here, we take an `&'a i32` and return an `&'b i32` as a result of coercion
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

