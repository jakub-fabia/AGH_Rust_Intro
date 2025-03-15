fn main() {
    println!("{}", increment(10));
    let primes: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
 
    println!("Primes numbers:");
     
    for n in 0..primes.len() {
        println!("{}", n);
    }
}
 
fn increment(n : i32) -> i32 {
    n + 1
}