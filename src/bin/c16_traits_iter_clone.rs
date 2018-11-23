// ITERATORS
// The Iterator trait is used to implement iterators over collections such as arrays.
// The trait requires only a method to be defined for the next element, which may be manually defined in an impl block 
// or automatically defined (as in arrays or ranges). For convenience, the for construct turns some collections
// into iterators using the into_iterator() method.

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci { // returns a Fibonacci sequence generator
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3  using `for`"); // for works through an iterator till it returns `None`
    for i in 0..3 {
        println!("> {}", i);
    }
    // The take(n) method reduces an Iterator to its first n terms
    println!("The first 4 terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }
    // The skip method shortens an `Iterator` by dropping its first n terms
    println!("The next 4 terms of a Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() { // the iter method produces an Iterator over an array/slice
        println!("> {}", i); 
    }

    // CLONE
    let nil = Nil; // Instantiate Nil
    let copied_nil = nil; // copy nil; there are no resources to move
    // Both Nil's can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2)); // instantiate Pair
    println!("original: {:?}", pair);

    let moved_pair = pair; // Copy pair into moved_pair; moves resources. pair has lost its resources
    println!("copy: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair); // drop the original pair
    println!("clone: {:?}", cloned_pair);
}

// CLONE
// When dealing with resources, the default behaviour is to transfer them during assignments or function calls.
// However, sometimes we need to make a copy of the resource as well. 
// The clone trait helps us do that.

#[derive(Debug, Clone, Copy)] // a unit struct without resources
struct Nil;

#[derive(Debug ,Clone)]
struct Pair(Box<i32>, Box<i32>); // a tuple struct with resources that implements the Clone trait

