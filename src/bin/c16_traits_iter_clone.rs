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
}