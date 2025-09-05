use std::io;

fn main() {
    println!("This is Slices");
    println!("Enter a string separated by commas to get the first string:");

    let mut words = String::new();

    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line");

    let word = words.trim().split(" ").collect::<Vec<&str>>()[0];

    println!("The first word is: {word}");
}
