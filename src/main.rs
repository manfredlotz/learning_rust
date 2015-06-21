fn main() {
    println_integer();
    println_float();
    
}

fn println_integer() {
    println!("Showing how to format integers");
    
    let i = 3;

    println!("<{}>", i);
    // field with 4
    println!("<{:4}>", i);
    // field with 4, left justified
    println!("<{:<4}>", i);
    // field with 4, filled with zero
    println!("<{:04}>", i);
}

fn println_float() {
    println!("Showing how to format floats");

    let pi = 3.14159;

    println!("<{}>", pi);
    // 2 digits after period
    println!("<{:.2}>", pi);
    // 5 digits before and 2 digits after period
    println!("<{:7.2}>", pi);

}
