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