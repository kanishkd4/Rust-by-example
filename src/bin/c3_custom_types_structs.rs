#[allow(unused)]

// 3 types of structs: tuple structs(named tuples), class C structs and unit structs

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f32);

// a struct with 2 fields
struct Point {
    x: f32,
    y: f32,
}

// structs can be used as fields for other structs
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    // Instantiate a point
    let point = Point { x: 0.3, y: 0.4 };

    // access the fields of a point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using y from the old point
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point; // my_x and my_y are new variables created here that contain the components of the struct
    println!("my_x is {}; my_y is {}", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains: {:?} and {:?}", integer, decimal);
}
