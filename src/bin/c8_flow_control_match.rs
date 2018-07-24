#[allow(dead_code)]

fn main() {
    // MATCH
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"), // match a single value
        2 | 3 | 5 | 7 | 11 => println!("A prime number"), // match several values
        13...19 => println!("A teen"), // match an inclusive range. I've only seen the pattern ... in a match. ..= also works
        _ => println!("Ain't special"), // everything else
    }
    
    let boolean = true;
    // match is also an expression
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // DESTRUCTING
    // a match expression can destruct a tuple 
    let pair = (0, -2); // - simple destructuring of the tuple would be let (x, y) = pair;
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is 0, and y is {:?}", y),
        (x, 0) => println!("x is {:?}, second is 0", x),
        _ => println!("It doesn't matter what they are"),
    }

    // an enum is destructed similarly
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        _ => println!("Ignoring others"), // ideally, we would deconstruct all parts of an enum and not use an _
    }

    // There is a difference between destructuring pointers(using &, ref and ref mut) and dereferencing pointers(using *)
    let reference = &4; // If a reference is pattern matched against &val, it results in a comparison
    match reference { // if the matching &'s are dropped, then the i32 should be assigned to val (didn't understand this)
        &val => println!("Got a value via destructuring: {:?}", val), 
    }
    
    // to avoid the &, dereference before the comparison
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // If we don't start with a reference, we can use the ref keyword that modifies the assignment 
    // so that a reference is created for the element. The reference is assigned
    // ref in a binding is the same as & in an expression - both have types &T
    let ref _is_a_reference = 3;

    let value = 5; // if we define 2 values without references, they can be retreived via ref and ref mut
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // using ref 
    match mut_value {
        ref mut m => { // got a reference, have to dereference it before we can add anything to it
            *m += 10;
            println!("We added 10, `mut_value`: {:?}", m);
        },
    }

    // Destructing a struct
    struct Foo { x: (u32, u32), y: u32 }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y: c } = foo;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // we can also destructure structs and rename the varaibles. Order is not important
    let Foo { y: i, x: j } = foo;
    println!("i: {:?}, j = {:?}", i, j);

    // we can also ignore some variables
    let Foo { y, .. } = foo; // the placeholder `..` is needed
    println!("y: {}", y);


    // GUARDS
    // a match guard can be added to filter the arm
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"), // the if condition part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No corelation"),
    }

    // BINDINGS
    // Indirectly accessing a variable makes it impossible to branch and use the variable without rebinding
    // match provides the @ sigil for binding values to names
    println!("Tell me the type of person");
    match age() {
        0 => println!("Not born yet"),
        // could match 1...12 but what would the actual age be? - instead bind n to the 
        n @ 1...12 => println!("A child of {:?}", n), // again `...` instead of `..=` in match. Just stick to ..= in your own code
        n @ 13...19 => println!("A child of age {:?}", n),
        // nothing bound, simply return the result
        n => println!("An old person of age {:?}", n),  // `n =>` is equivalent to `n @ _ =>`
    }

    // IF LET  and WHILE LET
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("This is a long string and `{:?}`", i);
        },
        _ => {}, // needed as match is exhaustive
    }

    // using an if let - a cleaner implementation of match when we only care about one case
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let construct reads, if let destructures number into Some(i), evaluate block ({})
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // if we need to specify failure, specify an else
    if let Some(i) = letter {
        println!("Matched: {:?}", i);
    } else {
        println!("Didn't match a number, let's go with a letter");
    }

    // provide an altered failing condition.
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number, let's go with a letter");
    } else {
        println!("I don't like letters, let's go with emoticon");
    }

    // In the same way, if let can be used to match any enum value.
    let a = FooE::Bar;
    let b = FooE::Baz;
    let c = FooE::Qux(100);
    if let FooE::Bar = a {
        println!("a is Foobar")
    }

    if let FooE::Bar = b {
        println!("b is Foobar");
    }

    if let FooE::Qux(value) = c {
        println!("c is {}", value);
    }
}

fn age() -> i32 {
    15
}

enum FooE {
    Bar,
    Baz,
    Qux(u32),
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32),
}