pub fn run() {
    println!("Hello from the print.rs file");

    //formatting
    println!("Number: {}", 1);
                   // ^^ placeholder

    // positional args
    println!("{} is from {} and likes to {}", "Brad", "Mass", "code");
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // named args
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    
    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}