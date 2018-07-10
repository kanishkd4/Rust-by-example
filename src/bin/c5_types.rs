
// CASTING
// Rust provides no implicit conversiion bu explicit conversion can be used using as

// INFERENCING
// The type inferencing system looks at the value as well as how the variable is used in the code to infer the type.


#![allow(overflowing_literals)]

// ALIASING
// the type statement can be used to give a new name to an existing type
// the main use of aliasing is avoiding boilerplate, e.g. the IoResult<T> type is an alias for the Result<T, IoError> type
type u64_t = u64;


fn main() {
    let decimal = 65.4321_f32;

    // explicit conversion as allowed by rust
    let integer: u8 = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type T, std::T::MAX + 1 is added or subtracted till the value fits into the new type
    // mostly, this means that don't cast like an idiot!

    // have to use `as` with a variable binding - numeric literals can be annotated by using suffixes
    // if a literal is unsuffixed, the type depends on how it's used.
    println!("1000 as u16 is {}", 1000u16);
    println!("1000 as u8 is {}", 1000u8); 
    println!("-1 as u8 is {}", (-1i8) as u8);

    println!("128 as i16 is {}", 128 as i16);
    println!("128 as i8 is {}", 128 as i8);

    // INFERENCING
    let elem = 5u8; // the compiler knows the type as we have annotated it
    
    let mut vec = Vec::new(); // the compiler does not know the type of the vector yet
    vec.push(elem); // it now knows the vector is u8
    println!("{:?}", vec);  
}
