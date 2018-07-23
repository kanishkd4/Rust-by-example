// By default, all modules are private. Only public modules declared with pub can be accessed from outside the scope

mod my_mod {
    fn private_function() {
        println!("called `mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // items can access other items in the same module, even when private
    pub fn indirect_access() {
        println!("called `my_mod::indirect_access()` that\n>");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()1");
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

fn main() {
    function();
    my_mod::function();
}