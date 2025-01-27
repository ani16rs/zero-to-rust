fn main() {

    // MUTABILITY
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of 3H in seconds is: {THREE_HOURS_IN_SECONDS}");

    const MAX_X :u32 = 100_000;
    println!("The value of MAX_X is: {MAX_X}");

    // SHADOWING
    let z = 5;
    let z = 10;
    let z = z + 1;
    println!("The value of z is: {z}");

    {
        let z = z * 2;
        println!("The value of z is: {z}");
    }

    println!("The value of z is: {z}");

    let spaces = "     ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // error: can't change mut variable data type
    // let mut s = "     ";
    // s = s.len();
}
