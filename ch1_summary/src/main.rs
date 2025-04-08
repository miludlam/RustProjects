use std::io;

fn main() {
    println!("Hello, world!");

    // Convert temperatures between Fahrenheit and Celsius.
    temp_converter();
}

// Converts from Celcius to Fahrenheit
fn c_to_f(degree: f32) {
    let temp_converted = (degree * 1.8) + 32.0;
    println!("{:.1}C is {:.1}F", degree, temp_converted);
}

// Converts from Fahrenheit to Celcius
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
                println!("Exiting...");
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
