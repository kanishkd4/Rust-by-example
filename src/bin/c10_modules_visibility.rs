// By default, all modules are private. Only public modules declared with pub can be accessed from outside the scope

mod my_mod { // private module. Can be called in main() but not outside this code
    fn private_function() { // not available outside the mod. Cannot be called in main() but can be called inside this module
        println!("called `mod::private_function()`"); 
    }

    pub fn function() { // public function - can be called anywhere where my_mod is visible
        println!("called `my_mod::function()`");
    }

    // items can access other items in the same module, even when private
    pub fn indirect_access() { // indirect access can be called in main() and can indirectly call private_function()
        println!("called `my_mod::indirect_access()` that\n>");
        private_function();
    }

    pub mod nested {
        pub fn function() { // called as my_mod::nested::function()
            println!("called `my_mod::nested::function()");
        }
        
        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
        
        // functions declared using `pub(in path)` are only visible within the given path
        // path must be a parent or ancestor module
        pub(in my_mod) fn public_function_in_my_mod() { 
            println!("called `my_mod::nested::public_function_in_my_mod()`, that\n");
            public_function_in_nested();
        }

        // functions declared using `pub(self)` syntax are only visible within the parent module
        pub(self) fn public_function_in_nested() { 
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // functions declared using `pub(super)` are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mode::nested::public_function_in_super_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

// STRUCT VISIBILITY
// struct visibility is private by default and can be overwritten by the pub modifier
// this only matters when a struct is accessible from outside the module where it is defined and has the goal of hiding information
mod my {
    pub struct OpenBox<T> { // a public struct with a public field of generic type
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> { // a public struct with a private field of generic type T
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}


fn main() {
    function();
    my_mod::function();

    // public modules can be accessed from outside the parent module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // The following cannot be called
    // my_mod::nested::public_function_in_my_mod() // pub(in path) can only be called from the mode specified
    // my_mod::private_function() // cannot be called outside the mod at all
    // my_mod::nested::private() // also cannot be called as it is private
    // my_mod::private_nested::function() // cannot be called as the module is private


    // STRUCT VISIBILITY
    // public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };
    println!("The open box contains {}", open_box.contents); // the fields are accesses as usual

    // public structs with private fields cannot be constructed using field names
    // let closed_box = my::ClosedBox { contents: "classified information" }
    // however, structs with private field names can be created using public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // the private fields of a public struct cannot be accessed 
}