#![allow(dead_code)]

// ERROR HANDLING
// panic is used for tests and dealing with unrecoverable errors. 
// It prints an error message, starts unwinding the task and usually exists the program. 

fn give_princes(gift: &str) {
    if gift == "snake" { panic!("AAAaaaa"); }
    println!("I love {}", gift);
}

fn main() {
    give_princes("teddy bear");
    // give_princes("snake"); // will panic here

    // OPTION AND UNWRAP
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;
    give_commoner(food);
    give_commoner(snake);
    give_commoner(void); // continues the program because of explicit handling
    let food = Some("cabbage");
    give_princess1(food);
    // give_princess1(void); // will panic here

    // COMBINATORS: map
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    // COMBINATORS: and_then
    let (cordon_bleu, steak, sushi) = (Food1::CordonBleu, Food1::Steak, Food1::Sushi);
    eat1(cordon_bleu, Day::Monday);
    eat1(steak, Day::Tuesday);
    eat1(sushi, Day::Wednesday);
}

// OPTION AND UNWRAP
// Option is an enum used when absence is a possibility: It has 2 elements: Some(T) and None
// The cases can be handled via a match or implicitly with unwrap. Implicit handling will either return
// the inner element or panic.
fn give_commoner(gift: Option<&str>) { // handling errors using a match
    match gift {
        Some("snake") => println!("Found a snake"), 
        Some(inner) => println!("{}?", inner),
        None => println!("no gift"),
    }
}

fn give_princess1(gift: Option<&str>) {
    let inside = gift.unwrap(); // unwrap returns a panic when it receives a None.
    if inside == "snake" { panic!("AAAAAAaa"); }
    println!("I love {}", inside);
}

// COMBINATORS: map
// Heavy usage of match may be tedius. Combinators can be used to manage control flow in a modular fashion. 
// Option has a built in method called map(), a combinator for mapping Some -> Some and None -> None.
// Multiple map calls can be chained together for more flexibility
#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(Food)) => Some(Chopped(Food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> { // cook food; using map instead of match for case handling
    chopped.map(|Chopped(Food)| Cooked(Food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("I love {:?}", food),
        None => println!("Not edible"),
    }
}

// COMBINATORS: and_then
// Using map() on a function that returns an Option<T> results in netsted Option<Option<T>>
// and_then works as a flatmap and calls its function input with the wrapped value and returns the result.
// If the Option is None, it returns None instead.
#[derive(Debug)] enum Food1 { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food1) -> Option<Food1> {
    match food {
        Food1::Sushi => None, // we don't have the ingredients to make sushi
        _ => Some(food),
    }
}

fn have_recipe(food: Food1) -> Option<Food1> {
    match food {
        Food1::CordonBleu => None, // we don't have a recipe for Cordon Bleu
        _ => Some(food),
    }
}

fn cookable_v1(food: Food1) -> Option<Food1> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food1) -> Option<Food1> { // v1 can be conveniently rewritten more compactly
    have_ingredients(food).and_then(have_recipe)
}

fn eat1(food: Food1, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("On {:?} we get to eat {:?}", day, food),
        None => println!("We don't get to eat on {:?}", day),
    }
}