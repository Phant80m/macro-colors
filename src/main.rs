use macro_colors::*;
fn main() {
    println!("Hello, world!");
    // Macro to print with green background color and bold style
    let x = 3;
    bold_green_println!("hello world {}", x);
}
