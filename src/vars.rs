// vars hold primitive data or refs to data
// vars are immutable (unchangeable) by default
// rust is a block-scoped language
// snake_case is typically used over camelCase for rust :(

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    age = 38;
    //    ^^ if age does not have the `mut` keyword when assigned this will throw an error

    println!("My name is {} and I am {}", name, age);

    // constants (apparently not used often)
    const ID: i32 = 001;
           // ^^^ typing is required for constants
    println!("ID: {}", ID);

    // multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}