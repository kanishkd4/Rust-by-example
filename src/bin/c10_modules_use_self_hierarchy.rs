// The use declaration can be used to bind a full path to a new name
// an example - `import torch.data.utils.DataLoader as data_loader` becomes `use torch::data::utils::DataLoader as data_loader;`

// use bindings have a local scope. if a use is declared in a scope, the binding is not valid outside that scope

// SUPER AND SELF
// these can be used in the path to remove ambiguity when accesing items and to prevent unnecesary hardcoding of paths

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() { // we'll access all the functions named functions() from this scope
        print!("called `my::indirect_call()` that\n ");

        self::function(); // the self keywork refers to the current module scope; in this case my
        function(); // self::function and function() give the same result.

        // we can use self to access another module inside my
        self::cool::function();

        // the super keyword refers to the parent scope (outside the my module)
        super::function();

        { // this will bind the cool::function() in the crate scope - the outermost scope in this case
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}