mod print;
mod vars;
mod types;

fn main() {
    println!("\n\n### PRINT OUTPUT ###");
    print::run();
    println!("\n\n### VARS OUTPUT ###");
    vars::run();
    println!("\n\n### TYPES OUTPUT ###");
    types::run();
}
