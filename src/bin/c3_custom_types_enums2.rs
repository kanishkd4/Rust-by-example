#![allow(dead_code)]
// The use declaration can be used so that manual scoping isn't necessary

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// A common use for enums is to create a linked list - https://doc.rust-lang.org/1.21.0/std/collections/struct.LinkedList.html
// A linked list allows pushing and popping elements at either end at constant time
// It is however, almost always better to use Vec or VecDeque instead of a linked list as
// array based containers are faster, more memory efficient and make better use of the CPU cache
use List::*;
enum List {
    // Cons is a tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil is a node that signifies the end of a linked list
    Nil,
}

impl List {
    // create an empty list
    fn new() -> List {
        // `Nil` has a type List
        Nil
    }
    
    // consume a list, and return the same list with a new element in its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has a type List
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 { // return the length of the list
        // self has to be matched due to the behaviour of this method.
        // `self` has the type &List and `*self` has the type List, matching on a concrete type T is preferred to a reference &T
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0 // an empty list has 0 length
        }
    }

    // return representation of a list as a heap allocated String
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! is similar to print! but returns a heap allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}


fn main() {
    // USE
    // explicitly use each name so they are available without manual scoping

    use Status::{Poor, Rich};
    // automatically `use` each name inside work
    use Work::*; 

    let status = Poor; // equivalent to Status::Poor
    let work = Civilian; // equivalent to Work::Civilian

    match status {
        // we don't need scoping due to the explicit `use` above
        Rich => println!("The Rich have lots of money"),
        Poor => println!("The Poor have no money..."),
    }
    // we can use work the same way

    // TESTCASE: LINKED-LIST
    // create an empty linked list
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}