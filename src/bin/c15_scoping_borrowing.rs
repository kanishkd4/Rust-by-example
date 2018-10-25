// BORROWING
// Rust uses a borrowing mechanism to allow the user to access data without taking ownership of it.
// Instead of passing an object by value(T), it can be passed as a reference(&T). 
// While references to an object exist, the object cannot be destroyed.

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying a box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is borrowed: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32); // cannot destroy boxed_i32 as the inner value is borrowed
    }

    eat_box_i32(boxed_i32); // destroyed here

    // MUTABILITY
    let immutabook = Book {
        author: "Douglas Hofstader",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;
    borrow_book(&immutabook);   // borrow immutable book as mutable
    borrow_book(&mutabook);     // borrow mutable book as immutable
    new_edition(&mut mutabook); // borrow mutable book as mutable

    // FREEZING
    let mut _mutable_integer = 7i32;
    {
        let _large_integer = &_mutable_integer; // immutable borrow
        // _mutable_integer = 15; // won't work as it is borrowed as immutable
    }
    _mutable_integer = 3; // will work here

    // ALIASING
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let _borrowed_point = &point;
        let _another_borrow = &point; // any number of immutable borrows can be done together
        // immutable borrows are dropped here
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;
        // point cannot be borrowed as immutable here as it is borrowed as mutable
    }
    // immutable references to point are allowed again
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);

    // THE REF PATTERN
    let c = 'Q';
    // A `ref` borrow on the left side of an assignment is equivalent to an `&` borrow on the right side
    let ref ref_c1 = c;
    let ref_c2 = &c; // the two lines above do the same thing

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2); // doesn't necesarily need to be dereferenced.

    let point = Point1 { x: 0, y: 0 };
    // `ref` is also valid when destructuring a struct
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Point1 { x: ref ref_to_x, y: _ } = point;
        *ref_to_x // returns a copy of the x field of point
    };

    let mut mutable_point = point; // a mutable copy of point
    {
        let Point1 { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1; // mutate the `y` field of mutalbe_point via a mutable reference.
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32); // a mutable tuple that includes a pointer
    {
        // Destructure mutable_tuple to change the value of `last`
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}

// MUTABILITY - Mutable data can be mutably borrowed using &mut T. This is a mutable reference and gives read/write
// access to the borrower. In constrast, &T borrows data via an immutable reference, and the borrower
// can read the data but not modify it.

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory.
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

// FREEZING
// When data is immutably borrowed, it also freezes. Frozen data cannot be modified via the original object till 
// all references go out of scope. 

// ALIASING
// Data can be immutably borrowed any number of times, but while immutably borrowed, the original data cannot be mutably borrowed.
// On the other hand, only one mutable borrow is allowed at a time. 
struct Point { x: i32, y: i32, z: i32 }

// THE REF PATTERN
// When doing pattern matching or destructuring via the let binding, the ref keyword can be used to take references to the
// fields of a struct/tuple
#[derive(Clone, Copy)]
struct Point1 { x: i32, y: i32 }