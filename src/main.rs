use std::io::stdin;

fn main() {
    println!("Enter the first number:");
    let mut first_number = String::new();
    stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: u32 = first_number.trim().parse().expect("Please type a number!");

    println!("Enter the operator:");
    let mut operator = String::new();
    stdin().read_line(&mut operator).expect("Failed to read line");
    let operator: char = operator.trim().parse().expect("Please type a operator!");

    println!("Enter the second number:");
    let mut second_number = String::new();
    stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: u32 = second_number.trim().parse().expect("Please type a number!");

    let result = match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Please type a valid operator!")
    };

    println!("{} {} {} = {}", first_number, operator, second_number, result);
}
