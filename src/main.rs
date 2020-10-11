use std::io;
use regex::Regex;
use std::process::exit;

// conversion factors, see convert function
const CTF: f64 = 9.0 / 5.0;
const FTC: f64 = 5.0 / 9.0;

fn main() {
    println!("Temperature Converter (C/F)");
    println!("Please enter a temperature to convert:");

    let re = Regex::new(r"\d+(\.\d+)??[cCfF]").unwrap(); // ex. 212F, 0.00C, 123456.789f

    loop {
        // collect user input
        let mut input = String::new();

        loop {
		    io::stdin()
			    .read_line(&mut input)
			    .expect("Failed to read line.");
	
		    if re.is_match(&input) {
			    break
            } else if &input == "quit\n" {
                exit(1);
            } else {
			    println!("Please enter a valid temperature.");
		    }
        }
    
        // clean input, extract information
        let mut clean_input = input.trim().to_lowercase();
    
        let units = clean_input.pop().unwrap();
        let val = clean_input.parse::<f64>().unwrap();

        // convert temperature
        let result = convert(val, units);

        // print result
        println!("{:.2}{}\n", result.0, result.1)
    }
}


fn convert(val: f64, units: char) -> (f64, char) {
    if units == 'f' {
        return (FTC * (val - 32.0), 'C')
    }
    (CTF * val + 32.0, 'F')
}