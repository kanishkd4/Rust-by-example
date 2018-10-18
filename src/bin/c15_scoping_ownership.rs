// Scopes play an important part in ownership, borrowing and lifetimes. They indicate to the compiler
// when borrows are valid, when resources can be freed and when variables are created or destroyed

// RAII - Resource acquisition is initialization
// Variables in rust own resources, e.g. memory in the heap. 
// When an object goes out of scope, its destructor is called and its own resources are freed. 

fn create_box() {
    let _box1 = Box::new(3i32);

    // _box1 is destroyed here and memory gets freed
}

// The notion of a destructor is provided using the Drop trait. It's called when
// the resource goes out of scope. The trait is not required to be implemented for every type,
// only implement it for your type if you require its own destructor logic
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped")
    }
}

fn main() {
    let _box2 = Box::new(5i32); // an integer allcated in the heap

    // A nested scope
    {
        let _box3 = Box::new(4i32);
        // _box3 is destroyed here
    }
    for _ in 0u32..1_000 {
        create_box(); // will always be destroyed inside this scope
    }

    // _box2 is destroyed here

    let x = ToDrop;
    println!("Made a ToDrop");

    // OWNERSHIP AND MOVES
    let x = 5u32; // stack allocated integer
    let y = x; // a copy as x is a simple type
    println!("x is {} and y is {}", x, y);
    let a = Box::new(5i32); // heap allocated integer
    println!("a contains: {}", a);

    let b = a;// move a into b; the default is copy for simple types and move for complex types (or heap allocated data)
    // a is a pointer to a heap allocated integer and moving b into a moves the pointer address of a into b
    // the data remains the same but now, b owns it
    destroy_box(b); // the function takes ownership of the heap allocated memory from b

    // MUTABILITY
    // Mutability of data can be changed when ownership is transfered. 
    let immutable_box = Box::new(5u32);
    println!("immutable box contains: {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable box contains: {}", mutable_box);
    *mutable_box = 4;
    println!("mutable box now contains: {}", mutable_box);
}

// OWNERSHIP AND MOVES
// Resources can have only one owner.
// When doing assignments (let y = x) or passing function arguments by value,
// the ownership of the resources is transferred - this is a move.
// After moving resources, the previous owner can no longer be used

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

