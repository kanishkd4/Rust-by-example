// the final expression of a function is used as the return value. 
// The return statement can be used to return a value earlier from within the function

fn main() {
    
    print!("Is 4 divisible by 2: {:?}", is_divisible_by(4, 2));
    fizzbuzz_to(5);
    
    let rectangle = Rectangle {
        p1: Point::origin(),     // static methods are called using ::
        p2: Point::new(3.0, 4.0),
    };

    // instance methods are called using the dot operator
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0); // mutable methods can only be applied on mutable objects
    
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 { return false; }
    lhs % rhs == 0 // last expression, so the return keyword isn't needed here
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 5) {
        println!("fizzbuzz");
    } else {
        println!("{}", n);
    }
}

// when a function returns (), the type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..n+1 {
        fizzbuzz(n);
    }
}

// MEHTODS
// methods are functions attached to objects.
struct Point {
    x: f64,
    y: f64,
}
// Implementation block: all `Point` methods go in here.
impl Point { 
    fn origin() -> Point { // this is a static method. They don't need to be called by an instance. 
        Point { x: 0.0, y: 0.0 } // these methods are generally used as constructors
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 { // this is an instance method; where &self is sugar for `self: &Self`
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1-y2)).abs() // `abs` is an f64 method that gives the absolute value
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1-x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) { // &mut self desuggars to `self: &mut Self`
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self; // destructure self
        println!("Destroying pair({}, {})", first, second);
    }
}


