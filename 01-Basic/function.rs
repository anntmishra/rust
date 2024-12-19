//function to add two numbers

fn add(a: i128, b: i128) -> i128 {
    a + b
}

fn main() {
    let x = add(9999999999999, 999999999999999999999);
    
    println!("The result is: {}", x);
}