
// An enum keyword allows the creation of a type that may be one of a few different variants.
// Any variant valid as a struct is also valid as an enum
#![allow(dead_code)]

enum web_event {
    // an enum can be unit like
    page_load,
    page_unload,
    // like tuple structs
    key_press(char),
    paste(String),
    // or like structures
    click { x: i64, y: i64 },
}

// A function that takes a web_event enum and returns nothing
fn inspect(event: web_event) {
    match event {
        web_event::page_load => println!("page loaded"),
        web_event::page_unload => println!("page unloaded"),
        
        // Destructure c from inside the enum
        web_event::key_press(c) => println!("pressed '{}'.", c),
        web_event::paste(s) => println!("pasted \"{}\".", s),

        // Destructure click into `x` and `y`
        web_event::click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

fn main() {
    let pressed = web_event::key_press('x');
    // `to_owned()` creates an owned string from a string slice.
    let pasted = web_event::paste("my text".to_owned());
    let click = web_event::click { x: 20, y: 80 };
    let load = web_event::page_load;
    let unload = web_event::page_unload;

    // Based on the type of argument in inspect, the function prints a different line

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}