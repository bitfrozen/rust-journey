use std::io::stdin;

fn main() {
  loop {
    println!("Choose what you want to convert:\n\
        \tTo convert to Celsius, press C\n\
        \tTo convert to Fahrenheit, press F\n\
        \tTo quit, press Q");
    let mut selection = String::new();
    stdin().read_line(&mut selection).expect("Failed to read line");
    let selection = selection.trim().to_uppercase();
    match selection.as_str() {
      "C" => {
        let user_temp = get_float_from_user();
        celsius_converter(user_temp);
        break;
      }
      "F" => {
        let user_temp = get_float_from_user();
        fahrenheit_converter(user_temp);
        break;
      }
      "Q" => {
        println!("Quitting the program.");
        break;
      }
      _ => {
        println!("Invalid selection. Please try again.");
      }
    }
  }
}

fn get_float_from_user() -> f32 {
  println!("Enter temperature you want to convert:");
  let mut temperature = String::new();
  stdin().read_line(&mut temperature).expect("Failed to read temperature");
  let temperature: f32 = match temperature.trim().parse() {
    Ok(t) => t,
    Err(_) => {
      println!("Failed to parse temperature");
      return 0.0;
    }
  };

  return temperature;
}

fn fahrenheit_converter(temp: f32) {
  let result = temp * 9.0 / 5.0 + 32.0;
  println!("Celsius {temp} is {result:.1} in Fahrenheit");
}

fn celsius_converter(temp: f32) {
  let result = (temp - 32.0) * 5.0 / 9.0;
  println!("Fahrenheit {temp} is {result:.1} in Celsius");
}
