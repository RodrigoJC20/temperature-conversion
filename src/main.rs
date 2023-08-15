use std::io;

fn main() {
    println!("Choose a temperature unit:\n");
    println!("1. Farenheit");
    println!("2. Celsius");
    println!("3. Kelvin\n");

    let mut origin_unit = String::new();

    io::stdin()
        .read_line(&mut origin_unit)
        .expect("Failed to read unit type");

    let origin_unit: u32 = match origin_unit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse unit to a number")
    };

    if origin_unit < 1 || origin_unit > 3 { panic!("Please, enter a valid temperature unit"); }

    println!("Write the temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse temperature to a number")
    };

    println!("Choose the target unit\n");
    println!("1. Farenheit");
    println!("2. Celsius");
    println!("3. Kelvin");
    
    let mut target_unit = String::new();

    io::stdin()
        .read_line(&mut target_unit)
        .expect("Failed to read target unit");

    let target_unit: u32 = match target_unit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse the targe unit to a number")
    };

    if target_unit < 1 || target_unit > 3 { panic!("Please, enter a valid temperature unit"); }

    let result: f64 = match target_unit {
        1 => convert_to_farenheit(origin_unit, temperature),
        2 => convert_to_celsius(origin_unit, temperature),
        3 => convert_to_kelvin(origin_unit, temperature),
        _ => panic!("Please, select a valid temperature unit")
    };

    println!("The result of the conversion is: {}", result);
}

fn convert_to_celsius(unit: u32, temperature: f64) -> f64 {
    let degrees: f64 = match unit {
        1 => (temperature - 32.0) * 5.0/9.0,
        2 => temperature,
        3 => temperature - 273.15,
        _ => panic!("Origin unit is incorrect")
    };

    degrees
}

fn convert_to_farenheit(unit: u32, temperature: f64) -> f64 {
    let degrees: f64 = match unit {
        1 => temperature,
        2 => temperature * 9.0/5.0 + 32.0,
        3 => temperature * 9.0/5.0 - 459.67,
        _ => panic!("Origin unit is incorrect")
    };

    degrees
}

fn convert_to_kelvin(unit: u32, temperature: f64) -> f64 {
    let degrees: f64 = match unit {
        1 => (temperature + 459.67) * 5.0/9.0,
        2 => temperature + 273.25,
        3 => temperature,
        _ => panic!("Origin unit is incorrect")
    };

    degrees
}
