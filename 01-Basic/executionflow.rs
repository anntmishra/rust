/* fn main(){
let a =99;
if a>99{
    println!("big number");

}else if {
    println!("same number")
}else{
    println!("small number")
}
}
 */

/* fn mainone() {
    let a = 99;
    if a > 99 {
        println!("big number");
    } else if a == 99 {
        println!("same number");
    } else {
        println!("small number");
    }
} */


//nested if else

fn categorize_number(num: i32) -> &'static str {
    if num > 0 {
        if num % 2 == 0 {
            "Positive even number"
        } else {
            "Positive odd number"
        }
    } else if num < 0 {
        if num % 2 == 0 {
            "Negative even number"
        } else {
            "Negative odd number"
        }
    } else {
        "Zero"
    }
}

fn main() {
    let number = 5;

    // Call the function and print the result
    println!("The number {} is a {}.", number, categorize_number(number));
}