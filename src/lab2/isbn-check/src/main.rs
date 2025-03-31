fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect()
}

fn main() {
    println!("Enter ISBN-10 number:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let numbers = parse_input(&input);
    if numbers.len() != 10 {
        println!("Invalid ISBN-10 number. It should contain exactly 10 digits.");
        return;
    }
    let mut sum = 0;

    for i in 0..10 {
        sum += numbers[i] * (10 - i as i32);
    }
    if sum % 11 == 0 {
        println!("Valid ISBN-10 number.");
    } else {
        println!("Invalid ISBN-10 number.");
    }
    return
}