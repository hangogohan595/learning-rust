use std::io;

fn main() {
    'main: loop {
        println!("Pick a temperature to convert:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if choice.trim() == "quit" {
            break 'main;
        }

        let choice: i32 = match choice.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if choice == 1 {
            println!("Enter a temperature in Celsius:");

            loop {
                let mut temp = String::new();

                io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to read line");

                if temp.trim() == "quit" {
                    break 'main;
                }

                let temp: f32 = match temp.trim().parse() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                println!(
                    "{temp} degrees Celsius is equivalent to {} degrees Fahrenheit",
                    ((temp * (9.0 / 5.0)) + 32.0)
                );
                break 'main;
            }
        } else if choice == 2 {
            println!("Enter a temperature in Fahrenheit:");

            loop {
                let mut temp = String::new();

                io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to read line");

                if temp.trim() == "quit" {
                    break 'main;
                }

                let temp: f32 = match temp.trim().parse() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                println!(
                    "{temp} degrees Fahrenheit is equivalent to {} degrees Celsius",
                    ((temp - 32.0) * (5.0 / 9.0))
                );
                break 'main;
            }
        } else {
            continue;
        }
    }
}
