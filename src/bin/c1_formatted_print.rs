fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // using positional arguments

    // Special formatting can be specified after a ":"
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // We can right align text with a specific width
    println!("{number:>width$}", number=1, width=6);

    // we can pad numbers with extra zeros
    println!("{number:>0width$}", number=1, width=6);

    // all std library types are printable with :? too
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian", 
        actor="actor's");

    println!("Now {:?} will print!", Structure(3));

    // Rust also provides pretty printing with :#?
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
}

#[derive(Debug)]
struct Structure(i32);
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

