use std::io;

/**
 * Main function for program entry. Cycles through the three
 * chapter summary questions, temperature converter, Fibonacci
 * sequencer, and lyrics for 12 days of Christmas.
 */
fn main() {
    println!("Launching Temperature Converter...");
    // Convert temperatures between Fahrenheit and Celsius.
    temp_converter();

    println!("Launching Fibonacci Number Generator...");
    // Generate the nth Fibonacci number
    fibonacci();

    println!("Launching 12 Days of Christmas...");
    twelve_days_of_xmas();

    println!("Exiting...");
}

/**
 * Functions for temperature conversion. Prompts for degree and unit
 * and makes conversion decision based on inputs.
 */
fn c_to_f(degree: f32) {
    let temp_converted = (degree * 1.8) + 32.0;
    println!("{:.1}C is {:.1}F", degree, temp_converted);
}

fn f_to_c(degree: f32) {
    let temp_converted = (degree - 32.0) * 5.0 / 9.0;
    println!("{:.1}F is {:.1}C", degree, temp_converted);
}

fn temp_converter() {
    println!("Enter a temperature followed by an F or C (e.g. 32F), or X to quit.");

    loop {
        let mut temperature = String::new();
        let to_unit;

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature = temperature.trim();
        let len: usize = temperature.len();

        let unit = match &temperature[len - 1..] {
            "c" | "C" => {
                to_unit = "F".to_owned();
                "C"
            }
            "f" | "F" => {
                to_unit = "C".to_owned();
                "F"
            }
            "x" | "X" => {
                break;
            }
            _ => {
                println!("Invalid unit for conversion");
                continue;
            }
        };

        let degree: f32 = match temperature[..len - 1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid degree(s) for conversion");
                continue;
            }
        };

        println!("Converting {degree}{unit} to {to_unit}");
        if to_unit == "F" {
            c_to_f(degree);
        } else {
            f_to_c(degree);
        }

        println!("Enter a new temperature or X to quit");
    }
}

/**
 * Fibonacci number generator. Accepts and integer and returns the
 * Fib number up to that count.
 */
fn fib_recursive(num: u16) -> u16 {
    if num <= 1 {
        return num;
    } else {
        return fib_recursive(num - 1) + fib_recursive(num - 2);
    }
}

fn fibonacci() {
    println!("Enter an integer to calculate a new Fibonacci Number, or X to quit.");

    loop {
        // program loop to run until exit
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input == "x" || input == "X" {
            break;
        }

        let count: u16 = match input.parse() {
            Ok(num) => num, // only match positive integers
            Err(_) => {
                println!("Input must be a positive integer");
                continue;
            }
        };

        let mut sum: u16 = 0;

        // loop for FibNum
        for number in 1..count + 1 {
            let seq_sum: u16 = fib_recursive(number);
            print!("{seq_sum} ");
            sum += seq_sum;
        }

        println!("\nThe Fibonacci sum for F({count}) is {sum}");
        println!("Enter a new integer or X to quit");
    }
}

/**
 * Iterates through the 12 days of Christmas and prints out the lyrics
 */
fn twelve_days_of_xmas() {
    let special_num_suffix = ["st", "nd", "rd"];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..gifts.len() {
        let mut suffix = "th";
        if day < 3 {
            suffix = special_num_suffix[day];
        }
        println!(
            "On the {}{} of Christmas, my true love gave to me...",
            day + 1,
            suffix
        );
        for count in (0..day + 1).rev() {
            print!("\t");
            if count == 0 && day != 0 {
                print!("and ");
            }
            println!("{}", gifts[count]);
        }
    }
}
