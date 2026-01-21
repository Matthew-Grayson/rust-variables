fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // This line produces a compile-time error because `x` is immutable
    println!("The value of x is: {x}");
}