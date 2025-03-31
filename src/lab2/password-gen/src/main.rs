use rand::Rng;
use std::io;

fn main() {
    let lowercase: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                     'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
                     'u', 'v', 'w', 'x', 'y', 'z'];
    let uppercase: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
                     'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                     'U', 'V', 'W', 'X', 'Y', 'Z'];
    let numbers: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let special: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '-', '_',
                   '+', '=', ':', ';', '?', '/', '~'];

    let mut selected_sets = Vec::new();

    let password_length = get_password_length();
    
    if get_user_choice("Include lowercase letters? (y/n): ") {
        selected_sets.push(&lowercase);
    }
    if get_user_choice("Include uppercase letters? (y/n): ") {
        selected_sets.push(&uppercase);
    }
    if get_user_choice("Include numbers? (y/n): ") {
        selected_sets.push(&numbers);
    }
    if get_user_choice("Include special characters? (y/n): ") {
        selected_sets.push(&special);
    }

    if selected_sets.is_empty() {
        println!("Error: You must select at least one character type!");
        return;
    }

    let password = generate_password(password_length, &selected_sets[..]);
    println!("Generated Password: {}", password);
}

fn get_password_length() -> usize {
    loop {
        let mut input = String::new();
        println!("Enter the password length: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Invalid input. Please enter a positive number."),
        }
    }
}

fn get_user_choice(prompt: &str) -> bool {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

fn generate_password(length: usize, charset: &[&&[char]]) -> String {
    let mut rng = rand::rng();
    let mut password = String::new();

    for _ in 0..length {
        let random_set = charset[rng.random_range(0..charset.len())];
        let random_char = random_set[rng.random_range(0..random_set.len())];
        password.push(random_char);
    }
    
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let lowercase = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                         'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
                         'u', 'v', 'w', 'x', 'y', 'z'];
        let charset = [&lowercase];
        let password = generate_password(12, &charset);
        assert_eq!(password.len(), 12);
    }

    #[test]
    fn test_password_contains_valid_chars() {
        let lowercase = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                         'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
                         'u', 'v', 'w', 'x', 'y', 'z'];
        let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let charset = [&lowercase, &numbers];
        let password = generate_password(20, &charset);
        
        for c in password.chars() {
            assert!(lowercase.contains(&c) || numbers.contains(&c), "Invalid character found in password");
        }
    }
}