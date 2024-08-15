#![allow(unused)]
use std::{io::{self}, process::exit};
fn main() {
    println!("Temperature converter");
    loop{
        println!("1. F to C");
        println!("2. F to K");
        println!("3. C to F");
        println!("4. C to K");
        println!("5. K to C");
        println!("6. K to F");
        println!("7. Exit");
        let option_prompt = std::format!("Use number key plus enter to select");
        let input = get_input(option_prompt).unwrap();
        let option = str::trim(input.as_str());
        if option == "7"{
            println!("Exiting...");
            exit(0);
        }

        let original = ((option.parse::<i32>().unwrap()-1)/2);
        let mut from_final_value = 0.0;
        let mut from_type = "F";
        if original == 1 {
            from_type = "C";
        }
        else if original == 2 {
            from_type = "K";
        }

        let from_prompt = std::format!("Enter original ({}) here:", from_type);
        loop {
            let from_value = get_input(from_prompt.clone()).expect("null");
            let from_value = str::trim(&from_value);
            let parsed = from_value.parse::<f64>();
            if parsed.is_ok() {
                from_final_value = parsed.expect("");
                break;
            }
        }

        println!("Calculating...");
        println!("The result is: ");
        match option{
            "1" => println!("{:.2} in C", (from_final_value-32.0)*5.0/9.0),
            "2" => println!("{:.2} in K", (from_final_value-32.0)*5.0/9.0 + 273.15),
            "3" => println!("{:.2} in F", (from_final_value*(9.0/5.0))+32.0),
            "4" => println!("{:.2} in K", from_final_value+273.15),
            "5" => println!("{:.2} in C", from_final_value-273.15),
            "6" => println!("{:.2} in F", (from_final_value-273.15)*9.0/5.0+32.0),
            _ => println!("User selected an invalid option"),
        }

        let _ = get_input("Press enter to go again".to_string());

    }
}

fn get_input(info_text: String) -> std::io::Result<String>{
    println!("{}", info_text);
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}
