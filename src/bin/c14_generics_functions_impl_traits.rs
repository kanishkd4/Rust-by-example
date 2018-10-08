// Generics is the topic of generalizing types and functionalities to broader cases.
// This is useful for reducing code duplication in many cases and but can call for rather involving syntax.
// Being generic requires taking great care to specify over which types a generic type is actually 
// considered valid. The simplest and most common case for generics is for type parameters
// notation: Specify a generic by using angle brackets and upper camel case

// example: a generic function named foo that takes an argument T of any type
fn foo<T>(arg: T) {} // becuase T has been specified as a generic type parameter using <T>, 
// it is considered generic when used here as arg: T, even if T has been previously defined as a struct

struct A; // a concrete type

struct Single(A); // Single is the first use of type A

// <T> preceeds the first use of T so SingleGen is a generic type. 
// Because the type parameter T is a generic, it can be any type, including the concrete type A(struct) defined above
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A); // a concrete type that explicitly takes A

    // Here, SingleGen has a type parameter explicitly specified
    let _char: SingleGen<char> = SingleGen('a');
    // let ch:() = _char; // the type is SingleGen

    // SingleGen can also have a type parameter implicitly specified
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    // IMPLEMENTATIONS
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());

    // TRAITS
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    // empty and null are both moved in the above line
}


// FUNCTIONS
// The same set of rules can be applied to function: a type T becomes a generic if preceeded by a <T>
struct S(A); // concrete type S that takes concrete type A defined above // commented for struct S below
struct SGen<T>(T); // Generic type SGen

fn reg_fn(_s: S) {} // a regular function that takes an argument of type S - not a generic function

fn gen_spec_t(_s: SGen<A>) {} // this is also not a generic as SGen has been given the type A
fn gen_spec_i32(_s: SGen<i32>) {} // this is also not a generic
fn generic<T>(_s: SGen<T>) {} // a generic function

// IMPLEMENTATIONS
// Implementations require similar care as functions to remain generic
struct S1; // concrete type S
struct GenericVal<T>(T, ); // Generic type GenericVal

// impl of GenericVal where we explicitly specify type parameters
impl GenericVal<f32> {} // specified f32
impl GenericVal<S> {} // specified S as defined above

// '<T>' must precede the type to remain generic
impl <T> GenericVal<T> {}

struct Val {
    val: f64
}

struct GenVal<T> {
    gen_val: T
}

// impl of val
impl Val {
    fn value(&self) -> &f64 { &self.val }
}

// impl of GenVal for generic type 'T'
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

// TRAITS: not done yet but covering for generics
// define a trait that reimplements the Drop trait as a generic method to drop and an input

struct Empty;
struct Null;

// A trait generic over 'T'.
trait DoubleDrop<T> {
    // Define a method on the caller that takes an additional single parameter 'T' and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement DoubleDrop<T> for any generic parameter 'T' and caller 'U'
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments, deallocating both
    fn double_drop(self, _: T) {}
} 




