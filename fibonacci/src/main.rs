use std::io;

fn main() {
    println!("Nth Fibonacci Number Generator");
    println!("Enter the value of n: ");

    loop {
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if n == 0 {
            println!(
                "This program uses the convention that n=1 represents the first Fibonacci number"
            );
        } else {
            println!(
                "The {n}{} Fibonacci number is {}",
                {
                    if n == 1 {
                        "st"
                    } else if n == 2 {
                        "nd"
                    } else if n == 3 {
                        "rd"
                    } else {
                        "th"
                    }
                },
                {
                    let mut fibonacci = vec![0, 1];

                    for i in 2..n {
                        fibonacci.push(fibonacci[(i - 1) as usize] + fibonacci[(i - 2) as usize]);
                    }

                    fibonacci[(n - 1) as usize]
                }
            );
            break;
        }
    }
}
