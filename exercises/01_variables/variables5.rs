fn main() {
    let mut number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    number = "3";
    // Parse the string to integer and add 2
    let result = number.parse::<i32>().unwrap() + 2;
    println!("Number plus two is: {}", result);
}