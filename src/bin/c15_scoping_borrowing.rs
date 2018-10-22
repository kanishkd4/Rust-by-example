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
    
}