// A trait is a collection of methods defined for an unkown type: Self. They can access other methods declared in the same trait
// Traits can be implemented for any data type

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // static method signature: `Self` refers to the implementor type
    fn new(name: &'static str) -> Self;
    
    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use implementor's trait methods
            println!("{} is already naked", self.name());
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

// Implement the animal trait for Sheep
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }
    // Default trait methods can be overwritten
    fn talk(&self) {
        // for example, we can add some quiet comtemplation
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly"); // type annotation is necessary in this case
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
