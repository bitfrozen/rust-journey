use std::io::stdin;

fn main() {
  println!("Which order of Fibonacci:");
  let mut user_input = String::new();
  if let Err(e) = stdin().read_line(&mut user_input) {
    println!("Error while reading input: {e}")
  }

  let user_input: u8 = match user_input.trim().parse() {
    Ok(result) => result,
    Err(e) => {
      println!("Error while parsing user input: {}", e);
      0
    }
  };

  println!("You've entered {user_input}");
  let result = get_fibonacci(user_input);
  if result > 0 {
    println!("Fibonacci number for sequence {} is {}", user_input, result);
  }
}

fn get_fibonacci(sequence: u8) -> i32 {
  match sequence {
    0 => 0,
    1 => 1,
    n if n > 40 => {
      println!("Sequence too large!");
      -1
    }
    _ => get_fibonacci(sequence - 1) + get_fibonacci(sequence - 2),
  }
}
