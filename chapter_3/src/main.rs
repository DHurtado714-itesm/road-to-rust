fn main() {
    variables();
    data_types()
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
}