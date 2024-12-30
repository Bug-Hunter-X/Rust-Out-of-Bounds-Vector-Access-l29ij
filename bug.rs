fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let num = numbers.get(10).unwrap(); //Error: index out of bounds
    println!("The number at index 10 is: {}", num);
}