fn trim_me(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut j = input.len() - 1;
    while bytes[i] == b' '{
        i+=1;
    }
    while bytes[j] == b' '{
        j-=1;
    }
    input[i..j+1].to_string()
}
 
fn compose_me(input: &str) -> String {
    input.to_string() + " world!"
 
}
 
fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}
 
fn main() {
    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");
 
 
    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");
 
 
    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    
    let array = [1, 2, 3, 4, 5];
    let array_slice : &[i32] = &array[2..4];
    println!("{:?}", array_slice);
}