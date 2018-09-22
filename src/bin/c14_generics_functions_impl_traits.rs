// Generics is the topic of generalizing types and functionalities to broader cases.
// This is useful for reducing code duplication in many cases and but can all for rather involving syntax.
// Being generic requires taking great care to specify over which types a generic type is actually 
// considered valid. The simplest and most common case for generics is for type parameters

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
}


