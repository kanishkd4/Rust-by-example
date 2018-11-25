// DRY - don't repeat yourself.
// Macros allow writing DRY code by factoring out the common parts of functions and/or test suites
// Here's an example that tests the +=, *= and -= operators on Vec<T>:

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // tt or token tree designator is used for operators and tokens
    ($a:ident, $b:ident, $func:ident, $op:tt) => (
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    );
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => (
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y); // *x = x.$method(*y)
            }
        }
    );
}

// Implement add_assign, mul_assign, and sub_assign functions
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);  

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        }
    }
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

// DSL: Domain specific languages
// A DSL is a mini language embedded in a rust macro. It is valid rust because the macro system
// expands to normal rust constructs, but it looks like a small language. 
// This allows us to define concise or intuitive syntax for some special functions(within bounds)

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }        
    }};
}

// VARIADICS
// A variadic interface takes an arbitrary number of arguments, e.g. the println! takes an arbitrary
// number of arguments, as determined by the format string. 
macro_rules! calculate_var {
    (eval $e:expr) => {{ // pattern for a single eval
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
    (eval $e:expr, $(eval $es:expr),+) => {{ // decompose multiple val's recursively
        calculate_var! { eval $e }
        calculate_var! { $(eval $es),+ }
    }}
}

fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    calculate_var! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
